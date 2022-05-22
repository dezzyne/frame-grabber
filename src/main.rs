extern crate log;

use std::env;
use std::fs::File;
use std::io::Write;
use std::path::Path;

use log::Level;
use path_absolutize::Absolutize;
use vid2img::FileSource;

fn main() {
    if env::args().len() != 3 {
        return;
    }
    let args: Vec<String> = env::args().collect();
    let input_file = &args[1];
    let output_directory = &args[2];

    let input_file_as_path = Path::new(input_file);
    if !input_file_as_path.is_file() {
        log::log!(Level::Trace, "Input argument should be a file.");
        return;
    }

    let output_directory_as_path = Path::new(output_directory);
    if !output_directory_as_path.is_dir() {
        log::log!(Level::Trace, "Output argument should be a directory.");
        return;
    }

    log::log!(
        Level::Debug,
        "{}\n{}",
        input_file_as_path.absolutize().unwrap().to_str().unwrap(),
        output_directory_as_path
            .absolutize()
            .unwrap()
            .to_str()
            .unwrap()
    );

    #[cfg(target_os = "macos")]
    env::set_var("GST_PLUGIN_SYSTEM_PATH", "/usr/local/lib/gstreamer-1.0");

    let input_file_as_path_as_ref = input_file_as_path.as_ref();

    let video_source = FileSource::new(input_file_as_path_as_ref, (1920, 1080)).unwrap();

    let mut first_frame = File::create(output_directory_as_path.join("first_image.png")).unwrap();

    for (_n, frame) in video_source.into_iter().enumerate() {
        if let Ok(Some(png_img_data)) = frame {
            first_frame.write_all(&png_img_data).unwrap();
            break;
        }
    }
}
