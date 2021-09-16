pub mod sketches;

fn main() {
    nannou::app(sketches::sea_ways::model)
        .update(|_, _, _| {})
        .run();
}
