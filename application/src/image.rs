use egui_extras::RetainedImage;
use poll_promise::Promise;

pub struct WebImage {
    source: String,
    promise: Option<Promise<ehttp::Result<RetainedImage>>>,
}

impl WebImage {
    pub fn new<S: AsRef<str>>(source: S) -> Self {
        let source = source.as_ref().to_string();
        let promise = None;
        Self { source, promise }
    }

    pub fn source(&self) -> &String { &self.source }

    pub fn retained_image<C: FnMut(&mut egui::Ui, &RetainedImage)>(&mut self, ui: &mut egui::Ui, mut callback: C) {
        let promise = self.promise.get_or_insert_with(|| {
            // Begin download.
            // We download the image using `ehttp`, a library that works both in WASM and on native.
            // We use the `poll-promise` library to communicate with the UI thread.
            let ctx = ui.ctx().clone();
            let (sender, promise) = Promise::new();
            let request = ehttp::Request::get(&self.source);
            ehttp::fetch(request, move |response| {
                let image = response.and_then(parse_response);
                sender.send(image); // send the results back to the UI thread.
                ctx.request_repaint(); // wake up UI thread
            });
            promise
        });

        match promise.ready() {
            None => {
                ui.spinner(); // still loading
            }
            Some(Err(err)) => {
                ui.colored_label(ui.visuals().error_fg_color, err); // something went wrong
            }
            Some(Ok(image)) => {
                callback(ui, image);
            }
        }
    }
}

#[allow(clippy::needless_pass_by_value)]
fn parse_response(response: ehttp::Response) -> Result<RetainedImage, String> {
    let content_type = response.content_type().unwrap_or_default();
    if content_type.starts_with("image/") {
        RetainedImage::from_image_bytes(&response.url, &response.bytes)
    } else {
        Err(format!(
            "Expected image, found content-type {:?}",
            content_type
        ))
    }
}