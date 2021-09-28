use nannou::prelude::*;

#[derive(Clone)]
struct Line {
    points: Vec<Point2>
}

impl Line {
    fn new() -> Self {
        Line {points: vec![]}
    }

    fn is_empty(&self) -> bool {
        self.points.is_empty()
    }
}

struct Model {
    touch: bool,
    lines: Vec<Line>,
    line: Line
}

fn main() {
    nannou::app(model)
        .event(event)
        .update(update)
        .simple_window(view)
        .size(600, 600)
        .run();
}

fn model(_app: &App) -> Model {
    Model {touch: false, lines: vec![], line: Line::new()}
}

fn update(_app: &App, _model: &mut Model, _update: Update) {
}

fn event(_app: &App, model: &mut Model, event: Event) {
    match event {
        Event::WindowEvent {
            id: _,
            simple: some_we
        } => {
            if let Some(we) = some_we {
                match we {
                    MousePressed(_) => {
                        model.touch = true;
                    }
                    MouseReleased(_) => {
                        model.touch = false;
                        model.lines.push(model.line.clone());
                        model.line = Line::new()
                    }
                    MouseMoved(pp) => {
                        if model.touch {
                            model.line.points.push(pp);
                        }
                    }
                    KeyPressed(Key::Space) => {
                        model.lines = vec![];
                        model.line = Line::new();
                    }
                    KeyPressed(Key::Back) => {
                        if !model.line.is_empty() {
                            model.line = Line::new()
                        } else if !model.lines.is_empty() {
                            model.lines.pop();
                        }
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
    draw.background().rgb(255.0, 255.0, 255.0);

    for l in model.lines.iter() {
        draw_line(&draw, &l);
    }

    draw_line(&draw, &model.line);

    draw.to_frame(app, &frame).unwrap();
}

fn draw_line(draw: &Draw, line: &Line) {
    let points = line.points.iter().map(|pp| {
        (pt2(pp.x, pp.y), RED)
    });
    let rev = line.points.iter().map(|pp| {
        (pt2(-pp.x, -pp.y), BLUE)
    });
    draw.polyline()
        .weight(2.0)
        .points_colored(points);
    draw.polyline()
        .weight(2.0)
        .points_colored(rev);
}
