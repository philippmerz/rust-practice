use std::thread::current;

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
    let mut rng = rng();
    let steps: Vec<f32> = (0..(w/UNIT).round() as i32)
    .map(|_| normal.sample(&mut rng)).collect();

    draw.background().color(WHITE);

    let mut currentY = 0.0;
    for (i, step) in steps.iter().enumerate() {
        draw.line()
            .start(pt2(i as f32 * UNIT - w / 2.0, currentY))
            .end(pt2((i + 1) as f32 * UNIT - w / 2.0, currentY + *step))
            .weight(3.0)
            .color(BLACK);
        currentY += step;
    }
}