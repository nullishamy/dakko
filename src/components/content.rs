use html2text::config;

#[derive(Debug)]
pub struct ContentComponent {
    content: String,
}

impl ContentComponent {
    pub fn new(html: String) -> Self {
        Self { content: html }
    }

    pub fn render(&mut self, ui: &mut egui::Ui, width: usize) {
        ui.label(&self.content);
    }
}
