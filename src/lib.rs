use rodio::{Decoder, OutputStream, source::Source};
use std::fs::File;
use std::io::BufReader;

pub fn play(filepath: &str) {
    let (_stream, stream_handle) = OutputStream::try_default().unwrap();
    let file = BufReader::new(File::open(filepath).unwrap());
    let source = Decoder::new(file).unwrap();

    let mut dur: u64 = 0;
    if let Some(duration) = source.total_duration() {
        dur = duration.as_millis() as u64;
    }

    _ = stream_handle.play_raw(source.convert_samples());
    std::thread::sleep(std::time::Duration::from_millis(dur));
}

#[test]
fn play_audio() {
    play("test.wav");
}
