use clap::Parser;
use serde::{Deserialize, Serialize};
use rusqlite::{Connection, params, Result};

#[derive(Parser, Debug)]
struct Args {
    #[arg(long)]
    json_file: Option<std::path::PathBuf>,
}


#[derive(Clone, Deserialize, Serialize, Debug)]
struct Product {
    id: u32,
    category: String,
    name: String,
}

#[derive(Clone, Deserialize, Serialize, Debug)]
struct Sale {
    id: String,
    product_id: u32,
    date: i64,
    quantity: f64,
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

    let mut conn = open_my_db().unwrap();
    populate_db(&mut conn, &json).unwrap();
    print_db(&mut conn).unwrap();
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


fn open_my_db() -> Result<Connection> {
    let path = "./my_db.db3";
    let conn = Connection::open(path)?;

    conn.execute("DROP TABLE IF EXISTS sales", [])?;
    conn.execute("DROP TABLE IF EXISTS products", [])?;

    conn.execute(
        "CREATE TABLE products (
                  id              INTEGER PRIMARY KEY,
                  category        TEXT NOT NULL,
                  name            TEXT NOT NULL
                  )",
        [],
    )?;

    conn.execute(
        "CREATE TABLE sales (
                  id              TEXT PRIMARY KEY,
                  product_id      INTEGER NOT NULL,
                  date            INTEGER NOT NULL,
                  quantity        REAL NOT NULL,
                  unit            TEXT NOT NULL,
                  FOREIGN KEY(product_id) REFERENCES products(id)
                  )",
        [],
    )?;

    Ok(conn)
}

fn populate_db(conn: &mut Connection, json: &SalesAndProducts) -> Result<()> {
    let tx = conn.transaction()?;

    for ref product in json.products.iter() {
        tx.execute(
            "INSERT INTO products (id, category, name) VALUES (?1, ?2, ?3)",
            params![product.id, product.category, product.name],
        )?;
    }

    for ref sale in json.sales.iter() {
        tx.execute(
            "INSERT INTO sales (id, product_id, date, quantity, unit) VALUES (?1, ?2, ?3, ?4, ?5)",
            params![
                sale.id,
                sale.product_id,
                sale.date,
                sale.quantity,
                sale.unit
            ],
        )?;
    }

    tx.commit()?;
    Ok(())
}

fn print_db(conn: &mut Connection) -> Result<()> {
    let mut stmt = conn.prepare("SELECT id, category, name FROM products")?;
    let product_iter = stmt.query_map([], |row| {
        Ok(Product {
            id: row.get(0)?,
            category: row.get(1)?,
            name: row.get(2)?,
        })
    })?;

    for product in product_iter {
        println!("Found product {:?}", product.unwrap());
    }

    let mut stmt = conn.prepare("SELECT id, product_id, date, quantity, unit FROM sales")?;
    let sale_iter = stmt.query_map([], |row| {
        Ok(Sale {
            id: row.get(0)?,
            product_id: row.get(1)?,
            date: row.get(2)?,
            quantity: row.get(3)?,
            unit: row.get(4)?,
        })
    })?;

    for sale in sale_iter {
        println!("Found sale {:?}", sale.unwrap());
    }

    Ok(())
}
