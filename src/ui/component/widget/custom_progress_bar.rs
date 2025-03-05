use crate::resources::resource::Resource;
use egui::{Color32, ProgressBar, Response, Ui, Widget};
use crate::resources::bignumber::BigNumber;

pub struct CustomProgressBar {
    resource: Resource,
}

impl CustomProgressBar {
    pub fn new(resource: Resource) -> Self {
        Self { resource }
    }
}

impl Widget for CustomProgressBar {
    fn ui(self, ui: &mut Ui) -> Response {
        let progress = BigNumber::to_f64(&self.resource.amount) / 1.0;
        let pb = ProgressBar::new(progress as f32).animate(true).fill(Color32::RED).show_percentage();

        ProgressBar::ui(pb, ui)
    }
}