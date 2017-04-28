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
    procfile_as_string
        .lines()
        .filter(|line| {
            match line.chars().next() {
                None => false,
                Some(c) => c != '#'
            }
        })
        .map(|line| {
            let process: Vec<String> = line.split(": ").map(|slice| slice.to_string()).map(|slice| slice.to_string()).collect();
            ForerustProcess {
                name: process[0].clone(),
                command: process[1].clone()
            }
        })
        .collect()
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

    #[test]
    fn it_ignores_comments() {
        let procfile_as_string = String::from("# this procfile is empty");

        let forerust_processes = parse_procfile(procfile_as_string);

        assert!(forerust_processes.len() == 0);
    }
}
