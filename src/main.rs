use nannou::prelude::*;

const SMILEY_RADIUS: f32 = 200.0;
const SMILEY_EYE_RADIUS: f32 = 20.0;
const STROKE_WEIGHT: f32 = 10.0;
const EYE_INITIAL_POSITION_X: f32 = 100.0;
const EYE_INITIAL_POSITION_Y: f32 = 80.0;

pub struct Smiley {
    rotation: f32,
    smile: Vec<Point2>,
    eyes: Vec<(f32, f32)>,
}

impl Smiley {
    pub fn new() -> Self {
        let points: Vec<Point2> = (-1000..1000)
            .map(|i| {
                let x = i as f32 * 0.1;
                let y = 0.005 * (x * x) - 100.0;
                pt2(x, y)
            })
            .collect();

        Self {
            rotation: 0.0,
            smile: points,
            eyes: vec![
                (-EYE_INITIAL_POSITION_X, EYE_INITIAL_POSITION_Y),
                (EYE_INITIAL_POSITION_X, EYE_INITIAL_POSITION_Y),
            ],
        }
    }
}

fn main() {
    nannou::app(model)
        .update(update)
        .loop_mode(LoopMode::refresh_sync())
        .run();
}

fn model(app: &App) -> Smiley {
    let _window = app
        .new_window()
        .title("Smiley")
        .size(800, 800)
        .view(view)
        .build()
        .unwrap();

    Smiley::new()
}

fn update(_app: &App, model: &mut Smiley, _update: Update) {
    model.rotation += 0.01;
    model.eyes[0].0 = EYE_INITIAL_POSITION_X * model.rotation.cos();
    model.eyes[0].1 = EYE_INITIAL_POSITION_Y * model.rotation.sin();
    model.eyes[1].0 = -EYE_INITIAL_POSITION_X * model.rotation.sin();
    model.eyes[1].1 = EYE_INITIAL_POSITION_Y * model.rotation.cos();
}

fn view(app: &App, model: &Smiley, frame: Frame) {
    // Prepare to draw.
    let draw = app.draw();

    draw.background().color(BEIGE);

    draw.ellipse()
        .color(YELLOW)
        .radius(SMILEY_RADIUS)
        .stroke(BLACK)
        .stroke_weight(STROKE_WEIGHT);

    draw.ellipse()
        .color(BLACK)
        .radius(SMILEY_EYE_RADIUS)
        .x(model.eyes[0].0)
        .y(model.eyes[0].1);

    draw.ellipse()
        .color(BLACK)
        .radius(SMILEY_EYE_RADIUS)
        .x(model.eyes[1].0)
        .y(model.eyes[1].1);

    let points = (-1000..1000).map(|i| {
        let i = (i + 1000) as usize;
        let point = model.smile[i];
        (point, BLACK)
    });

    draw.polyline()
        .weight(10.0)
        .points_colored(points)
        .rotate(model.rotation);

    // draw.line().color(BLACK).weight(10.0).points(pt2(100.0, -80.0), pt2(-100.0, -80.0));

    // Write to the window frame.
    draw.to_frame(app, &frame).unwrap();
}
