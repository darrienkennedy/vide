use std::process::Command;

pub struct Editor {
    bin: String,
}

impl Editor {
    pub fn new() -> Editor {
        Editor {
            bin: "/usr/bin/vim".to_string(),
        }
    }

    pub fn edit(&self, file: String) {
        let mut proc_edit = Command::new(&self.bin).arg(file).spawn().unwrap();
        proc_edit.wait().unwrap();
    }

    pub fn read(&self, file: String) {
        let mut proc_read = Command::new(&self.bin).arg("-R").arg(file).spawn().unwrap();
        proc_read.wait().unwrap();
    }
}
