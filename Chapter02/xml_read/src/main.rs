use clap::Parser;
use std::fs::File;
use std::io::BufReader;
use xml::reader::{EventReader, XmlEvent};


#[derive(Parser, Debug)]
struct Args {
    #[arg(long)]
    xml_file: Option<std::path::PathBuf>,
}

#[derive(Debug, Default, Clone)]
struct Product {
    id: u32,
    category: String,
    name: String,
}

#[derive(Debug, Default, Clone)]
struct Sale {
    id: String,
    product_id: u32,
    date: i64,
    quantity: f64,
    unit: String,
}

enum ParseState {
    Product,
    Sale,
    None,
}

enum ParseProduct {
    Id,
    Category,
    Name,
    None,
}

enum ParseSale {
    Id,
    ProductId,
    Date,
    Quantity,
    Unit,
    None,
}

fn main() {
    let mut products = vec![];
    let mut sales = vec![];

    let args = Args::parse();
    let xml_path = args.xml_file.expect("No XML file provided");
    let xml_file = File::open(xml_path).expect("Could not open XML file");
    let file = BufReader::new(xml_file);

    let parser = EventReader::new(file);

    let mut parse_state = ParseState::None;
    let mut parse_product = ParseProduct::None;
    let mut parse_sale = ParseSale::None;
    let mut product = Product::default();
    let mut sale = Sale::default();

    for event in parser {
        match &parse_state {
            ParseState::None => match event {
                Ok(XmlEvent::StartElement { name, .. }) if name.local_name == "product" => {
                    parse_state = ParseState::Product;
                    product = Product::default();
                    println!("Found product");
                }
                Ok(XmlEvent::StartElement { name, .. }) if name.local_name == "sale" => {
                    parse_state = ParseState::Sale;
                    sale = Sale::default();
                    println!("Found sale");
                },
                Ok(XmlEvent::EndElement { .. }) => {
                    println!("End document");
                    parse_state = ParseState::None;
                }
                _ => {}
            },
            ParseState::Product => match &parse_product {
                ParseProduct::None => match event {
                    Ok(XmlEvent::StartElement { ref name, .. }) if name.local_name == "id" => {
                        parse_product = ParseProduct::Id;
                    }
                    Ok(XmlEvent::StartElement { ref name, .. }) if name.local_name == "category" => {
                        parse_product = ParseProduct::Category;
                    }
                    Ok(XmlEvent::StartElement { ref name, .. }) if name.local_name == "name" => {
                        parse_product = ParseProduct::Name;
                    }
                    Ok(XmlEvent::EndElement { .. }) => {
                        parse_product = ParseProduct::None;
                        parse_state = ParseState::None;
                        products.push(product.clone());
                        println!("Exit product: {:?}", product);
                    }
                    _ => {}
                },
                ParseProduct::Id => match event {
                    Ok(XmlEvent::Characters(ref text)) => {
                        product.id = text.parse().unwrap();
                    }
                    Ok(XmlEvent::EndElement { .. }) => {
                        parse_product = ParseProduct::None;
                    }
                    _ => {}
                },
                ParseProduct::Category => match event {
                    Ok(XmlEvent::Characters(ref text)) => {
                        product.category = text.to_string();
                    }
                    Ok(XmlEvent::EndElement { .. }) => {
                        parse_product = ParseProduct::None;
                    }
                    _ => {}
                },
                ParseProduct::Name => match event {
                    Ok(XmlEvent::Characters(ref text)) => {
                        product.name = text.to_string();
                    }
                    Ok(XmlEvent::EndElement { .. }) => {
                        parse_product = ParseProduct::None;
                    }
                    _ => {}
                },
                _ => {}
            },
            ParseState::Sale => match &parse_sale {
                ParseSale::None => match event {
                    Ok(XmlEvent::StartElement { ref name, .. }) if name.local_name == "id" => {
                        parse_sale = ParseSale::Id;
                    }
                    Ok(XmlEvent::StartElement { ref name, .. }) if name.local_name == "product-id" => {
                        parse_sale = ParseSale::ProductId;
                    }
                    Ok(XmlEvent::StartElement { ref name, .. }) if name.local_name == "date" => {
                        parse_sale = ParseSale::Date;
                    }
                    Ok(XmlEvent::StartElement { ref name, .. }) if name.local_name == "quantity" => {
                        parse_sale = ParseSale::Quantity;
                    }
                    Ok(XmlEvent::StartElement { ref name, .. }) if name.local_name == "unit" => {
                        parse_sale = ParseSale::Unit;
                    }
                    Ok(XmlEvent::EndElement { .. }) => {
                        parse_sale = ParseSale::None;
                        parse_state = ParseState::None;
                        sales.push(sale.clone());
                        println!("Exit sale: {:?}", sale);
                    }
                    _ => {}
                }
                ParseSale::Id => match event {
                    Ok(XmlEvent::Characters(ref text)) => {
                        sale.id = text.to_string();
                    }
                    Ok(XmlEvent::EndElement { .. }) => {
                        parse_sale = ParseSale::None;
                    }
                    _ => {}
                }
                ParseSale::ProductId => match event {
                    Ok(XmlEvent::Characters(ref text)) => {
                        sale.product_id = text.parse().unwrap();
                    }
                    Ok(XmlEvent::EndElement { .. }) => {
                        parse_sale = ParseSale::None;
                    }
                    _ => {}
                }
                ParseSale::Date => match event {
                    Ok(XmlEvent::Characters(ref text)) => {
                        sale.date = text.parse().unwrap();
                    }
                    Ok(XmlEvent::EndElement { .. }) => {
                        parse_sale = ParseSale::None;
                    }
                    _ => {}
                }
                ParseSale::Quantity => match event {
                    Ok(XmlEvent::Characters(ref text)) => {
                        sale.quantity = text.parse().unwrap();
                    }
                    Ok(XmlEvent::EndElement { .. }) => {
                        parse_sale = ParseSale::None;
                    }
                    _ => {}
                }
                ParseSale::Unit => match event {
                    Ok(XmlEvent::Characters(ref text)) => {
                        sale.unit = text.to_string();
                    }
                    Ok(XmlEvent::EndElement { .. }) => {
                        parse_sale = ParseSale::None;
                    }
                    _ => {}
                }
                _ => {}
            }
            _ => {}
        }
    }

    println!("Products: {:?}", products);
    println!("Sales: {:?}", sales);
}
