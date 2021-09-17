pub mod sketches;

use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(name = "sketches", about = "A CLI wrapper over my nannou sketches.")]
struct Opt {
    /// Set the seed.
    #[structopt(short, long, default_value = "1")]
    seed: u64,

    /// Set the number of frames. By default one.
    #[structopt(short, long, default_value = "1")]
    times: u64,

    /// Set the width. By default 1000.
    #[structopt(short, long, default_value = "1000")]
    width: u64,

    /// Set the height. By default 1000.
    #[structopt(short, long, default_value = "1000")]
    height: u64,

    /// Set the name of the sketch.
    #[structopt(long, default_value = "unnamed")]
    name: String,
    // /// Set the width.
    // #[structopt(short, long, default_value = "1")]
    // times: u64,
}

fn main() {
    let opt = Opt::from_args();

    nannou::app(sketches::sea_ways::model)
        .update(|_, _, _| {})
        .run();
}
