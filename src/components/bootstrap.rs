use crate::{app, http};

pub struct BootstrapComponent;
impl BootstrapComponent {
    pub fn render(app: &mut app::MainApp, ui: &mut egui::Ui) {
        ui.label("let's get you logged in");
        ui.horizontal(|ui| {
            ui.label("instance url:");
            ui.add(
                egui::text_edit::TextEdit::singleline(&mut app.bootstrap.base_url)
                    .hint_text("https://labyrinth.zone"),
            );
        });

        ui.horizontal(|ui| {
            ui.label("instance type:");

            egui::ComboBox::from_id_source("instance_type")
                .selected_text(format!("{:?}", &mut app.bootstrap.instance_type))
                .show_ui(ui, |ui| {
                    ui.selectable_value(
                        &mut app.bootstrap.instance_type,
                        megalodon::SNS::Mastodon,
                        "Mastodon",
                    );
                    ui.selectable_value(
                        &mut app.bootstrap.instance_type,
                        megalodon::SNS::Pleroma,
                        "Pleroma",
                    );
                    ui.selectable_value(
                        &mut app.bootstrap.instance_type,
                        megalodon::SNS::Firefish,
                        "Firefish",
                    );
                    ui.selectable_value(
                        &mut app.bootstrap.instance_type,
                        megalodon::SNS::Gotosocial,
                        "GotoSocial",
                    );
                    ui.selectable_value(
                        &mut app.bootstrap.instance_type,
                        megalodon::SNS::Friendica,
                        "Friendica",
                    );
                });
        });

        if ui.button("continue").clicked() {
            app.send_request(http::HttpRequest::Configure(app.bootstrap.clone()));
            app.send_request(http::HttpRequest::RequestAuthURL(
                app.bootstrap.callback_addr,
            ));
        }
    }
}
