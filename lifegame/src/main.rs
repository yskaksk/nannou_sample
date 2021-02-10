use nannou;

use lifegame::{model, event, update, view};

fn main() {
    nannou::app(model)
        .event(event)
        .update(update)
        .simple_window(view)
        .run();
}
