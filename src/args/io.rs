use std::error::Error;
use std::fmt;

#[derive(Debug)]
enum CustomError {
    SameFileTypeError,
    InvalidTypeError,
}

impl fmt::Display for CustomError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            CustomError::SameFileTypeError => {
                write!(f, "Input and output file types can't be the same")
            }
            CustomError::InvalidTypeError => write!(f, "Invalid type error"),
        }
    }
}

impl Error for CustomError {}

#[derive(Debug, PartialEq)]
pub enum IoDirection {
    AudioToImage,
    ImageToAudio,
}

pub fn get_io_direction(input: &str, output: &str) -> Result<IoDirection, Box<dyn Error>> {
    let input_type = get_file_type(input);
    let output_type = get_file_type(output);

    if input_type == output_type {
        return Err(Box::new(CustomError::SameFileTypeError));
    }

    if input_type == FileType::Invalid || output_type == FileType::Invalid {
        return Err(Box::new(CustomError::InvalidTypeError));
    }

    if input_type == FileType::Audio {
        Ok(IoDirection::AudioToImage)
    } else {
        Ok(IoDirection::ImageToAudio)
    }
}

#[derive(PartialEq)]
enum FileType {
    Image,
    Audio,
    Invalid,
}

fn get_file_type(filename: &str) -> FileType {
    let audio_extensions: Vec<&str> = vec![".wav"];
    let image_extensions: Vec<&str> = vec![".png"];

    if audio_extensions.iter().any(|&ext| filename.ends_with(ext)) {
        FileType::Audio
    } else if image_extensions.iter().any(|&ext| filename.ends_with(ext)) {
        FileType::Image
    } else {
        FileType::Invalid
    }
}
