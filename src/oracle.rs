use std::process::Command;
use std::path::Path;

pub fn run_js_parser(input: &str) -> String {
    let js_parser_path = Path::new("src-js/ilmentufa/run_camxes.js");
    let output = Command::new("node")
        .arg(js_parser_path)
        .arg("-std") // -std parser ID
        .arg("-m")
        .arg("R") // Raw output
        .arg(input)
        .output()
        .expect("Failed to execute JS parser");

    String::from_utf8_lossy(&output.stdout).trim().to_string()
}
