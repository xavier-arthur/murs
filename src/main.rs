mod opts;

use murs::TerminalOutput::{*, self};
use structopt::StructOpt;
use std::{
    io::stdin,

    process::{Command, ExitCode},
    process::Output
};


fn read_stdin() -> Vec<String> {
    let stdin = stdin().lines();
    let mut input: Vec<String> = Vec::new();

    for line in stdin {
        let line = line.unwrap();
        input.push(line);
    }

    input
}

fn print_output(output: Output, from: TerminalOutput) {
    let show_output = |s: Vec<u8>| {
        for char in s {
            print!("{}", char as char);
        }
    };

    match from {
        STDERR => show_output(output.stderr),
        ALL | STDOUT => show_output(output.stdout)
    }
}

fn main() -> ExitCode {
    let args = opts::Opt::from_args();

    let input: Vec<String> = if let Some(v) = args.input {
        vec![v]
    } else {
        read_stdin()
    };

    let bin_path = match args.ytdl_bin_file {
        Some(v) => v,
        None => String::from("youtube-dl")
    };

    let input_size = input.len();
    let input_no_comments = input.into_iter().filter(|v| !v.starts_with("#"));

    input_no_comments.enumerate().for_each(|(i, v)| {
        println!("{} of {} | downloading: {}. . . ", i + 1, input_size ,&v);

        let output = Command::new(&bin_path)
            .arg("-x")
            .arg("--audio-format")
            .arg("mp3")
            .arg(&v)
            .current_dir(&args.output)
            .output()
        .expect(&format!("could not download {}", &v));

        if output.stderr.len() > 0 {
            print_output(output, STDERR);
        } else if args.verbose {
            print_output(output, ALL);
        }
    });

    ExitCode::SUCCESS
}
