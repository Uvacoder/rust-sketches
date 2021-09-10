use nannou::noise::Seedable;
use nannou::noise::{NoiseFn, OpenSimplex};
use nannou::prelude::*;
use std::time::{SystemTime, UNIX_EPOCH};

struct Model {
    things: Vec<Thing>,
    seed: u64,
    colors: Vec<LinSrgb<f32>>,
    noise: OpenSimplex,
}

struct Thing {
    positions: Vec<Vec2>,
}

impl Thing {
    pub fn new(p: Vec2) -> Self {
        let mut positions = Vec::new();
        positions.push(p);
        Thing { positions }
    }
}

fn main() {
    nannou::app(model).update(update).run();
}

const N_THINGS: usize = 2000;
const SCREEN_WIDTH: f32 = 1024.0;
const SCREEN_HEIGHT: f32 = 1024.0;

fn model(app: &App) -> Model {
    // app.set_loop_mode(LoopMode::loop_once());
    let _window = app
        .new_window()
        .size(SCREEN_WIDTH as u32, SCREEN_HEIGHT as u32)
        .view(view)
        .key_pressed(key_pressed)
        .build()
        .unwrap();
    let colors = [
        // lin_srgb(0xff as f32, 0xf7 as f32, 0xec as f32),
        // lin_srgb(0xfe as f32, 0xe8 as f32, 0xc8 as f32),
        // lin_srgb(0xfd as f32, 0xd4 as f32, 0x9e as f32),
        lin_srgb(0xfd as f32, 0xbb as f32, 0x84 as f32),
        lin_srgb(0xfc as f32, 0x8d as f32, 0x59 as f32),
        lin_srgb(0xef as f32, 0x65 as f32, 0x48 as f32),
        lin_srgb(0xd7 as f32, 0x30 as f32, 0x1f as f32),
        lin_srgb(0xb3 as f32, 0x00 as f32, 0x00 as f32),
        lin_srgb(0x7f as f32, 0x00 as f32, 0x00 as f32),
    ]
    .iter()
    .map(|c| (c.red / 255_f32, c.green / 255_f32, c.blue / 255_f32).into())
    .collect();

    let mut things = Vec::new();

    for i in 0..N_THINGS {
        let thing = Thing::new(vec2(
            (random::<f32>() - 0.5) * SCREEN_WIDTH,
            (random::<f32>() - 0.5) * SCREEN_HEIGHT,
        ));
        things.push(thing);
    }

    let seed = 1;
    let noise = nannou::noise::OpenSimplex::new().set_seed(seed as u32);
    Model {
        things,
        seed,
        colors,
        noise,
    }
    // Model {
    //     seed: thread_rng().gen_range(10..1000),
    //     colors,
    // }
}

const NOISE_SCALE: f32 = 0.005;

fn update(app: &App, model: &mut Model, _update: Update) {
    let time = app.elapsed_frames() as f32 / 120.0;
    let sn = 0.01 + time.cos() as f64 * 0.005;
    for thing in model.things.iter_mut() {
        thing.positions.clear();
        thing.positions.push(vec2(
            (random::<f32>() - 0.5) * SCREEN_WIDTH,
            (random::<f32>() - 0.5) * SCREEN_HEIGHT,
        ));

        for k in 0..10 {
            let last = thing.positions[0];
            let new = last
                + vec2(
                    model
                        .noise
                        .get([sn * last.x as f64, sn * last.y as f64, 0.0])
                        as f32,
                    model
                        .noise
                        .get([sn * last.x as f64, sn * last.y as f64, 1.0])
                        as f32,
                );
            thing.positions.insert(0, new);
        }
    }
}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();

    let time = app.elapsed_frames() as f32 / 60.0;

    if app.elapsed_frames() == 1 {
        draw.background().color(BLACK);
    }
    draw.rect()
        .w_h(SCREEN_WIDTH, SCREEN_HEIGHT)
        .color(srgba(0.0, 0.0, 0.0, 0.1));

    for thing in model.things.iter() {
        draw.polyline()
            .points(thing.positions.iter().cloned())
            .color(WHITE);
    }

    draw.to_frame(app, &frame).unwrap();

    // let file_path = captured_frame_path(app, &frame);
    // app.main_window().capture_frame(file_path);
}

fn captured_frame_path(app: &App, _frame: &Frame) -> std::path::PathBuf {
    let start = SystemTime::now();
    let since_the_epoch = start
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards");

    app.project_path()
        .expect("failed to locate `project_path`")
        .join("sketches")
        .join(format!("sea-ways-{}", since_the_epoch.as_secs()))
        .with_extension("jpg")
}

fn key_pressed(app: &App, _model: &mut Model, key: Key) {
    if key == Key::Q {
        app.quit();
    }
}
