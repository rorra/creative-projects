use clap::Parser;
use std::fs::File;
use std::io::Read;
use toml;
use serde_derive::Deserialize;

#[derive(Parser, Debug)]
struct Args {
    #[arg(long)]
    config_path: Option<std::path::PathBuf>,
}

#[allow(unused)]
#[derive(Deserialize)]
struct Config {
    input: Input,
    redis: Redis,
    sqlite: SQLite,
    postgresql: Postgresql,
}

#[allow(unused)]
#[derive(Deserialize)]
struct Input {
    xml_file: String,
    json_file: String,
}

#[allow(unused)]
#[derive(Deserialize)]
struct Redis {
    host: String,
}

#[allow(unused)]
#[derive(Deserialize)]
struct SQLite {
    db_file: String,
}

#[allow(unused)]
#[derive(Deserialize)]
struct Postgresql {
    username: String,
    password: String,
    host: String,
    port: String,
    database: String,
}


fn main() {
    let args = Args::parse();

    let config_path = match args.config_path {
        Some(config_path) => config_path,
        None => {
            println!("No config path provided. Use --config-path to specify a config");
            return;
        }
    };


    let config: Config = read_toml_dynamically(config_path).unwrap();
    //println!("{:?}", config);

    println!("[Postgresql].Database: {}", config.postgresql.database);
}

fn read_toml_dynamically(config_path: std::path::PathBuf) -> Result<Config, String> {
    let file_path = config_path.join("config.toml");

    let mut file = File::open(file_path).map_err(|e| e.to_string())?;
    let mut contents = String::new();
    file.read_to_string(&mut contents).map_err(|e| e.to_string())?;
    toml::from_str(&contents).map_err(|e| e.to_string())
}

