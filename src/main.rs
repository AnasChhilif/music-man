use rodio::{source::Source, Decoder, OutputStream};
use std::fs::File;
use std::io::BufReader;

mod infoparser;

slint::slint! {
    import { Button, Slider } from "std-widgets.slint";
export component MainWindow inherits Window {
        Text {
            text: "hello world";
            color: green;
        }
        Button {
            width: 127px;
            height: 70px;
            text: "l7ma9 o lhrba";
            x: 122px;
            y: 48.8217px;
        }
        Slider {
            height: 71px;
            minimum: 0;
            value: 69;
            maximum: 100;
            x: 106px;
            y: 221px;
            width: 210px;
        }
    }
}

fn main() {
    println!("Hello, world!");
    MainWindow::new().unwrap().run().unwrap();

    let (_stream, stream_handle) = OutputStream::try_default().unwrap();

    // Load a sound from a file, using a path relative to Cargo.toml
    let file = BufReader::new(File::open("music/test2.mp3").unwrap());

    // Decode that sound file into a source
    let source = Decoder::new(file).unwrap();

    // Play the sound directly on the device
    let _ = stream_handle.play_raw(source.convert_samples());

    let mut info = infoparser::Info::new();

    info.fill_info("music/test2.mp3").unwrap();

    println!("Artist: {}", info.artist);
    println!("Title: {}", info.title);
    println!("Album: {}", info.album);

    // The sound plays in a separate audio thread,
    // so we need to keep the main thread alive while it's playing.
    std::thread::sleep(std::time::Duration::from_secs(5));
}
