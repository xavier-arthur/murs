use std::{
    io::stdin,

    process::Command,
    process::Output,
};

use murs::get_folder;

fn read_stdin() -> Vec<String> {
    let stdin = stdin().lines();
    let mut input: Vec<String> = Vec::new();

    for line in stdin {
        let line = line.unwrap();
        input.push(line);
    }

    input
}

fn _print_output(output: Output) -> () {
    for char in output.stdout {
        print!("{}", char as char);
    }
}

fn main() {

    let input = read_stdin();
    let input_size = input.len();

    let output_folder = if let Some(v) = get_folder() {
        v
    } else {
        String::from(".")
    };

    let input_no_comments = input.into_iter().filter(|v| !v.starts_with("#"));


    input_no_comments.enumerate().for_each(|(i, v)| {
        println!("{} of {} | downloading: {}. . . ", i + 1, input_size ,&v);

        Command::new("youtube-dl")
            .arg("-x")
            .arg("--audio-format")
            .arg("mp3")
            .arg(&v)
            .current_dir(&output_folder)
            .output()
        .expect(&format!("could not download {}", &v));
    });
}
