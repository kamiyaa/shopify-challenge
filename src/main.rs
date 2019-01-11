#![feature(proc_macro_hygiene, decl_macro)]
#[macro_use]
extern crate rocket;
#[macro_use]
extern crate lazy_static;

extern crate serde;
extern crate serde_json;

#[macro_use]
extern crate serde_derive;

use std::collections::HashMap;
use std::sync;

lazy_static! {
    /* used as a reference database */
    static ref database: sync::Mutex<HashMap<String, Product>> = sync::Mutex::new(HashMap::new());
}

#[derive(Clone, Debug, Serialize, Deserialize)]
struct Product {
    pub title: String,
    pub price: f64,
    pub inventory: u64,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
struct JsonObjVec<T> {
    pub results: Vec<T>,
}

impl<T> JsonObjVec<T> {
    pub fn new() -> Self {
        JsonObjVec {
            results: Vec::new(),
        }
    }
}

/// Handler for GET / Request
#[get("/")]
fn default_page() -> &'static str {
    "Hello, Shopify!"
}

fn all_products_to_json() -> String
{
    let db = database.lock().unwrap();
    let mut products: JsonObjVec<Product> = JsonObjVec::new();
    for (_, val) in (*db).iter() {
        products.results.push(val.clone());
    }

    serde_json::to_string_pretty(&products).unwrap_or(String::from("{}"))
}

/// Handler for GET /api/products Request
#[get("/api/products")]
fn get_all_products() -> String {
    all_products_to_json()
}

/// Handler for GET /api/products/<title> Request
#[get("/api/products/<title>")]
fn get_product(title: String) -> String {
    let db = database.lock().unwrap();
    match db.get(&title) {
        Some(s) => serde_json::to_string_pretty(&s).unwrap_or(String::from("{}")),
        None => String::from("{}"),
    }
}

/// Populate the reference database
fn populate_database()
{
    let mut db = database.lock().unwrap();

    // put in some test data
    let product = Product {
            title: String::from("Apples"),
            price: 5.00,
            inventory: 10,
        };
    db.insert(String::from("Apples"), product);

    let product = Product {
            title: String::from("Pears"),
            price: 6.00,
            inventory: 15,
        };
    db.insert(String::from("Pears"), product);

    let product = Product {
            title: String::from("Banana"),
            price: 4.00,
            inventory: 19,
        };
    db.insert(String::from("Banana"), product);

}

fn main() {
    populate_database();

    let routes = routes![default_page, get_product, get_all_products];
    rocket::ignite().mount("/", routes).launch();
}
