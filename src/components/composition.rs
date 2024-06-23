#[derive(Debug)]
pub struct CompositionArea {
	text: String
}

impl CompositionArea {
	pub fn new() -> Self {
		Self { text: String::new() }
	}

	pub fn render(&mut self, ui: &mut egui::Ui) {
		ui.heading("Composition area");
		ui.text_edit_multiline(&mut self.text);
	}
}
