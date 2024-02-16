use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use std::sync::{Arc, Mutex};
use sqlx::{mysql::{MySqlPool, MySqlPoolOptions}, Error};

// Example product structure
struct Product {
    id: u32,
    name: String,
    price: f32,
}

// Database-Simulation for products
struct Database {
    products: Vec<Product>,
}

impl Database {
    fn new() -> Self {
        Database {
            products: vec![
                Product {
                    id: 1,
                    name: String::from("Product 1"),
                    price: 10.0,
                },
                Product {
                    id: 2,
                    name: String::from("Product 2"),
                    price: 15.0,
                },
                Product {
                    id: 3,
                    name: String::from("Product 3"),
                    price: 20.0,
                },
            ],
        }
    }

    // Return method for products (all)
    fn get_products(&self) -> &Vec<Product> {
        &self.products
    }

    // Return method for one product by id
    fn get_product_by_id(&self, id: u32) -> Option<&Product> {
        self.products.iter().find(|p| p.id == id)
    }
}

async fn index(db: web::Data<Database>) -> impl Responder {
    let products = db.get_products();
    let mut response_body = String::from("<h1>Products</h1><ul>");

    for product in products {
        response_body.push_str(&format!(
            "<li>{}</li><li>Price: ${}</li><br>",
            &product.name, &product.price
        ));
    }
    response_body.push_str("</ul>");
    HttpResponse::Ok().body(response_body)
}

async fn product_detail(db: web::Data<Database>, path: web::Path<u32>) -> impl Responder {
    if let Some(product) = db.get_product_by_id(*path) {
        HttpResponse::Ok().body(format!(
            "<h2>{}</h2><p>Price: ${}</p>",
            &product.name, &product.price
        ))
    } else {
        HttpResponse::NotFound().body("Product not found")
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Create new database instance
    let db = Arc::new(Mutex::new(Database::new()));

    // Start HTTP-Server
    HttpServer::new(move || {
        App::new()
            .data(db.clone()) // Pass database reference
            .route("/", web::get().to(index))
            .route("/{id}", web::get().to(product_detail))
    })
        .bind("127.0.0.1:8080")?
        .run()
        .await
}
// connection to google-sql

pub async fn establish_connection() -> Result<MySqlPool, Error> {
    let db_url = "mysql://username:password@<PUBLIC_IP>:3306/Rust_testdb"; // Replace placeholders with actual values
    let pool = MySqlPoolOptions::new()
        .max_connections(5) // Max connections
        .connect(db_url)
        .await?;

    Ok(pool)

}