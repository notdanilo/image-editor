use egui::{Color32, Sense, Vec2, Widget};
use crate::app::ADDRESS;
use crate::WebImage;

pub struct Gallery {
    image_size: f64,
    images: Vec<WebImage>,
    selection: Option<usize>
}

impl Gallery {
    pub fn new() -> Self {
        let image_size = 32.0;
        let images = vec![
            WebImage::new(format!("http://{}:8000/DSC_0008.JPG", ADDRESS)),
            WebImage::new(format!("http://{}:8000/000051.bb94bc2c.3538291841.png", ADDRESS))
        ];
        let selection = None;
        Self { image_size, images, selection }
    }
    pub fn show(&mut self, ui: &mut egui::Ui) {
        egui::ScrollArea::both().show(ui, |ui| {
            ui.add(egui::Slider::new(&mut self.image_size, 0.0..=512.0));
            let image_size = self.image_size as f32;
            let image_size = Vec2::new(image_size, image_size);
            ui.horizontal(|ui| {
                for (index, image) in self.images.iter_mut().enumerate() {
                    image.retained_image(ui, |ui, image| {
                        if egui::ImageButton::new(image.texture_id(ui.ctx()), image_size).sense(Sense::click_and_drag()).ui(ui).clicked() {
                            self.selection = Some(index);
                        }
                    });
                }
            });
        });
    }

    pub fn selection(&self) -> Option<String> {
        self
            .selection
            .and_then(|selection|
                self
                    .images
                    .get(selection)
                    .map(|image| image.source().clone()))
    }
}