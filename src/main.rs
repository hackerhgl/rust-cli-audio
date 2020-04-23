use std::io;
use std::io::BufReader;

fn main() {
    println!("Hello, world!");
    let device = rodio::default_output_device().unwrap();
    let sink = rodio::Sink::new(&device);

    let devices = rodio::devices;

    let file = std::fs::File::open("src/ambient.mp3").unwrap();
    sink.append(rodio::Decoder::new(BufReader::new(file)).unwrap());

    sink.play();

    let mut ctrl = String::new();

    loop {
        io::stdin().read_line(&mut ctrl).unwrap();
        println!("Hello this: {}", ctrl);
        if ctrl.trim() == "p" {
            if sink.is_paused() {
                sink.play();
            } else {
                sink.pause();
            }
        }  else if ctrl.trim() == "x" {
            break;
        }

        ctrl.clear();
    }
}
