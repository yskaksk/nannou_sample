use nannou;

use lifegame::{event, model, update, view};

fn main() {
    nannou::app(model)
        .event(event)
        .update(update)
        .simple_window(view)
        .run();
}
