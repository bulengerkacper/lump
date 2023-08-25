use std::io::{BufRead, BufReader,Read};
use std::process::{Command, Stdio};
use std::thread;

struct command {}

pub fn collect_data() {
    let thread_handle = thread::spawn(|| {
        let output = Command::new("pstree")
            .output()
            .expect("Failed to execute command");

        let pstree_output = String::from_utf8_lossy(&output.stdout);

        pstree_output.to_string()
    });

    // Wait for the thread to complete and get the result
    let pstree_output = thread_handle.join().unwrap();

    println!("pstree output:\n{}", pstree_output);
}
