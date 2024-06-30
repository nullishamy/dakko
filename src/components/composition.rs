use core::fmt;

use megalodon::entities;

#[derive(Debug)]
pub struct PostableContent {
    pub content: String,
    pub reply_id: Option<String>,
    pub visibility: entities::StatusVisibility,
}

pub struct CompositionArea {
    text: String,
    reply_id: Option<String>,
    on_post: Box<dyn FnMut(PostableContent) + 'static>,
    visibility: entities::StatusVisibility,
}

impl fmt::Debug for CompositionArea {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("CompositionArea")
            .field("text", &self.text)
            .finish()
    }
}

impl CompositionArea {
    pub fn new(on_post: impl FnMut(PostableContent) + 'static, reply_id: Option<String>) -> Self {
        Self {
            text: String::new(),
            on_post: Box::new(on_post),
            reply_id,
            visibility: entities::StatusVisibility::Public,
        }
    }

    pub fn render(&mut self, ui: &mut egui::Ui) {
        ui.heading("Composition area");

        let text = egui::TextEdit::multiline(&mut self.text);
        ui.add_sized((ui.available_width(), 25.0), text);

        ui.add_space(2.0);

        egui::ComboBox::from_id_source(format!(
            "visibility-reply-{}",
            self.reply_id.clone().unwrap_or("compose".to_string())
        ))
        .selected_text(format!("{:?}", &mut self.visibility))
        .show_ui(ui, |ui| {
            ui.selectable_value(
                &mut self.visibility,
                entities::StatusVisibility::Public,
                "Public",
            );

            ui.selectable_value(
                &mut self.visibility,
                entities::StatusVisibility::Unlisted,
                "Unlisted",
            );

            ui.selectable_value(
                &mut self.visibility,
                entities::StatusVisibility::Private,
                "Private",
            );

            ui.selectable_value(
                &mut self.visibility,
                entities::StatusVisibility::Direct,
                "Direct",
            );
        });

        if ui.button("post").clicked() {
            (self.on_post)(PostableContent {
                content: self.text.clone(),
                reply_id: self.reply_id.clone(),
                visibility: self.visibility.clone()
            })
        }
    }
}
