use itertools::Itertools;
use std::io::Write;
use std::process::{Command, Stdio};

fn main() {
    let other_inputs = vec![
        "Public speaking is very easy.\n",
        "1 2 6 24 120 720\n",
        "1 b 214\n",
        "9\n",
        "opekma\n",
    ];

    for combo in (0..=9).permutations(6) {
        let last_input = combo.iter().join(" ") + "\n";
        let mut child = Command::new("./bomb")
            .stdin(Stdio::piped())
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .spawn()
            .unwrap();

        let mut stdin = child.stdin.take().unwrap();
        for input in &other_inputs {
            stdin.write_all(input.as_bytes()).unwrap()
        }
        stdin.write_all(last_input.as_bytes()).unwrap();

        let output = child.wait_with_output().unwrap();
        if output.status.code() != Some(8) {
            println!("Found valid combination by exit code: {}", last_input);
            break;
        }
    }
    println!("Done, didn't find a valid combination.");
}
