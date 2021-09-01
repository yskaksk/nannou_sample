use nannou::prelude::*;
use nannou::rand::random_range;

struct Face {
    size: f32,
    x: f32,
    y: f32,
}

struct Model {
    faces: Vec<Face>
}

fn main() {
    nannou::app(model)
        .event(event)
        .update(update)
        .simple_window(view)
        .size(600, 400)
        .run();
}

fn model(_app: &App) -> Model {
    Model {faces: vec![]}
}

fn update(_app: &App, _model: &mut Model, _update: Update) {
}

fn event(app: &App, model: &mut Model, event: Event) {
    match event {
        Event::WindowEvent {
            id: _,
            simple: some_we
        } => {
            if let Some(we) = some_we {
                match we {
                    MouseMoved(pt2) => {
                        let x = pt2.x;
                        let y = pt2.y;
                        model.faces.push(new_face(&app, x, y));
                    }
                    _ => {}
                }
            }
        }
        _ => {}
    }
}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();
    draw.background().rgb(0.0, 127.0, 255.0);

    for f in model.faces.iter() {
        draw_face(&draw, f.size, f.x, f.y)
    }

    draw.to_frame(app, &frame).unwrap();
}

fn new_face(app: &App, x: f32, y: f32) -> Face {
    let t = app.time;
    let size = t.sin() * 110.0 + 140.0;
    return Face {size, x, y}
}

fn draw_face(draw: &Draw, size: f32, x: f32, y: f32) {
    // 輪郭
    ellipse_with_stroke(&draw, x, y, size, size, WHITE, BLACK);
    // 口
    //ellipse_with_stroke(&draw, x, -size / 5.0 + y, size / 3.0, size / 8.0, RED, BLACK);
    draw.ellipse()
        .x_y(x, -size / 5.0 + y)
        .w(size / 3.0)
        .h(size / 8.0)
        .color(RED);
    // 目
    let eye_size = size / 10.0;
    draw.ellipse()
        .x_y(size / 5.0 + x, size / 5.0 + y)
        .w(eye_size)
        .h(eye_size)
        .color(BLACK);
    draw.ellipse()
        .x_y(-size / 5.0 + x, size / 5.0 + y)
        .w(eye_size)
        .h(eye_size)
        .color(BLACK);
}

fn ellipse_with_stroke(draw: &Draw, x: f32, y:f32, w:f32, h:f32, color: Rgb8, stroke_color: Rgb8) {
    let points = (0..=360).map(|i| {
        let radian = deg_to_rad(i as f32);
        let xpos = radian.sin() * (w / 2.0) + x;
        let ypos = radian.cos() * (h / 2.0) + y;
        (pt2(xpos, ypos), stroke_color)
    });
    draw.polyline()
        .weight(3.0)
        .points_colored(points);
    draw.ellipse()
        .x_y(x, y)
        .w(w)
        .h(h)
        .color(color);
}
