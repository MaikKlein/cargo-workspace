extern crate rayon;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate toml;
use std::fs::File;
use std::env;
use std::process::Command;
use std::io::{Write, Read};
use rayon::prelude::*;

#[derive(Debug, Deserialize, Serialize)]
struct Outer {
    workspace: Workspace,
}
#[derive(Debug, Deserialize, Serialize)]
struct Workspace {
    members: Vec<String>,
}

fn main() {
    let args: Vec<_> = env::args().collect();
    assert!(args.len() >= 2, "[Bug]: Incorrect number of args");
    let args_to_skip = if args[1] == "workspace" { 2 } else { 1 };
    let mut file = File::open("Cargo.toml").expect("Unable to find Cargo.toml");
    let mut string = String::new();
    file.read_to_string(&mut string).expect(
        "Unable to read file to string",
    );
    let workspace: Outer = toml::from_str(&string).expect("Is not a workspace");
    let workspace = workspace.workspace;
    let outputs: Vec<_> = workspace
        .members
        .par_iter()
        .map(|member| {
            let output = Command::new("cargo")
                .current_dir(member)
                .args(&args[args_to_skip..])
                .output();
            output.unwrap()
        })
        .collect();
    let stdout: Vec<u8> = outputs
        .iter()
        .flat_map(|o| o.stdout.iter().cloned())
        .collect();
    let stderr: Vec<u8> = outputs
        .iter()
        .flat_map(|o| o.stderr.iter().cloned())
        .collect();
    println!(
        "{}",
        std::str::from_utf8(&stdout).expect("stdout could not be converted to utf8")
    );
    writeln!(
        &mut std::io::stderr(),
        "{}",
        std::str::from_utf8(&stderr).expect("stderr could not be converted to utf8")
    ).expect("Unable to write to stderr");
}
