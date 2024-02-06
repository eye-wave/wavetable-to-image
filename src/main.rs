use args::io::get_io_direction;
use clap::Parser;

use crate::{args::io::IoDirection, decoders::audio::decode_wav, encoders::image::generate_image};

mod args;
mod decoders;
mod encoders;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    input: String,

    #[arg(short, long)]
    output: String,

    #[arg(short)]
    n: u8,

    window_size: Option<usize>,
}

fn main() {
    let args = Args::parse();

    let io_direction = match get_io_direction(&args.input, &args.output) {
        Ok(dir) => dir,
        Err(err) => panic!("{}", err),
    };

    if io_direction == IoDirection::ImageToAudio {
        println!("Not implemented yet");
        return;
    }

    println!("{} -> {}", &args.input, &args.output);

    let window_size = args.window_size.unwrap_or(2048);
    let wav_data = decode_wav(&args.input, &window_size);

    generate_image(&wav_data, &window_size, &args.output, &(args.n as usize));
}
