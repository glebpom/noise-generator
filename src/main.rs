extern crate portaudio;
extern crate clap;
extern crate rand;

use portaudio as pa;
use clap::{App, Arg};
use std::str;
use std::process;

const CHANNELS: i32 = 1;
const SAMPLE_RATE: f64 = 44_100.0;
const FRAMES_PER_BUFFER: u32 = 64;

fn run(volume: f32, is_white: bool) -> Result<(), pa::Error> {
    if is_white {
        println!("Generating white noise");
    } else {
        println!("Generating pink noise");
    }
    let pa = try!(pa::PortAudio::new());

    let mut settings = try!(pa.default_output_stream_settings(CHANNELS, SAMPLE_RATE, FRAMES_PER_BUFFER));

    settings.flags = pa::stream_flags::CLIP_OFF;

    let mut b0 = rand::random::<f32>();
    let mut b1 = rand::random::<f32>();
    let mut b2 = rand::random::<f32>();
    let mut b3 = rand::random::<f32>();
    let mut b4 = rand::random::<f32>();
    let mut b5 = rand::random::<f32>();
    let mut b6 = rand::random::<f32>();
    let mut white = 0.0;
    let mut pink = 0.0;
    let callback = move |pa::OutputStreamCallbackArgs { buffer, frames, .. }| {
        let mut idx = 0;
        for _ in 0..frames {
            white = rand::random::<f32>();

            if is_white == false {
                // http://www.firstpr.com.au/dsp/pink-noise/#Pseudo
                b0 = 0.99886 * b0 + white * 0.0555179;
                b1 = 0.99332 * b1 + white * 0.0750759;
                b2 = 0.96900 * b2 + white * 0.1538520;
                b3 = 0.86650 * b3 + white * 0.3104856;
                b4 = 0.55000 * b4 + white * 0.5329522;
                b5 = -0.7616 * b5 - white * 0.0168980;
                pink = b0 + b1 + b2 + b3 + b4 + b5 + b6 + white * 0.5362;
                b6 = white * 0.115926;

                buffer[idx] = (pink / 37.0) * volume;
            } else {
                buffer[idx] = white * volume;
            }

            idx += 1;
        }
        pa::Continue
    };

    let mut stream = try!(pa.open_non_blocking_stream(settings, callback));

    try!(stream.start());

    loop {
        pa.sleep(1_000);
    }

    //    try!(stream.stop());
    //    try!(stream.close());
    //
    //    println!("Done");

    //    Ok(())
}

fn main() {
    let matches = App::new("Noise generator")
        .version("0.1")
        .author("Gleb Pomykalov <gleb@pomykalov.ru>")
        .about("Generates white/pink noise")
        .arg(Arg::with_name("white")
            .short("w")
            .long("white")
            .help("Generate white noise (Pink by default)"))
        .arg(Arg::with_name("volume")
            .short("v")
            .long("vol")
            .help("Volume level 0.0-1.0")
            .default_value("1.0")
            .takes_value(true))
        .get_matches();

    let mut volume = 1.0;
    match matches.value_of("volume").unwrap().parse::<f32>() {
        Ok(v) => {
            if v < 0.0 || v > 1.0 {
                println!("Failed to parse volume. Should be between number between 0.0 and 1.0");
                process::exit(1);
            }
            volume = v;
        },
        Err(_) => {
            println!("Failed to parse volume. Should be between number between 0.0 and 1.0")
        }
    };
    let mut is_white = false;
    if matches.occurrences_of("white") > 0 {
        is_white = true;
    }
    run(volume, is_white).unwrap()
}

