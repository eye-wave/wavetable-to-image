use hound::{SampleFormat, WavReader};
use std::fs::File;

pub fn decode_wav(file_path: &str, window_size: &usize) -> Vec<f32> {
    let file = match File::open(file_path) {
        Ok(file) => file,
        Err(err) => {
            println!("{:?}", err);
            return Vec::new();
        }
    };

    let reader = match WavReader::new(file) {
        Ok(reader) => reader,
        Err(err) => {
            println!("{:?}", err);
            return Vec::new();
        }
    };

    let max_size = *window_size * 256;
    let sample_format = reader.spec().sample_format;

    if sample_format != SampleFormat::Float && sample_format != SampleFormat::Int {
        return vec![];
    }

    let mut samples: Vec<f32> = Vec::new();
    let max_value = (1 << (reader.spec().bits_per_sample - 1)) as f32;

    match sample_format {
        SampleFormat::Float => {
            for sample in reader.into_samples::<f32>() {
                if let Ok(s) = sample {
                    samples.push(s);
                } else {
                    break;
                }

                if samples.len() >= max_size {
                    break;
                }
            }
        }
        SampleFormat::Int => {
            for sample in reader.into_samples::<i32>() {
                if let Ok(s) = sample {
                    samples.push(s as f32 / max_value);
                } else {
                    break;
                }

                if samples.len() >= max_size {
                    break;
                }
            }
        }
    }

    samples
}
