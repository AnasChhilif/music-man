use rodio::{source::Source, Decoder, OutputStream};
use std::fs::File;
use std::io::BufReader;

mod infoparser;

fn main() {
    println!("Hello, world!");

    let (_stream, stream_handle) = OutputStream::try_default().unwrap();

    // Load a sound from a file, using a path relative to Cargo.toml
    let file = BufReader::new(File::open("music/test2.mp3").unwrap());

    // Decode that sound file into a source
    let source = Decoder::new(file).unwrap();

    // Play the sound directly on the device
    let _ =stream_handle.play_raw(source.convert_samples());

    let mut info = infoparser::Info::new();

    info.fill_info("music/test2.mp3").unwrap();

    println!("Artist: {}", info.artist);
    println!("Title: {}", info.title);
    println!("Album: {}", info.album);


    // The sound plays in a separate audio thread,
    // so we need to keep the main thread alive while it's playing.
    std::thread::sleep(std::time::Duration::from_secs(10));
}
