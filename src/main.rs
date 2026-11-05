use nannou::prelude::*;
use rand::rng;
use rand_distr::{Distribution, Normal};

const UNIT: f32 = 1e-3;

fn main() {
    nannou::app(model).update(update).run();
}

struct Model {
    _window: Entity,
}

fn model(app: &App) -> Model {
    let _window = app
        .new_window()
        .maximized(true)
        .title("Brownian motion")
        .view(view)
        .build();
    Model { _window }
}

fn update(_app: &App, _model: &mut Model) {}

fn view(app: &App, _model: &Model) {
    let draw = app.draw();
    let window = app.window_rect();
    let (w, h) = (window.w(), window.h());

    let normal = Normal::new(0.0_f32, UNIT * w).unwrap();
    let seg: f32 = normal.sample(&mut rng());

    draw.background().color(WHITE);
    draw.line()
        .start(pt2(-1.0 * (w / 2.0), 0.0))
        .end(pt2(w / 2.0, seg))
        .weight(3.0)
        .color(BLACK);
}
