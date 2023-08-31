use std::process::Command;

pub struct Cache {
    pub content: Vec<(String, String, String, String)>,
}

impl Cache {
    pub fn check_pstree_state(&mut self) -> bool {
        let output_ps = Command::new("ps")
            .arg("aux")
            .arg("--sort")
            .arg("-pcpu")
            .output()
            .expect("Failed to execute command");
        let ps_output = String::from_utf8_lossy(&output_ps.stdout);
        let out: Vec<(String, String, String, String)> =
            self.parse_ps_output(ps_output.to_string());
        if self.content != out {
            self.content = out;
            return true;
        }
        return false;
    }

    pub fn collect_data(&mut self) -> bool {
        loop {
            if self.check_pstree_state() {
                return true;
            }
        }
    }
    fn parse_ps_output(&self, ps_output: String) -> Vec<(String, String, String, String)> {
        let mut processes = Vec::new();

        for line in ps_output.lines().skip(1) {
            let fields: Vec<&str> = line.split_whitespace().collect();

            if fields.len() >= 11 {
                let proc_name = fields[10].to_string();
                let pid = fields[1].to_string();
                let cpu = fields[2].to_string();
                let mem = fields[3].to_string();
                if let Some(last) = self.get_last_segment_after_last_slash(&proc_name) {
                    if last.chars().any(|c| c.is_alphabetic()) {
                        processes.push((last, pid, cpu, mem));
                    } else {
                        processes.push((proc_name, pid, cpu, mem));
                    }
                } else {
                    processes.push((proc_name, pid, cpu, mem));
                }
            }
        }

        processes
    }

    fn get_last_segment_after_last_slash(&self, input: &String) -> Option<String> {
        let segments: Vec<&str> = input.split('/').collect();
        segments.last().map(|segment| segment.to_string())
    }
}