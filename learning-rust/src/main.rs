mod read_file;

// use rodio::Source;
// use std::fs::File;
// use std::io::BufReader;
use std::process::Command;

use futures::executor::block_on;

async fn run_command(command : &str) {
    let _ = Command::new(command)
        .spawn()
        .expect("failed to execute process");
}

async fn download_video() {
    let _ = Command::new("./yt-dlp.exe")
        .arg("-x")
        .arg("--audio-format")
        .arg("wav")
        .arg("https://www.youtube.com/watch?v=SMGb1uawIW0&list=PL6han0RP73M6jULp1So_fJm1PAxmojQ21")
        .arg("-o")
        .arg("samples/%(title)s.%(ext)s")
        .spawn()
        .expect("failed to execute process");
}

fn main() {
    println!("starting main function");

    // create a loop that will print numbers to 1000
    // let mut counter = 0;
    // loop {
    //     counter += 1;
    //     println!("counter is {}", counter);
    //     if counter == 1000 {
    //         break;
    //     }
    // }

    let mut names = vec!["Bob", "Frank", "Ferris"];

    for name in names.iter_mut() {
        if name == &mut "Bob" {
            println!("Hello BOB GAMING {}", name);
        } else {
            println!("Hello {}", name);
        }
    }

    read_file::read_file("assets/test.txt");

    // let (stream, stream_handle) = rodio::OutputStream::try_default().unwrap();

    // // Load a sound from a file, using a path relative to Cargo.toml
    // let file = File::open("them_chords.mp3").unwrap();
    // let source = rodio::Decoder::new(BufReader::new(file)).unwrap();
    // let _ = stream_handle.play_raw(source.convert_samples());

    // // The sound plays in a separate audio thread,
    // // so we need to keep the main thread alive while it's playing.
    // // Press ctrl + C to stop the process once you're done.
    // loop {
    //     // get user input with a > prompt
    //     let mut input = String::new();
    //     std::io::stdin().read_line(&mut input).unwrap();

    //     // remove the newline character from the input
    //     input = input.trim().to_string();

    //     // if the user input is "quit", then exit the program
    //     if input == "quit" {
    //         break;
    //     }
    // }

    // https://www.youtube.com/watch?v=kSdYNWJ3OwU
    let future = run_command("ls");
    block_on(future);

    let future2 = download_video(); 
    block_on(future2);
    // std::process::exit(0);
}
