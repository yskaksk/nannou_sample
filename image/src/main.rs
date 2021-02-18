use nannou::prelude::*;

struct Model {
    texture: wgpu::Texture,
}

fn main() {
    nannou::app(model).run();
}

fn model(app: &App) -> Model {
    app.new_window().size(1024, 1024).view(view).build().unwrap();
    // load the image from disk and upload it to a GPU texture
    let assets = app.assets_path().unwrap();
    let img_path = assets.join("images").join("neko_sample.jpg");
    let texture = wgpu::Texture::from_path(app, img_path).unwrap();
    Model { texture }
}

fn view(app: &App, model: &Model, frame: Frame) {
    frame.clear(BLACK);

    let draw = app.draw();
    draw.texture(&model.texture);

    draw.to_frame(app, &frame).unwrap();
}

