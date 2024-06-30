use std::{
    collections::HashMap,
    net::{IpAddr, Ipv4Addr},
    sync::mpsc,
};

use egui::{panel::Side, Id};
use megalodon::{entities, oauth::TokenData};
use serde::{Deserialize, Serialize};
use std::net;

use crate::{
    components::{bootstrap::BootstrapComponent, status::StatusComponent},
    http::{HttpRequest, HttpResponse},
};

#[derive(Deserialize, Serialize)]
#[serde(default)] // if we add new fields, give them default values when deserializing old state
pub struct MainApp {
    pub client_state: Option<ClientState>,
    pub auth_state: Option<AuthState>,

    #[serde(skip)]
    pub bootstrap: BootstrapState,
    #[serde(skip)]
    pub http: Option<(mpsc::Sender<HttpRequest>, mpsc::Receiver<HttpResponse>)>,
    #[serde(skip)]
    pub active: Option<ActiveState>,
}

#[derive(Debug, Hash, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
pub enum Timeline {
    Home,
}

#[derive(Debug)]
pub struct TimelineState {
    key: Timeline,
    statuses: Vec<StatusComponent>,
    fetching: bool,
    up_to_date: bool,
}

#[derive(Debug)]
pub struct ActiveState {
    account: entities::Account,
    current_timeline: Timeline,
    timelines: HashMap<Timeline, TimelineState>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BootstrapState {
    pub base_url: String,
    pub instance_type: megalodon::SNS,
    pub auth_url: Option<String>,
    pub callback_addr: net::SocketAddr,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClientState {
    pub client_id: String,
    pub client_secret: String,
    pub base_url: String,
    pub instance_type: megalodon::SNS,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthState {
    pub token: TokenData,
}

impl Default for MainApp {
    fn default() -> Self {
        Self {
            client_state: None,
            auth_state: None,
            bootstrap: BootstrapState {
                base_url: "https://labyrinth.zone".to_string(),
                instance_type: megalodon::SNS::Pleroma,
                auth_url: None,
                callback_addr: net::SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 9119),
            },
            http: None,
            active: None,
        }
    }
}

impl MainApp {
    pub fn new(
        cc: &eframe::CreationContext<'_>,
        http: (mpsc::Sender<HttpRequest>, mpsc::Receiver<HttpResponse>),
    ) -> Self {
        cc.egui_ctx.set_visuals(egui::Visuals::light());
        egui_extras::install_image_loaders(&cc.egui_ctx);

        let mut app_state: MainApp = if let Some(storage) = cc.storage {
            eframe::get_value(storage, eframe::APP_KEY).unwrap_or_default()
        } else {
            Default::default()
        };

        app_state.http = Some(http);

        // Send our context over as early as possible
        app_state.send_request(HttpRequest::Init(cc.egui_ctx.clone()));

        if let Some(client_state) = &app_state.client_state {
            if let Some(auth_state) = &app_state.auth_state {
                app_state.send_request(HttpRequest::Authenticate(
                    client_state.clone(),
                    auth_state.clone(),
                ));
                app_state.send_request(HttpRequest::RequestSelf);
            }
        }

        app_state
    }

    pub fn send_request(&self, request: HttpRequest) {
        let http = self.http.as_ref().unwrap();
        http.0.send(request).unwrap();
    }

    pub fn poll_response(&self) -> Option<HttpResponse> {
        let http = self.http.as_ref().unwrap();
        http.1.try_recv().ok()
    }

    pub fn is_logged_in(&self) -> bool {
        self.client_state.is_some() && self.auth_state.is_some()
    }

    pub fn has_active_state(&self) -> bool {
        self.active.is_some()
    }
}

impl MainApp {
    fn active_state(&self) -> &ActiveState {
        self.active.as_ref().unwrap()
    }

    fn active_state_mut(&mut self) -> &mut ActiveState {
        self.active.as_mut().unwrap()
    }

    fn load_timeline_from(&mut self, timeline: Timeline, from_id: Option<String>) {
        let active = self.active_state_mut();
        if let Some(tl) = active.timelines.get_mut(&timeline) {
            if tl.fetching || tl.up_to_date {
            } else {
                tl.fetching = true;
                self.send_request(HttpRequest::RequestTimeline(timeline, from_id));
            }
        } else {
            active.timelines.insert(
                timeline,
                TimelineState {
                    key: timeline,
                    statuses: vec![],
                    fetching: true,
                    up_to_date: false,
                },
            );
            self.send_request(HttpRequest::RequestTimeline(timeline, from_id));
        }
    }

    fn load_timeline(&mut self, timeline: Timeline) {
        self.load_timeline_from(timeline, None);
    }

    fn handle_http_response(&mut self, res: HttpResponse) {
        match res {
            HttpResponse::RequestAuthURL(app_data) => {
                let url = app_data.url.unwrap();
                self.bootstrap.auth_url = Some(url.clone());
                self.client_state = Some(ClientState {
                    client_id: app_data.client_id,
                    client_secret: app_data.client_secret,
                    base_url: self.bootstrap.base_url.clone(),
                    instance_type: self.bootstrap.instance_type.clone(),
                });
                self.send_request(HttpRequest::LaunchServer(self.bootstrap.callback_addr));
                self.send_request(HttpRequest::OpenURL(url));
            }
            HttpResponse::TokenData(token) => {
                self.auth_state = Some(AuthState { token });
                self.send_request(HttpRequest::RequestSelf);
            }
            HttpResponse::RequestSelf(account) => {
                if let Some(active) = &mut self.active {
                    active.account = account;
                } else {
                    self.active = Some(ActiveState {
                        account,
                        timelines: HashMap::new(),
                        current_timeline: Timeline::Home,
                    });
                }
            }
            HttpResponse::RequestTimeline(timeline, statuses) => {
                let sender = self.http.as_ref().unwrap().0.clone();
                let statuses = statuses
                    .into_iter()
                    .map(|status| {
                        let sender = sender.clone();
                        StatusComponent::new(status, move |reply| {
                            let opts = megalodon::megalodon::PostStatusInputOptions {
                                in_reply_to_id: reply.reply_id,
                                ..Default::default()
                            };
                            sender.send(HttpRequest::PostStatus(opts, reply.content)).unwrap();
                        })
                    })
                    .collect::<Vec<_>>();
                let active = self.active_state_mut();
                if let Some(tl) = active.timelines.get_mut(&timeline) {
                    tl.statuses.extend(statuses);
                    tl.fetching = false;
                    tl.up_to_date = true;
                } else {
                    active.timelines.insert(timeline, {
                        TimelineState {
                            key: timeline,
                            statuses,
                            fetching: false,
                            up_to_date: true,
                        }
                    });
                }
            }
        }
    }

    fn sidebar_panel(&mut self, ui: &mut egui::Ui) {
        if self.has_active_state() {
            let active = self.active_state();
            let account = &active.account;
            ui.horizontal(|ui| {
                ui.add(
                    egui::Image::from_uri(account.avatar.clone())
                        .fit_to_exact_size((75.0, 75.0).into())
                        .rounding(egui::Rounding::same(10.0))
                        .maintain_aspect_ratio(true),
                );
                ui.add_space(5.0);
                ui.vertical(|ui| {
                    ui.label(egui::RichText::new(account.display_name.clone()).size(16.0));
                    ui.add_space(10.0);
                    ui.label(egui::RichText::new(format!("@{}", account.acct)).size(16.0));
                });
            });
        } else {
            ui.label("loading...");
        }
    }

    fn notification_panel(&mut self, ui: &mut egui::Ui) {
        ui.heading("notifications");
    }

    fn main_panel(&mut self, ui: &mut egui::Ui) {
        ui.vertical(|ui| {
            if self.is_logged_in() && self.has_active_state() {
                self.load_timeline(Timeline::Home);
            } else if let Some(auth_url) = &self.bootstrap.auth_url {
                let open_auth_url = ui.hyperlink_to(
                    "click here to login, if your browser does not open",
                    auth_url,
                );

                if open_auth_url.clicked() {
                    self.send_request(HttpRequest::OpenURL(auth_url.clone()));
                };
            } else {
                BootstrapComponent::render(self, ui);
            }
        });

        if !self.has_active_state() {
            return;
        }

        let active = self.active_state();
        let last_post_id = active
            .timelines
            .get(&active.current_timeline)
            .and_then(|t| t.statuses.last().map(|s| s.data().id.clone()));

        let scroll = egui::ScrollArea::vertical()
            .auto_shrink(false)
            .id_source("timeline")
            .show(ui, |ui| {
                let active = self.active_state_mut();
                let timeline = active.timelines.get_mut(&active.current_timeline);
                if let Some(timeline) = timeline {
                    // We have got the posts
                    let len = timeline.statuses.len();
                    for status in timeline.statuses.iter_mut() {
                        status.render(ui);
                        ui.add_space(10.0);
                        ui.separator();
                        ui.add_space(10.0);
                    }

                    len > 0
                } else {
                    // We are probably still fetching it
                    ui.label(format!("still fetching {:?}", active.current_timeline));
                    false
                }
            });

        let available_offset = scroll.content_size - scroll.inner_rect.size();
        if (scroll.state.offset.y >= (available_offset.y - 3000.0)) && scroll.inner {
            let active = self.active_state_mut();
            let timeline = active.timelines.get_mut(&active.current_timeline).unwrap();
            timeline.up_to_date = false;
            let current = active.current_timeline;
            self.load_timeline_from(current, last_post_id);
        }

        ui.separator();

        // ui.add(egui::github_link_file!(
        //     "https://github.com/emilk/eframe_template/blob/main/",
        //     "Source code."
        // ));
    }
}

impl eframe::App for MainApp {
    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        eframe::set_value(storage, eframe::APP_KEY, self);
    }

    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Put your widgets into a `SidePanel`, `TopBottomPanel`, `CentralPanel`, `Window` or `Area`.
        // For inspiration and more examples, go to https://emilk.github.io/egui
        if let Some(res) = self.poll_response() {
            self.handle_http_response(res);
        }

        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            egui::menu::bar(ui, |ui| {
                ui.menu_button("File", |ui| {
                    if ui.button("Quit").clicked() {
                        ctx.send_viewport_cmd(egui::ViewportCommand::Close);
                    }
                });
                ui.add_space(16.0);

                egui::widgets::global_dark_light_mode_buttons(ui);
            });
        });

        egui::SidePanel::new(Side::Left, Id::new("left-pane"))
            .min_width(200.0)
            .max_width(300.0)
            .show(ctx, |ui| {
                self.sidebar_panel(ui);
            });

        egui::SidePanel::new(Side::Right, Id::new("right-pane"))
            .min_width(200.0)
            .max_width(300.0)
            .show(ctx, |ui| {
                self.notification_panel(ui);
            });

        egui::CentralPanel::default().show(ctx, |ui| {
            self.main_panel(ui);
        });
    }
}

fn powered_by_egui_and_eframe(ui: &mut egui::Ui) {
    ui.horizontal(|ui| {
        ui.spacing_mut().item_spacing.x = 0.0;
        ui.label("Powered by ");
        ui.hyperlink_to("egui", "https://github.com/emilk/egui");
        ui.label(" and ");
        ui.hyperlink_to(
            "eframe",
            "https://github.com/emilk/egui/tree/master/crates/eframe",
        );
        ui.label(".");
    });
}
