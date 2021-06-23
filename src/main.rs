use nannou::lyon::lyon_tessellation::LineJoin;
use nannou::noise::NoiseFn;
use nannou::noise::Seedable;
use nannou::prelude::*;
use nannou::rand::rngs::StdRng;
use nannou::rand::{Rng, SeedableRng};

struct Model {
    seed: u64,
}

fn main() {
    nannou::app(model).run();
}

fn model(app: &App) -> Model {
    app.set_loop_mode(LoopMode::loop_once());

    let _window = app
        .new_window()
        .size(1024, 768)
        .view(view)
        .key_pressed(key_pressed)
        .build()
        .unwrap();

    Model { seed: 1 }
}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();
    let w = app.window_rect();

    draw.background().color(WHITE);

    let mut rng = StdRng::seed_from_u64(model.seed);
    let noise = nannou::noise::Perlin::new().set_seed(model.seed as u32);

    let line_count = 3000;
    let min_vertices_per_line = 5;
    let max_vertices_per_line = 30;
    let step_size = 5.0;

    let lines = (0..line_count).map(|_| {
        let mut point = pt2(
            rng.gen_range(w.left()..w.right()),
            rng.gen_range(w.bottom()..w.top()),
        );
        let mut line: Vec<Vec2> = vec![point];

        for _ in 0..(rng.gen_range(min_vertices_per_line..max_vertices_per_line)) {
            let scaled_x = point[0] * 0.0005;
            let scaled_y = point[1] * 0.0005;
            let noise_value = noise.get([scaled_x as f64, scaled_y as f64]);
            println!("Noise value: {}", noise_value);

            let angle = map_range(noise_value, 0.0, 1.0, 0.0, PI * 2.0 as f32);
            println!("Angle: {}", angle);

            point = pt2(
                point[0] + step_size * angle.sin(),
                point[1] + step_size * angle.cos(),
            );
            line.push(point);
        }

        line
    });

    for line in lines {
        draw.polyline()
            .join(LineJoin::Round)
            .weight(1.0)
            .color(BLUE)
            .points(line);
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
