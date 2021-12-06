use std::f64::consts::PI;

use wasm_bindgen::{JsCast, JsValue};
use web_sys::{CanvasRenderingContext2d, HtmlCanvasElement, HtmlImageElement, Storage, console::info, window};


pub fn set_item(key:&str, value:&str) {
    let storage = window().unwrap().local_storage().unwrap().unwrap();
    storage.set_item(key, value).unwrap();
}

pub fn get_item(key:&str) -> Option<String> {
    let storage = window().unwrap().local_storage().unwrap().unwrap();
    let item = storage.get_item(key).unwrap();
    return item;
}


pub struct Canvas {
    context:CanvasRenderingContext2d,
    canvas:HtmlCanvasElement,
    images:[HtmlImageElement;1]
}

pub fn performance_now_ms() -> f64 {
    window().unwrap().performance().unwrap().now()
}

impl Canvas {
    pub fn new() -> Canvas {
        let document = web_sys::window().unwrap().document().unwrap();
        let canvas = document.get_element_by_id("primary").unwrap();
        let canvas: web_sys::HtmlCanvasElement = canvas
            .dyn_into::<web_sys::HtmlCanvasElement>()
            .map_err(|_| ())
            .unwrap();

        let context = canvas
            .get_context("2d")
            .unwrap()
            .unwrap()
            .dyn_into::<web_sys::CanvasRenderingContext2d>()
            .unwrap();

        context.set_image_smoothing_enabled(false);
        context.set_font("1px Arial");
        context.set_text_align("center");
        context.set_text_baseline("middle");

        Canvas {
            context,
            canvas,
            images:[HtmlImageElement::new().unwrap()],
        }
    }

    pub fn width(&self) -> u32 {
        self.canvas.width()
    }

    pub fn height(&self) -> u32 {
        self.canvas.height()
    }

    pub fn clear(&self) {
        self.context.clear_rect(0.0, 0.0, self.width() as f64, self.height() as f64);
    }

    pub fn begin_path(&self) {
        self.context.begin_path();
    }

    pub fn move_to(&self, x:f64, y:f64) {
        self.context.move_to(x, y);
    }

    pub fn line_to(&self, x:f64, y:f64) {
        self.context.line_to(x, y);
    }

    pub fn close_path(&self) {
        self.context.close_path();
    } 

    pub fn fill(&self) {
        self.context.fill();
    }

    pub fn stroke(&self) {
        self.context.stroke();
    }

    pub fn set_fille_style(&self, value:&str) {
        self.context.set_fill_style(&JsValue::from_str(value));
    }

    pub fn set_stroke_style(&self, value:&str) {
        self.context.set_stroke_style(&JsValue::from_str(value));
    }

    pub fn draw_circle(&self, x:f64, y:f64, r:f64) {
        self.context.begin_path();
        let _ = self.context.arc(x, y, r, 0.0, 2.0 * PI);
        self.context.stroke();
    }

    pub fn _fill_rect(&self, x:f64, y:f64, w:f64, h:f64) {
        self.context.fill_rect(x, y, w, h);
    }

    pub fn set_text_style(&self, text_align:&str, baseline:&str) {
        self.context.set_text_align(text_align);
        self.context.set_text_baseline(baseline);
    }

    pub fn fill_text(&self, text:&str, x:f64, y:f64) {
        let _ = self.context.fill_text(text, x, y);
    }

    pub fn save(&self) {
        self.context.save();
    }

    pub fn restore(&self) {
        self.context.restore();
    }

    pub fn set_image_src(&self, img:usize, src:&str) {
        if let Some(img) = self.images.get(img) {
            img.set_src(src);
        }
    }

    pub fn draw_normalized_image(&self, img:usize, x:f64, y:f64) {
        if let Some(img) = self.images.get(img) {
            //let _ = self.context.draw_image_with_html_image_element(&img, x, y);
            let image = img;
            let dx = x - 0.5;
            let dy = y - 0.5;
            let dw = 1.0;
            let dh = 1.0;
            let _ = self.context.draw_image_with_html_image_element_and_dw_and_dh(image, dx, dy, dw, dh);
        }
    }

    pub fn set_scale(&self, scale:f64) {
        let _ = self.context.set_transform(1.0, 0.0, 0.0, 1.0, 0.0, 0.0);
        let _ = self.context.scale(scale, scale);

        //self.context.set_stroke_style(&JsValue::from_str("red"));
        self.context.set_line_width(1.0 / scale);
        
    }
}


pub fn play_sound(path:&str) {
    let result = web_sys::HtmlAudioElement::new_with_src(path);
    if let Ok(result) = result {
        result.play();
    }
}