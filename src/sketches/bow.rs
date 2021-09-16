// use std::time::{SystemTime, UNIX_EPOCH};

// use nannou::color::{rgb_u32, Gradient};
// use nannou::headless::HeadlessApp;
// use nannou::noise::{NoiseFn, Seedable};
use nannou::prelude::*;
// use nannou::rand::prelude::StdRng;
use nannou::rand::{thread_rng, Rng /*, SeedableRng */};

fn field(p: Vec2) -> Vec2 {
    let f = thread_rng().gen_range(-0.8..=0.8);
    let c = thread_rng().gen_range(-0.8..=0.8);
    let u = thread_rng().gen_range(-3.0..=3.0);
    let v = thread_rng().gen_range(-3.0..=3.0);

    let x = p.x + f * (p.y * u).tan().cos();
    let y = p.y + c * (p.x * v).tan().tan().sin();
    vec2(x, y)
}

// fn main() {
//     HeadlessApp::new(7200, 7200).run(|app, frame, nth| {
//         let start = SystemTime::now();
//         let since_the_epoch = start
//             .duration_since(UNIX_EPOCH)
//             .expect("Time went backwards");

//         let file_path = app::find_project_path()
//             .expect("failed to locate `project_path`")
//             .join("headless")
//             .join(format!(
//                 "weird-fields-{}-{:03}",
//                 since_the_epoch.as_secs(),
//                 nth
//             ))
//             .with_extension("jpg");

//         app.main_window().capture_frame(file_path);

//         if nth >= 5 {
//             app.quit();
//         }

//         let colors: Vec<LinSrgb<f32>> = [
//             // rgb_u32(0xffec5c),
//             // rgb_u32(0x008dcb),
//             lin_srgb(0xff as f32, 0xec as f32, 0x5c as f32),
//             lin_srgb(0x00 as f32, 0xcf as f32, 0xfa as f32),
//             // lin_srgb(0x6e as f32, 0xb5 as f32, 0xc0 as f32),
//             // lin_srgb(0x00 as f32, 0x6c as f32, 0x84 as f32),
//             // lin_srgb(0xe2 as f32, 0xe8 as f32, 0xe4 as f32),
//         ]
//         .iter()
//         .map(|c| (c.red / 255_f32, c.green / 255_f32, c.blue / 255_f32).into())
//         .collect();

//         // let seed = thread_rng().gen_range(10..1000);
//         let seed = nth;
//         let noise = nannou::noise::OpenSimplex::new().set_seed(seed as u32);

//         // Begin drawing
//         let draw = app.draw();

//         // draw.background().color(rgb_u32(0xFFF5EB));
//         draw.background().color(rgb_u32(0x2d3033));
//         // draw.background().color(WHITE);
//         let w = app.window_rect();

//         let gradient: Gradient<LinSrgb<f32>> = Gradient::new(colors.clone());
//         let mut rng = StdRng::seed_from_u64(seed);
//         let mut points = vec![];
//         for _ in 0..1000000 {
//             // let vx = model.rng.gen_range(-1.0..=1.0);
//             // let vy = model.rng.gen_range(-1.0..=1.0);
//             let vx = rng.gen_range(-1.0..=1.0);
//             let vy = rng.gen_range(-1.0..=1.0);
//             let v = vec2(vx, vy); // random input vector
//             let fv = field(v);
//             // let fv = fv.sub(v);
//             // let fv = field(fv);

//             let alpha = v.angle();
//             let beta = fv.angle();

//             let nalpha = noise.get([v.x as f64, fv.y as f64, alpha as f64]) as f32;
//             let nbeta = noise.get([fv.x as f64, v.y as f64, beta as f64]) as f32;

//             let x = map_range(
//                 alpha + nalpha,
//                 -TAU,
//                 TAU,
//                 w.left() - 250.0,
//                 w.right() + 250.0,
//             );
//             let y = map_range(beta + nbeta, -TAU, TAU, w.bottom() - 250.0, w.top() + 250.0);

//             points.push(vec2(x, y))
//         }

//         // for _ in 0..100000 {
//         //     let x = rng.gen_range(w.left()..w.right());
//         //     let y = rng.gen_range(w.bottom()..w.top());
//         //     draw.ellipse()
//         //         .x_y(x, y)
//         //         .w_h(1.0, 1.0)
//         //         .color(rgba(128.0, 128.0, 128.0, 0.4));
//         // }

//         // for point in points.iter_mut() {
//         // point.rotate(TAU / 4.0);
//         // point.x = point.x + point.x.cos() * TAU / 4.0;
//         // point.y = point.y + point.y.sin() * TAU / 4.0;
//         // }
//         let points: Vec<Vec2> = points
//             .iter_mut()
//             .map(|point| {
//                 // let x = point.x + point.x.cos() * TAU / 4.0;
//                 // let y = point.y + point.y.sin() * TAU / 4.0;
//                 let point = point.rotate(-TAU / 8.0);
//                 vec2(point.x, point.y)
//             })
//             .collect();

//         // let mut lengths = vec![];
//         for point in points.into_iter() {
//             let length = point.length();
//             // lengths.push(length);
//             draw.ellipse()
//                 .xy(point)
//                 .w_h(2.0, 2.0)
//                 .color(gradient.get(map_range(length - point.x, 0.0, 1130.0, 0.0, 1.0)));
//         }
//         // let max = lengths.iter().fold(-f32::INFINITY, |a, &b| a.max(b));
//         // println!("{}", max);

//         // Write the result of our drawing to the window's frame.
//         draw.to_frame(app, &frame).unwrap();
//     });
// }
