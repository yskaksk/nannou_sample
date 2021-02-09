use nannou;

use tetris::model;
use tetris::event;
use tetris::update;
use tetris::view;

fn main() {
    nannou::app(model)
        .event(event)
        .update(update)
        .simple_window(view)
        .run();
}
