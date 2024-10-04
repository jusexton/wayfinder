use std::{collections::HashMap, fs::File, io::BufReader, path::PathBuf};

use clap::{Args, Parser};

mod path;

enum InputOptions {
    RawJson(String),
    JsonFile(PathBuf),
}

#[derive(Args, Debug)]
#[group(required = true, multiple = false)]
struct Input {
    #[arg(long)]
    raw_json: Option<String>,

    #[arg(long)]
    json_file: Option<PathBuf>,
}

impl Input {
    fn into_options(self) -> InputOptions {
        if let Some(json) = self.raw_json {
            InputOptions::RawJson(json)
        } else {
            InputOptions::JsonFile(self.json_file.unwrap())
        }
    }
}

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Cli {
    #[command(flatten)]
    input: Input,

    #[arg(short, long = "target", required = true)]
    targets: Vec<String>,
}

fn print_paths(paths: HashMap<String, Vec<path::Path>>) {
    paths
        .into_iter()
        .enumerate()
        .for_each(|(idx, (key, value))| {
            let prefix = if idx > 0 { "\n" } else { "" };
            println!("{prefix}Paths to '{key}':");
            for path in value.iter().rev() {
                println!("  {}", path);
            }
        });
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Cli::parse();
    let paths = match args.input.into_options() {
        InputOptions::RawJson(json) => {
            path::find_paths(&mut serde_json::Deserializer::from_str(&json), args.targets)?
        }
        InputOptions::JsonFile(path) => {
            let f = File::open(path)?;
            let buf = BufReader::new(f);
            path::find_paths(
                &mut serde_json::Deserializer::from_reader(buf),
                args.targets,
            )?
        }
    };
    print_paths(paths);
    Ok(())
}
