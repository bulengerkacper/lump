use std::io::{BufRead, BufReader, Read};
use std::process::{Command, Stdio};
use std::thread;

pub struct Cache {
    content: String,
}

impl Cache {
    pub fn create_cache(&mut self) {

        let output = Command::new("pstree")
            .output()
            .expect("Failed to execute command");

        let pstree_output = String::from_utf8_lossy(&output.stdout);
        self.content = pstree_output.to_string();
    }

    pub fn check_pstree_state(&mut self) -> bool {
        let output = Command::new("pstree").output().expect("Failed to execute command");
        let pstree_output = String::from_utf8_lossy(&output.stdout);
        if pstree_output != self.content {
            self.content=pstree_output.to_string();
            println!("pstree changed");
            return true
        }
            return false
    }

    pub fn collect_data(&mut self) {
        self.check_pstree_state();
    }
}
