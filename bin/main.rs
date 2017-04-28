extern crate dotenv;
extern crate forerust;
extern crate futures;
extern crate tokio_core;
extern crate tokio_process;
extern crate tokio_io;
extern crate chrono;

extern crate ansi_term;
use ansi_term::Colour::*;

use futures::{BoxFuture, Future, Stream, future};

use std::fs::File;
use std::io;
use std::io::Read;
use tokio_core::reactor::Core;
use tokio_process::{CommandExt, Child};

use forerust::{ ForerustProcess, parse_procfile };

const PREFIX_COLOURS: [ansi_term::Color; 6] = [Cyan, Yellow, Green, Purple, Red, Blue];

fn get_lines_1<T: tokio_io::AsyncRead + Send + 'static>(colour: ansi_term::Colour, pad_size: usize, command_name: String, iostream: T) -> BoxFuture<(), io::Error> {
    let reader = io::BufReader::new(iostream);
    let lines = tokio_io::io::lines(reader);
    let cycle = lines.for_each(move |l| {
        let now = chrono::Local::now();
        let prefix = colour.paint(format!("{} {: >pad_size$} | ", now.format("%H:%M:%S"), command_name, pad_size = pad_size));
        println!("{}{}", prefix, l);
        Ok(())
    });
    cycle.boxed()
}

fn get_lines(colour: ansi_term::Colour, pad_size: usize, command_name: String, mut cmd: Child) -> BoxFuture<(), io::Error> {
    let stdout = cmd.stdout().take().unwrap();
    let stderr = cmd.stderr().take().unwrap();

    let cycle_stdout = get_lines_1(colour, pad_size, command_name.clone(), stdout);
    let cycle_stderr = get_lines_1(colour, pad_size, command_name.clone(), stderr);
    let cycle = cycle_stdout.join(cycle_stderr);

    cycle.join(cmd.wait_with_output()).map(|_| ()).boxed()
}

fn longest_command_length(processes: &Vec<ForerustProcess>) -> usize {
    processes.iter().map(|p| p.name.len()).max().unwrap()
}

fn main() {
    dotenv::dotenv().ok();

    let mut procfile = File::open("Procfile").unwrap();
    let mut procfile_contents = String::new();

    procfile.read_to_string(&mut procfile_contents);

    let processes = parse_procfile(procfile_contents);

    let pad_size = longest_command_length(&processes);
    let mut core = Core::new().unwrap();
    let handle = core.handle();
    let linegetters = processes.iter().enumerate().map(|(index, f_p)| {
        let colour = PREFIX_COLOURS[index % PREFIX_COLOURS.len()];
        get_lines(colour, pad_size, f_p.name.clone(), f_p.to_command().spawn_async(&handle).unwrap())
    });
    let combined = future::select_all(linegetters).map_err(|e| e.1);

    core.run(combined).unwrap();
}
