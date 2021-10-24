use std::cmp;

use nannou::prelude::*;
use nannou::image;
use nannou::image::{GenericImageView, DynamicImage};


struct Model {
    texture: wgpu::Texture,
}

fn main() {
    nannou::app(model).run();
}

fn model(app: &App) -> Model {

    let assets = app.assets_path().unwrap();
    //let img_path = assets.join("images").join("neko_sample.jpg");
    let img_path = assets.join("images").join("mountains.jpg");
    let img = image::open(img_path).unwrap();
    let (img_w, img_h) = img.dimensions();
    app.new_window().size(img_w, img_h).view(view).build().unwrap();

    let wsize = 7;

    let ib = img.to_rgb8();

    let img = image::ImageBuffer::from_fn(img_w, img_h, |x, y| {
        let mut r = 0;
        let mut g = 0;
        let mut b = 0;
        let mut count = 0;
        for xx in x.checked_sub(wsize).unwrap_or(0)..=cmp::min(x+wsize, img_w-1) {
            for yy in y.checked_sub(wsize).unwrap_or(0)..=cmp::min(y+wsize, img_h-1) {
                let pixel = ib.get_pixel(xx, yy);
                r += pixel[0] as i32;
                g += pixel[1] as i32;
                b += pixel[2] as i32;
                count += 1;
            }
        }
        r = r / count;
        g = g / count;
        b = b / count;
        image::Rgb([r as u8, g as u8, b as u8])
    });
    let di = DynamicImage::ImageRgb8(img);
    let texture = wgpu::Texture::from_image(app, &di);

    Model {
        texture: texture,
    }
}

fn view(app: &App, model: &Model, frame: Frame) {
    frame.clear(BLACK);

    let draw = app.draw();
    draw.texture(&model.texture);

    draw.to_frame(app, &frame).unwrap();
}
