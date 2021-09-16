use nannou::color::{rgb_u32, Gradient};
use nannou::noise::NoiseFn;
use nannou::noise::Seedable;
use nannou::prelude::*;
use nannou::rand::rngs::StdRng;
use nannou::rand::{Rng, SeedableRng};
use std::time::{SystemTime, UNIX_EPOCH};

pub struct Model {
    seed: u64,
    colors: Vec<LinSrgb<f32>>,
}

pub fn model(app: &App) -> Model {
    app.set_loop_mode(LoopMode::loop_once());
    let _window = app
        .new_window()
        // .size(1024, 768)
        .size(1024, 768)
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

    Model { seed: 1, colors }
    // Model {
    //     seed: thread_rng().gen_range(10..1000),
    //     colors,
    // }
}

const NOISE_SCALE: f32 = 0.005;

pub fn view(app: &App, model: &Model, frame: Frame) {
    if app.elapsed_frames() > 1 {
        app.quit();
    }

    let draw = app.draw();
    let w = app.window_rect();

    draw.background().color(rgb_u32(0xFFF5EB));
    // draw.background().color(BLACK);
    // draw.background().color(WHITE);

    let mut rng = StdRng::seed_from_u64(model.seed);
    let noise = nannou::noise::OpenSimplex::new().set_seed(model.seed as u32);
    let gradient: Gradient<LinSrgb<f32>> = Gradient::new(model.colors.clone());

    let line_count = 550;
    let min_vertices_per_line = 50;
    let max_vertices_per_line = 100;
    let step_size = 2.0;

    let lines = (0..line_count).map(|_| {
        let mut point = pt2(
            rng.gen_range(w.left() + 300.0..w.right() - 300.0),
            rng.gen_range(w.bottom() + 250.0..w.top() - 250.0),
        );
        let mut line: Vec<Vec2> = vec![point];

        for _ in 0..(rng.gen_range(min_vertices_per_line..max_vertices_per_line)) {
            let scaled_x = point[0] * NOISE_SCALE;
            let scaled_y = point[1] * NOISE_SCALE;
            let noise_value = noise.get([scaled_x as f64, scaled_y as f64]);

            let angle = map_range(noise_value, -1.0, 1.0, 0.0, TAU as f32);

            point = pt2(
                point[0] + step_size * angle.cos(),
                point[1] + step_size * angle.sin(),
            );
            line.push(point);
        }
        line
    });

    for line in lines {
        draw.polyline()
            .join_round()
            .weight(1.0)
            .points_colored(line.iter().enumerate().map(|(_i, &point)| {
                // let color = gradient.get(map_range(point.x, 0.0, w.right() - 250.0, 0.0, 1.0));

                let scaled_x = point.x * NOISE_SCALE;
                let scaled_y = point.y * NOISE_SCALE;
                let noise_value_0 = noise.get([scaled_x as f64, scaled_y as f64, 0.0]);
                let noise_value_1 = noise.get([scaled_x as f64, scaled_y as f64, 1.0]);
                let gradient_value = map_range(noise_value_1, -1.0, 1.0, 0.0, 1.0);
                println!(
                    "nv0 {} | nv1 {} | gv {}",
                    noise_value_0, noise_value_1, gradient_value
                );
                let color = gradient.get(gradient_value);

                (point, color)
            }));
    }

    draw.to_frame(app, &frame).unwrap();

    let file_path = captured_frame_path(app, &frame);
    app.main_window().capture_frame(file_path);
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
