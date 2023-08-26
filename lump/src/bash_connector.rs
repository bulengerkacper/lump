use std::process::Command;

pub struct Cache {
    pub content: String,
}
impl Cache {
    pub fn check_pstree_state(&mut self) -> bool {
        let output = Command::new("pstree")
            .output()
            .expect("Failed to execute command");
        let pstree_output = String::from_utf8_lossy(&output.stdout);
        if pstree_output != self.content {
            self.content = pstree_output.to_string();
            println!("pstree changed");
            return true;
        }
        return false;
    }

    pub fn collect_data(&mut self) {
        loop {
            if self.check_pstree_state() {}
        }
    }
}
