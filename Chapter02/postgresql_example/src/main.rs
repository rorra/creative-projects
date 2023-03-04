use clap::Parser;
use serde::{Deserialize, Serialize};
use postgres::{Client, NoTls};

#[derive(Parser, Debug)]
struct Args {
    #[arg(long)]
    json_file: Option<std::path::PathBuf>,
}


#[derive(Clone, Deserialize, Serialize, Debug)]
struct Product {
    id: i32,
    category: String,
    name: String,
}

#[derive(Clone, Deserialize, Serialize, Debug)]
struct Sale {
    id: String,
    product_id: i32,
    date: i64,
    quantity: f32,
    unit: String,
}

#[derive(Clone, Deserialize, Serialize, Debug)]
struct SalesAndProducts {
    products: Vec<Product>,
    sales: Vec<Sale>,
}


fn main() {
    let args = Args::parse();

    // Read the json file with the data
    let input_path = get_input_file(args.json_file, "json-file").unwrap();
    let json = read_json(input_path).unwrap();

    let mut client = open_my_db().unwrap();
    populate_db(&mut client, &json).unwrap();
    print_db(&mut client).unwrap();
}

fn get_input_file(
    option: Option<std::path::PathBuf>,
    arg_name: &str,
) -> Result<std::path::PathBuf, String> {
    match option {
        Some(path) => {
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


fn open_my_db() -> Result<Client, postgres::Error> {
    let mut client = Client::connect(
        "postgresql://rorra:password@localhost/rust2018",
        NoTls
    )?;

    client.execute("DROP TABLE IF EXISTS sales", &[])?;
    client.execute("DROP TABLE IF EXISTS products", &[])?;

    client.execute(
        "CREATE TABLE products (
                  id              INTEGER PRIMARY KEY,
                  category        VARCHAR(20) NOT NULL,
                  name            VARCHAR(20) NOT NULL
                  )",
        &[],
    )?;

    client.execute(
        "CREATE TABLE sales (
                  id              VARCHAR(20) PRIMARY KEY,
                  product_id      INTEGER NOT NULL,
                  date            BIGINT NOT NULL,
                  quantity        REAL NOT NULL,
                  unit            VARCHAR(10) NOT NULL,
                  FOREIGN KEY(product_id) REFERENCES products(id)
                  )",
        &[],
    )?;

    Ok(client)
}

fn populate_db(client: &mut Client, json: &SalesAndProducts) -> Result<(), postgres::Error> {
    let mut tx = client.transaction()?;

    for ref product in json.products.iter() {
        tx.execute(
            "INSERT INTO products (id, category, name) VALUES ($1, $2, $3)",
            &[&product.id, &product.category, &product.name],
        )?;
    }

    for ref sale in json.sales.iter() {
        tx.execute(
            "INSERT INTO sales (id, product_id, date, quantity, unit) VALUES ($1, $2, $3, $4, $5)",
            &[
                &sale.id,
                &sale.product_id,
                &sale.date,
                &sale.quantity,
                &sale.unit
            ],
        )?;
    }

    tx.commit()?;
    Ok(())
}

fn print_db(client: &mut Client) -> Result<(), postgres::Error> {
    for row in client.query("SELECT id, category, name FROM products", &[])? {
        let product = Product {
            id: row.get(0),
            category: row.get(1),
            name: row.get(2),
        };
        println!("Found product {:?}", product);
    }

    for row in client.query("SELECT id, product_id, date, quantity, unit FROM sales", &[])? {
        let sale = Sale {
            id: row.get(0),
            product_id: row.get(1),
            date: row.get(2),
            quantity: row.get(3),
            unit: row.get(4),
        };
        println!("Found sale {:?}", sale);
    }

    Ok(())
}
