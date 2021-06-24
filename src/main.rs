use nannou::color::{rgb_u32, Gradient};
use nannou::noise::NoiseFn;
use nannou::noise::Seedable;
use nannou::prelude::*;
use nannou::rand::rngs::StdRng;
use nannou::rand::{thread_rng, Rng, SeedableRng};

struct Model {
    seed: u64,
    colors: Vec<LinSrgb<f32>>,
}

fn main() {
    nannou::app(model).run();
}

fn model(app: &App) -> Model {
    app.set_loop_mode(LoopMode::loop_once());

    let _window = app
        .new_window()
        // .size(1024, 768)
        .size(1920, 1080)
        .view(view)
        .key_pressed(key_pressed)
        .build()
        .unwrap();

    let colors = [
        lin_srgb(0xfd as f32, 0xd4 as f32, 0x9e as f32),
        lin_srgb(0xfd as f32, 0xbb as f32, 0x84 as f32),
        lin_srgb(0xfc as f32, 0x8d as f32, 0x59 as f32),
        lin_srgb(0xef as f32, 0x65 as f32, 0x48 as f32),
        lin_srgb(0xd7 as f32, 0x30 as f32, 0x1f as f32),
        lin_srgb(0xb3 as f32, 0x00 as f32, 0x00 as f32),
        lin_srgb(0x7f as f32, 0x00 as f32, 0x00 as f32),
    ]
    .iter()
    .map(|c| {
        (
            c.red / 255 as f32,
            c.green / 255 as f32,
            c.blue / 255 as f32,
        )
            .into()
    })
    .collect();

    // Model { seed: 1, colors }
    Model {
        seed: thread_rng().gen_range(10..100),
        colors,
    }
}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();
    let w = app.window_rect();

    draw.background().color(rgb_u32(0x9F5F80));
    // draw.background().color(BLACK);
    // draw.background().color(WHITE);

    let mut rng = StdRng::seed_from_u64(model.seed);
    let noise = nannou::noise::Perlin::new().set_seed(model.seed as u32);
    let gradient: Gradient<LinSrgb<f32>> = Gradient::new(model.colors.clone());

    let line_count = 10000;
    let min_vertices_per_line = 10;
    let max_vertices_per_line = 50;
    let step_size = 5.0;

    let lines = (0..line_count).map(|_| {
        let mut point = pt2(
            rng.gen_range(w.left() - 300.0..w.right() + 300.0),
            rng.gen_range(w.bottom() - 300.0..w.top() + 300.0),
        );
        let mut line: Vec<Vec2> = vec![point];

        for _ in 0..(rng.gen_range(min_vertices_per_line..max_vertices_per_line)) {
            let scaled_x = point[0] * 0.0001;
            let scaled_y = point[1] * 0.0001;
            let noise_value = noise.get([scaled_x as f64, scaled_y as f64]);

            let angle = map_range(noise_value, 0.0, 1.0, 0.0, PI * 2.0 as f32);

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
            .points_colored(line.iter().enumerate().map(|(i, &point)| {
                let color = gradient.get(map_range(i, 0, line.len() - 1, 1.0, 0.0));
                (point, color)
            }));
    }

    draw.to_frame(app, &frame).unwrap();

    let file_path = captured_frame_path(app, &frame);
    app.main_window().capture_frame(file_path);
}

fn captured_frame_path(app: &App, frame: &Frame) -> std::path::PathBuf {
    app.project_path()
        .expect("failed to locate `project_path`")
        .join(app.exe_name().unwrap())
        .join(format!("{:03}", frame.nth()))
        .with_extension("png")
}

fn key_pressed(app: &App, _model: &mut Model, key: Key) {
    if key == Key::Q {
        app.quit();
    }
}
