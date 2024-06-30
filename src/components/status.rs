use core::fmt;

use megalodon::entities;

use super::{
    composition::{CompositionArea, PostableContent},
    content::ContentComponent,
};

#[derive(Debug)]
struct StatusControls {
    reply: CompositionArea,
    reply_open: bool,
    author: String,
    id: String,
}
impl StatusControls {
    fn render(&mut self, ui: &mut egui::Ui) {
        if self.reply_open {
            egui::Window::new(format!("reply to {}", self.author))
                .id(self.id.clone().into())
                .open(&mut self.reply_open)
                .show(ui.ctx(), |ui| {
                    self.reply.render(ui);
                });
        }
        let mut frame = egui::Frame::default()
            .outer_margin(egui::Margin {
                top: 5.0,
                ..Default::default()
            })
            .begin(ui);
        {
            frame.content_ui.horizontal(|ui| {
                if ui.button("reply").clicked() {
                    self.reply_open = true;
                }
                ui.add_space(2.0);
                ui.button("quote");
                ui.add_space(2.0);
                ui.button("boost");
                ui.add_space(2.0);
                ui.button("star!");
                ui.add_space(2.0);
                ui.button("react");
            });
        }
        frame.end(ui);
    }
}

#[derive(Debug)]
struct StatusAttachments {
    status: entities::Status,
    sensitive_open: bool,
}

impl StatusAttachments {
    pub fn render(&mut self, ui: &mut egui::Ui) {
        let status = &self.status;
        let attachments_sensitive = status.sensitive;
        if status.media_attachments.is_empty() {
            return;
        }

        if attachments_sensitive && !self.sensitive_open {
            ui.centered_and_justified(|ui| {
                ui.set_min_height(30.0);
                let show_images = ui.button("images marked as sensitive, click to show");
                if show_images.clicked() {
                    self.sensitive_open = true;
                }
            });
            return;
        }

        ui.horizontal_centered(|ui| {
            for attachment in &status.media_attachments {
                match &attachment.r#type {
                    entities::attachment::AttachmentType::Image => {
                        ui.add(
                            egui::Image::from_uri(attachment.url.clone())
                                .maintain_aspect_ratio(true)
                                .fit_to_exact_size((300.0, 300.0).into()),
                        );
                    }
                    u => {
                        ui.label(format!("unsupported attachment type {:#?}", u));
                    }
                }
            }
        });
    }
}

#[derive(Debug)]
struct StatusContent {
    content: ContentComponent,
    status: entities::Status,
    show_spoilers: bool,
    attachments: StatusAttachments,
}
impl StatusContent {
    fn render(&mut self, ui: &mut egui::Ui) {
        let spoiler_text = self.status.spoiler_text.clone();
        let has_spoilers = !self.status.spoiler_text.is_empty();

        if !spoiler_text.is_empty() {
            ui.vertical_centered_justified(|ui| {
                ui.label(egui::RichText::new(spoiler_text.clone()).italics());
                ui.separator();
                if self.show_spoilers && ui.button("hide content").clicked() {
                    self.show_spoilers = false;
                    self.attachments.sensitive_open = false;
                } else if !self.show_spoilers && ui.button("show content").clicked() {
                    self.show_spoilers = true;
                }
                ui.add_space(2.0);
            });
        }

        if (has_spoilers && self.show_spoilers) || !has_spoilers {
            self.content.render(ui);
            self.attachments.render(ui);
        }
    }
}

pub struct StatusComponent {
    status: entities::Status,
    controls: StatusControls,
    on_reply: Box<dyn FnMut(PostableContent) + 'static>,
    content: StatusContent,
    sensitive_open: bool,
}

impl fmt::Debug for StatusComponent {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("StatusComponent")
            .field("status", &self.status)
            .field("controls", &self.controls)
            .field("content", &self.content)
            .field("sensitive_open", &self.sensitive_open)
            .finish()
    }
}

impl StatusComponent {
    pub fn new(status: entities::Status, on_reply: impl FnMut(PostableContent) + 'static + Clone) -> StatusComponent {
        let reblog = status.reblog.as_ref();
        let status_or_reblog = reblog.map(|s| *s.clone()).unwrap_or(status.clone());
        let html = status_or_reblog
            .plain_content
            .as_ref()
            .unwrap_or(&"".to_string())
            .clone();

        Self {
            status,
            controls: StatusControls {
                reply: CompositionArea::new(
                    on_reply.clone(),
                    Some(status_or_reblog.id.clone()),
                ),
                reply_open: false,
                author: status_or_reblog.account.display_name.clone(),
                id: status_or_reblog.id.clone(),
            },
            content: StatusContent {
                content: ContentComponent::new(html),
                show_spoilers: false,
                status: status_or_reblog.clone(),
                attachments: StatusAttachments {
                    status: status_or_reblog,
                    sensitive_open: false,
                },
            },
            sensitive_open: false,
            on_reply: Box::new(on_reply),
        }
    }

    pub fn data(&self) -> &entities::Status {
        &self.status
    }

    pub fn render(&mut self, ui: &mut egui::Ui) {
        let status = &self.status;
        let reblog = status.reblog.as_ref();
        let status_or_reblog = reblog.map(|s| *s.clone()).unwrap_or(status.clone());

        if reblog.is_some() {
            let mut frame = egui::Frame::default()
                .outer_margin(egui::Margin {
                    left: 25.0,
                    ..Default::default()
                })
                .begin(ui);
            {
                frame.content_ui.horizontal(|ui| {
                    ui.add(
                        egui::Image::from_uri(status.account.avatar.clone())
                            .maintain_aspect_ratio(true)
                            .rounding(egui::Rounding::same(5.0))
                            .fit_to_exact_size((25.0, 25.0).into()),
                    );
                    ui.add_space(5.0);
                    ui.label(format!("{} boosted", status.account.display_name));
                });
            }
            frame.end(ui);
        }

        let mut frame = egui::Frame::default()
            .outer_margin(egui::Margin {
                right: 20.0,
                ..Default::default()
            })
            .begin(ui);
        {
            frame.content_ui.horizontal(|ui| {
                ui.set_width(ui.available_width());
                ui.vertical(|ui| {
                    ui.add(
                        egui::Image::from_uri(status_or_reblog.account.avatar.clone())
                            .maintain_aspect_ratio(true)
                            .rounding(egui::Rounding::same(10.0))
                            .fit_to_exact_size((50.0, 50.0).into()),
                    );
                });
                ui.add_space(5.0);
                ui.vertical(|ui| {
                    ui.horizontal_top(|ui| {
                        ui.label(
                            egui::RichText::new(status_or_reblog.account.display_name.clone())
                                .strong(),
                        );
                        ui.add_space(1.0);
                        ui.label(
                            egui::RichText::new(format!("@{}", status_or_reblog.account.acct))
                                .color(egui::Color32::BLUE),
                        );

                        ui.with_layout(egui::Layout::right_to_left(egui::Align::TOP), |ui| {
                            if ui.button("+").clicked() {
                                dbg!("expand");
                            }
                            ui.label(format!("{:?}", status.visibility));
                            ui.label(status.created_at.to_string());
                        });
                    });

                    self.content.render(ui);
                    self.controls.render(ui);
                })
            });
        }
        frame.end(ui);
    }
}
