use nannou::color::{rgb_u32, Gradient};
use nannou::noise::NoiseFn;
use nannou::noise::Seedable;
use nannou::prelude::*;
use nannou::rand::rngs::StdRng;
use nannou::rand::{Rng, SeedableRng};

struct Model {
    seed: u64,
    colors: [Rgb<u8>; 9],
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

    let colors = [
        rgb_u32(0xfff7ec),
        rgb_u32(0xfee8c8),
        rgb_u32(0xfdd49e),
        rgb_u32(0xfdbb84),
        rgb_u32(0xfc8d59),
        rgb_u32(0xef6548),
        rgb_u32(0xd7301f),
        rgb_u32(0xb30000),
        rgb_u32(0x7f0000),
    ];

    Model { seed: 1, colors }
}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();
    let w = app.window_rect();

    draw.background().color(BLACK);

    let mut rng = StdRng::seed_from_u64(model.seed);
    let noise = nannou::noise::Perlin::new().set_seed(model.seed as u32);
    let gradient: Gradient<LinSrgb<f32>> = Gradient::new(
        model
            .colors
            .iter()
            .map(|c| (c.red as f32, c.green as f32, c.blue as f32).into()),
    );

    println!("gradient {:#?}", gradient);
    println!("gradient get 0 {:#?}", gradient.get(0.0));
    println!("gradient get 0.5 {:#?}", gradient.get(0.5));
    println!("gradient get 1.0 {:#?}", gradient.get(1.0));

    let line_count = 3000;
    let min_vertices_per_line = 5;
    let max_vertices_per_line = 30;
    let step_size = 5.0;

    let lines = (0..line_count).map(|_| {
        let mut point = pt2(
            rng.gen_range(w.left() - 30.0..w.right() + 30.0),
            rng.gen_range(w.bottom() - 30.0..w.top() + 30.0),
        );
        let mut line: Vec<(Vec2, LinSrgb<f32>)> = vec![(point, gradient.get(0.0))];

        for i in 0..(rng.gen_range(min_vertices_per_line..max_vertices_per_line)) {
            let scaled_x = point[0] * 0.0005;
            let scaled_y = point[1] * 0.0005;
            let noise_value = noise.get([scaled_x as f64, scaled_y as f64]);

            let angle = map_range(noise_value, 0.0, 1.0, 0.0, PI * 2.0 as f32);

            point = pt2(
                point[0] + step_size * angle.cos(),
                point[1] + step_size * angle.sin(),
            );
            line.push((point, gradient.get(map_range(i, 0, line.len(), 0.0, 1.0))));
        }

        line
    });

    for line in lines {
        draw.polyline()
            .join_round()
            .weight(1.0)
            .points_colored(line);
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
