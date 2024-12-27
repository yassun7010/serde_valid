mod commands;
mod utils;

use clap::Parser;

#[derive(Debug, clap::Parser)]
enum Args {
    UpdateTags(commands::update_tags::Args),
}

fn main() {
    let args = Args::parse();
    match args {
        Args::UpdateTags(args) => {
            commands::update_tags::run(args);
        }
    }
}
