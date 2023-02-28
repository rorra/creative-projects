use clap::Parser;
use serde::{Deserialize, Serialize};

#[derive(Parser, Debug)]
struct Args {
    #[arg(long)]
    input_path: Option<std::path::PathBuf>,

    #[arg(long)]
    output_path: Option<std::path::PathBuf>,
}

#[derive(Deserialize, Serialize, Debug)]
struct SalesAndProducts {
    products: Vec<Product>,
    sales: Vec<Sale>,
}

#[derive(Deserialize, Serialize, Debug)]
struct Product {
    id: u32,
    category: String,
    name: String,
}

#[derive(Deserialize, Serialize, Debug)]
struct Sale {
    id: String,
    product_id: u32,
    date: u64,
    quantity: f64,
    unit: String,
}

fn main() {
    let args = Args::parse();

    let input_path = get_input_file(args.input_path, "input-path")
        .unwrap();
    let output_path = get_input_file(args.output_path, "output-path")
        .unwrap();

    let mut json = read_json(input_path).unwrap();
    json.sales[1].quantity += 1.5;

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

fn read_json(input_path: std::path::PathBuf) -> Result<SalesAndProducts, String> {
    let file = std::fs::File::open(input_path).map_err(|e| e.to_string())?;
    let reader = std::io::BufReader::new(file);
    let json: SalesAndProducts = serde_json::from_reader(reader).map_err(|e| e.to_string())?;
    Ok(json)
}