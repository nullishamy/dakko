use megalodon::entities;

use super::content::ContentComponent;

struct StatusControls {}
impl StatusControls {
    fn render(&self, ui: &mut egui::Ui) {
        let mut frame = egui::Frame::default()
            .outer_margin(egui::Margin {
                top: 5.0,
                ..Default::default()
            })
            .begin(ui);
        {
            frame.content_ui.horizontal(|ui| {
                ui.button("reply");
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

        ui.horizontal_centered(|ui| {
            if attachments_sensitive && !self.sensitive_open {
                let show_images = ui.button("images marked as sensitive, click to show");
                if show_images.clicked() {
                    self.sensitive_open = true;
                }
                return;
            }

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
            ui.label(egui::RichText::new(spoiler_text.clone()).italics());
            ui.separator();
            if self.show_spoilers && ui.button("hide content").clicked() {
                self.show_spoilers = false;
            } else if !self.show_spoilers && ui.button("show content").clicked() {
                self.show_spoilers = true;
            }
        }

        if (has_spoilers && self.show_spoilers) || !has_spoilers {
            self.content.render(ui, ui.available_width() as usize);
            self.attachments.render(ui);
        }
    }
}

#[derive(Debug)]
pub struct StatusComponent {
    status: entities::Status,
    content: StatusContent,
    sensitive_open: bool,
}
impl StatusComponent {
    pub fn new(status: entities::Status) -> StatusComponent {
        let reblog = status.reblog.as_ref();
        let status_or_reblog = reblog.map(|s| *s.clone()).unwrap_or(status.clone());
        let html = status_or_reblog
            .plain_content
            .as_ref()
            .unwrap_or(&"".to_string())
            .clone();

        Self {
            status,
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
        }
    }

    pub fn data(&self) -> &entities::Status {
        &self.status
    }

    pub fn render(&mut self, ui: &mut egui::Ui) {
        let status = &self.status;
        let reblog = status.reblog.as_ref();
        let status_or_reblog = reblog.map(|s| *s.clone()).unwrap_or(status.clone());

        if let Some(reblog) = reblog {
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

                    let controls = StatusControls {};
                    controls.render(ui);
                })
            });
        }
        frame.end(ui);
    }
}
