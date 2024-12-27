use toml_edit::DocumentMut;

use crate::utils;

#[derive(clap::Args, Debug)]
pub struct Args {}

pub fn run(_args: Args) {
    let project_root = utils::project_root();

    let cargo_toml = std::fs::read_to_string(project_root.join("Cargo.toml")).unwrap();
    let mut doc = cargo_toml.parse::<DocumentMut>().unwrap();

    let version = doc["workspace"]["package"]["version"]
        .clone()
        .into_value()
        .unwrap();
    doc["workspace"]["dependencies"]["serde_valid_derive"]["version"] = version.clone().into();
    doc["workspace"]["dependencies"]["serde_valid_literal"]["version"] = version.into();

    std::fs::write(project_root.join("Cargo.toml"), doc.to_string()).unwrap();
}
