use clap::Parser;
use std::fs::File;
use std::io::Read;
use toml;
use toml::Table;

#[derive(Parser, Debug)]
struct Args {
    #[arg(long)]
    config_path: Option<std::path::PathBuf>,
}

fn main() {
    let args = Args::parse();
    let content: Table;

    let config_path = match args.config_path {
        Some(config_path) => config_path,
        None => {
            println!("No config path provided. Use --config-path to specify a config");
            return;
        }
    };


    content = read_toml_dynamically(config_path).unwrap();
    println!("{:?}", content);

    println!(
        "[Postgresql].Database: {}",
        content
            .get("postgresql")
            .unwrap()
            .get("database")
            .unwrap()
            .as_str()
            .unwrap()
    );
}

fn read_toml_dynamically(config_path: std::path::PathBuf) -> Result<Table, String> {
    let file_path = config_path.join("config.toml");

    let mut file = File::open(file_path).map_err(|e| e.to_string())?;
    let mut contents = String::new();
    file.read_to_string(&mut contents).map_err(|e| e.to_string())?;
    contents.parse::<Table>().map_err(|e| e.to_string())
}

