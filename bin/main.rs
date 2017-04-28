extern crate futures;
extern crate tokio_core;
extern crate tokio_process;
extern crate tokio_io;
extern crate chrono;

extern crate ansi_term;
use ansi_term::Colour::*;

use std::io;
use std::process::{Command, Stdio};

use futures::{BoxFuture, Future, Stream, future};
use tokio_core::reactor::Core;
use tokio_process::{CommandExt, Child};

struct ForerustProcess {
    pub name: String,
    command: String,
}

const PREFIX_COLOURS: [ansi_term::Color; 6] = [Cyan, Yellow, Green, Purple, Red, Blue];

impl ForerustProcess {
    fn to_command(&self) -> Command {
        let mut cmd = Command::new(self.command.clone());
        cmd.stdout(Stdio::piped()).stderr(Stdio::piped());
        cmd
    }
}

fn get_lines(colour: ansi_term::Colour, prefix: String, mut cmd: Child) -> BoxFuture<(), io::Error> {
    let stdout = cmd.stdout().take().unwrap();
    let reader = io::BufReader::new(stdout);
    let lines = tokio_io::io::lines(reader);
    let colored_prefix = colour.paint(prefix);
    let cycle = lines.for_each(move |l| {
        let now = chrono::Local::now();
        let colored_time = colour.paint(format!("{}", now.format("%H:%M:%S")));
        println!("{} {}| {}", colored_time, colored_prefix, l);
        Ok(())
    });
    cycle.join(cmd.wait_with_output()).map(|_| ()).boxed()
}

fn main() {

    let processes = vec![
        ForerustProcess{ name: String::from("Hello"), command: String::from("./test1.rb") },
        ForerustProcess{ name: String::from("World"), command: String::from("./test2.rb") }
    ];

    let mut core = Core::new().unwrap();
    let handle = core.handle();
    let linegetters = processes.iter().enumerate().map(|(index, f_p)| {
        let colour = PREFIX_COLOURS[index % PREFIX_COLOURS.len()];
        get_lines(colour, f_p.name.clone(), f_p.to_command().spawn_async(&handle).unwrap())
    });
    let combined = future::select_all(linegetters).map_err(|e| e.1);

    core.run(combined).unwrap();
}
