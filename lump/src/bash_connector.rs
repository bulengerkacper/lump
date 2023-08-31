use std::process::Command;
use std::{thread, time::Duration};

pub struct Cache {
    pub content: Vec<(String, String,String,String)>,
}

impl Cache {
    pub fn check_pstree_state(&mut self) -> bool {
        let output = Command::new("pstree")
            .arg("-p")
            .output()
            .expect("Failed to execute command");
        let output_ps = Command::new("ps")
            .arg("aux")
            .arg("--sort")
            .arg("-pcpu")
            .output()
            .expect("Failed to execute command");
        let pstree_output = String::from_utf8_lossy(&output.stdout);
        let ps_output = String::from_utf8_lossy(&output_ps.stdout);        
        let out: Vec<(String, String,String,String)> = parse_ps_output(ps_output.to_string());
        if self.content != out {
            self.content = out;
            return true;
        }
        return false;
        //let out: Vec<(String, String)> = parse_pstree(pstree_output.to_string());
        // if self.content != out {
        //     self.content = out;
        //     return true;
        // }
        // return false;
    }

    pub fn collect_data(&mut self) -> bool {
        loop {
            if self.check_pstree_state() {
                return true;
            }
        }
    }
}

fn parse_pstree(output: String) -> Vec<(String, String)> {
    let lines: Vec<&str> = output.lines().collect();
    let mut results: Vec<(String, String)> = Vec::new();
    for line in lines {
        let parts: Vec<&str> = line.split('(').collect();

        if parts.len() >= 2 {
            let process_name = parts[0];
            let trimmed = process_name
                .replace("|", "")
                .replace("-", "")
                .replace("{", "")
                .replace("}", "")
                .replace("`", "");
            let pid_part = parts[1].split(')').next().unwrap_or_default();
            let process_pid = pid_part.trim_end_matches(')');
            results.push((trimmed.to_string(), process_pid.to_string()));
        }
    }

    results
}

fn parse_ps_output(ps_output: String) -> Vec<(String, String, String, String)> {
    let mut processes = Vec::new();

    for line in ps_output.lines().skip(1) {
        let fields: Vec<&str> = line.split_whitespace().collect();

        if fields.len() >= 11 {
            let proc_name = fields[10].to_string();
            let pid = fields[1].to_string();
            let cpu = fields[2].to_string();
            let mem = fields[3].to_string();
            println!("{} {} {} {}",proc_name,pid,cpu,mem);
            processes.push((proc_name, pid, cpu, mem));
        }
    }

    processes
}
