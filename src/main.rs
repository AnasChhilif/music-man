use rodio::{source::Source, Decoder, OutputStream};
use std::fs::File;
use std::io::BufReader;

mod infoparser;

slint::slint! {
    import { Button, Slider } from "std-widgets.slint";
    export component MainWindow inherits Window {
            in-out property<string> Title : "No Title";
            in-out property<string> Artist : "No Artist";
            in-out property<string> Album : "No Album";
        width: 600px;
        height: 400px;
         Text { text: "Title: " + root.Title;
              color: green;
             x: 10px; y: 40px;}
         Text { text: "Artist: " + root.Artist; color: green;
             x: 10px; y: 60px;}
         Text { text: "Album: " + root.Album; color: green;
             x: 10px; y: 80px;}
    }
}

fn main() {
    let (_stream, stream_handle) = OutputStream::try_default().unwrap();

    // Load a sound from a file, using a path relative to Cargo.toml
    let file = BufReader::new(File::open("music/test2.mp3").unwrap());

    // Decode that sound file into a source
    let source = Decoder::new(file).unwrap();

    // Play the sound directly on the device
    let _ = stream_handle.play_raw(source.convert_samples());
    let _main_window = MainWindow::new().unwrap().run().unwrap();

    let mut info = infoparser::Info::new();

    info.fill_info("music/test2.mp3").unwrap();

    println!("Artist: {}", info.artist);
    println!("Title: {}", info.title);
    println!("Album: {}", info.album);

    // The sound plays in a separate audio thread,
    // so we need to keep the main thread alive while it's playing.
    // std::thread::sleep(std::time::Duration::from_secs(5));
}
