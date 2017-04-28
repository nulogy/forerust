extern crate futures;
extern crate tokio_core;
extern crate tokio_process;
extern crate tokio_io;

use std::io;
use std::process::{Command, Stdio};

use futures::{BoxFuture, Future, Stream, future};
use tokio_core::reactor::Core;
use tokio_process::{CommandExt, Child};

struct ForerustProcess {
    pub name: String,
    command: String,
}

impl ForerustProcess {
    fn to_command(&self) -> Command {
        let mut cmd = Command::new(self.command.clone());
        cmd.stdout(Stdio::piped()).stderr(Stdio::piped());
        cmd
    }
}

fn get_lines(prefix: String, mut cmd: Child) -> BoxFuture<(), io::Error> {
    let stdout = cmd.stdout().take().unwrap();
    let reader = io::BufReader::new(stdout);
    let lines = tokio_io::io::lines(reader);
    let cycle = lines.for_each(move |l| {
        println!("{}| {}", prefix, l);
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
    let linegetters = processes.iter().map(|f_p| get_lines(f_p.name.clone(), f_p.to_command().spawn_async(&handle).unwrap()));
    let combined = future::select_all(linegetters).map_err(|e| e.1);

    core.run(combined).unwrap();
}
