extern crate serde;

use serde::{ Deserialize, Deserializer };
use serde::de;
use serde::de::Visitor;

use std::fmt;
use std::fmt::Display;
use std::result::Result;
use std::process::{Command, Stdio};

type ForerustProcfile = Vec<ForerustProcess>;

#[derive(Eq, PartialEq)]
pub struct ForerustProcess {
    pub name: String,
    pub command: String,
}

impl ForerustProcess {
    pub fn to_command(&self) -> Command {
        let mut cmd = Command::new("/bin/sh");
        cmd.arg("-c").arg(self.command.clone());
        cmd.stdout(Stdio::piped()).stderr(Stdio::piped());
        cmd
    }
}

pub fn parse_procfile(procfile_as_string: String) -> ForerustProcfile {
    procfile_as_string.lines().map(|line| {
        let process: Vec<String> = line.split(": ").map(|slice| slice.to_string()).map(|slice| slice.to_string()).collect();
        ForerustProcess {
            name: process[0].clone(),
            command: process[1].clone()
        }
    }).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_can_parse_a_procfile_into_a_forerust_process() {
        let expected_forerust_process = ForerustProcess {
            name: String::from("hello"),
            command: String::from("echo 'hello world!'")
        };
        let procfile_as_string = String::from("hello: echo 'hello world!'");

        let forerust_processes = parse_procfile(procfile_as_string);

        assert!(forerust_processes.len() == 1);
        assert!(forerust_processes[0] == expected_forerust_process);
    }
}
