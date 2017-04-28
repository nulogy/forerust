use std::process::{Command, Stdio};

#[derive(Eq, PartialEq)]
pub struct ForerustProcess {
    pub name: String,
    pub command: String,
}

impl ForerustProcess {
    pub fn to_command(&self) -> Command {
        let mut cmd = Command::new(self.command.clone());
        cmd.stdout(Stdio::piped()).stderr(Stdio::piped());
        cmd
    }
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
