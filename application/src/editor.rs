use egui::Vec2;
use crate::app::ADDRESS;
use crate::WebImage;

pub struct Editor {
    image: Option<WebImage>
}

impl Editor {
    pub fn new() -> Self {
        let image = Some(WebImage::new(format!("http://{}:8000/merged_canvas.698a8989.png", ADDRESS)));
        Self { image }
    }

    pub fn set_image(&mut self, image_source: Option<String>) {
        let should_change = self
            .image
            .as_ref()
            .map(|image| *image.source() != image_source.clone().unwrap_or_default())
            .unwrap_or(true);
        if should_change {
            self.image = image_source.map(|image_source| WebImage::new(image_source));
        }
    }

    pub fn show(&mut self, ui: &mut egui::Ui) {
        if let Some(image) = &mut self.image {
            image.retained_image(ui, |ui, image| {
                egui::ScrollArea::both().show(ui, |ui| {
                    image.show(ui);
                });
            })
        }
    }
}