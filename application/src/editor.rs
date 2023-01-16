use egui::{Color32, Frame, Image, Pos2, Rect, Sense, Stroke, Vec2, vec2};
use egui::emath::{RectTransform, Rot2};
use crate::app::ADDRESS;
use crate::WebImage;

pub struct Editor {
    image: Option<WebImage>,
    rotation: f32,
    translation: Vec2,
    zoom: f32,
    last_touch_time: f64,
}

impl Editor {
    pub fn new() -> Self {
        let image = Some(WebImage::new(format!("http://{}:8000/merged_canvas.698a8989.png", ADDRESS)));
        let rotation = 0.0;
        let translation = Vec2::ZERO;
        let zoom = 1.0;
        let last_touch_time = 0.0;
        Self { image, rotation, translation, zoom, last_touch_time }
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
        Frame::canvas(ui.style()).show(ui, |ui| {
            let (response, painter) =
                ui.allocate_painter(ui.available_size_before_wrap(), Sense::drag());

            // normalize painter coordinates to ±1 units in each direction with [0,0] in the center:
            let painter_proportions = response.rect.square_proportions();
            let to_screen = RectTransform::from_to(
                Rect::from_min_size(Pos2::ZERO - painter_proportions, 2. * painter_proportions),
                response.rect,
            );

            // check for touch input (or the lack thereof) and update zoom and scale factors, plus
            // color and width:
            let mut stroke_width = 1.;
            if let Some(multi_touch) = ui.ctx().multi_touch() {
                // This adjusts the current zoom factor and rotation angle according to the dynamic
                // change (for the current frame) of the touch gesture:
                self.zoom *= multi_touch.zoom_delta;
                self.rotation += multi_touch.rotation_delta;
                // the translation we get from `multi_touch` needs to be scaled down to the
                // normalized coordinates we use as the basis for painting:
                self.translation += to_screen.inverse().scale() * multi_touch.translation_delta;
                // touch pressure will make the arrow thicker (not all touch devices support this):
                stroke_width += 10. * multi_touch.force;

                self.last_touch_time = ui.input().time;
            }
            let zoom_and_rotate = self.zoom * Rot2::from_angle(self.rotation);
            let arrow_start_offset = self.translation + zoom_and_rotate * vec2(-0.5, 0.5);

            let color = if ui.visuals().dark_mode {
                Color32::WHITE
            } else {
                Color32::BLACK
            };

            // Paints an arrow pointing from bottom-left (-0.5, 0.5) to top-right (0.5, -0.5), but
            // scaled, rotated, and translated according to the current touch gesture:
            let arrow_start = Pos2::ZERO + arrow_start_offset;
            let arrow_direction = zoom_and_rotate * vec2(1., -1.);
            // painter.arrow(
            //     to_screen * arrow_start,
            //     to_screen.scale() * arrow_direction,
            //     Stroke::new(stroke_width, color),
            // );
            if let Some(image) = &mut self.image {
                image.retained_image(ui, |ui, image| {
                    let image = Image::new(image.texture_id(ui.ctx()), image.size_vec2())
                        .rotate(self.rotation, Vec2::ZERO);
                    ui.add(image);
                })
            }
        });
    }
}