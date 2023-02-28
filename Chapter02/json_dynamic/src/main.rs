use clap::Parser;

#[derive(Parser, Debug)]
struct Args {
    #[arg(long)]
    input_path: Option<std::path::PathBuf>,

    #[arg(long)]
    output_path: Option<std::path::PathBuf>,
}

fn main() {
    let args = Args::parse();

    let input_path = get_input_file(args.input_path, "input-path")
        .unwrap();
    let output_path = get_input_file(args.output_path, "output-path")
        .unwrap();

    let mut json = read_json(input_path).unwrap();

    if let serde_json::Value::Number(n) = &json["sales"][1]["quantity"] {
        let new_value = n.as_f64().unwrap() + 1.5;
        json["sales"][1]["quantity"] = serde_json::Value::Number(serde_json::Number::from_f64(new_value).unwrap());
    } else {
        panic!("sales doesn't have two values items or quantity is not a number");
    }

    std::fs::write(
        output_path,
        serde_json::to_string_pretty(&json).unwrap()
    ).expect("Unable to write file");
}

fn get_input_file(
    option: Option<std::path::PathBuf>,
    arg_name: &str
) -> Result<std::path::PathBuf, String> {
    match option {
        Some(path) => {
            if arg_name == "output-path" {
                return Ok(path);
            }

            if path.exists() {
                Ok(path)
            } else {
                Err(format!("{} does not exist", arg_name))
            }
        }
        None => Err(format!("{} is required", arg_name)),
    }
}

fn read_json(input_path: std::path::PathBuf) -> Result<serde_json::Value, String> {
    let file = std::fs::File::open(input_path).map_err(|e| e.to_string())?;
    let reader = std::io::BufReader::new(file);
    let json: serde_json::Value = serde_json::from_reader(reader).map_err(|e| e.to_string())?;
    Ok(json)
}