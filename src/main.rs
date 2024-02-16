use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use std::sync::{Arc, Mutex};
use actix_web::error::UrlencodedError::Encoding;
use jsonwebtoken::{decode, DecodingKey, encode, EncodingKey, Header, Validation};
use sqlx::{mysql::{MySqlPool, MySqlPoolOptions}, Error};
use serde::{Serialize, Deserialize};

#[cfg(test)]
mod tests{
    use super::*;
    use actix_web::test;
    use actix_web::http::StatusCode;

    #[actix_rt::test]
    async fn test_index(){
        let db = Arc::new(Mutex::new(Database::new()));
        let app = test::init_service(App::new().data(db.clone()).route("/{id}", web::get().to(product_detail))).await;
        let req = test::TestRequest::with_uri("/1").to_request();
        let resp = test::call_service(&app, req).await;
        assert_eq!(resp.status(), StatusCode::OK);
        let body = test::read_body(resp).await;
        assert!(body.contains("Product 1"));
    }

    #[actix_rt::test]
    async fn test_login(){
        let resp = login().await;
        assert_eq!(resp.status(), StatusCode::OK);
    }
}

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
                Product { id: 1, name: String::from("Product 1"), price: 10.0 },
                Product { id: 2, name: String::from("Product 2"), price: 15.0 },
                Product { id: 3, name: String::from("Product 3"), price: 20.0 },
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
    println!("A request was sent to the endpoint '/'");
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

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Erstellen einer neuen Datenbankinstanz
    let db = Arc::new(Mutex::new(Database::new()));

    // Starten des HTTP-Servers
    HttpServer::new(move || {
        App::new()
            .data(db.clone()) // Übergabe der Datenbankreferenz
            .route("/", web::get().to(index)) // Index-Endpunkt
    })
        .bind("127.0.0.1:8080")?
        .run()
        .await
}

// create a JWT-Token
fn generate_jwt_token(user_id: u32) -> String {
    let token_data = Claims { user_id };
    encode(&Header::default(), &token_data, &EncodingKey::from_secret("secret".as_ref())).unwrap()
}

// check for JWT-Token
fn verify_jwt_token(token: &str) -> Option<u32> {
    match decode::<Claims>(&token, &DecodingKey::from_secret("secret".as_ref()), &Validation::default()) {
        Ok(token_data) => Some(token_data.claims.user_id),
        Err(_) => None,
    }
}

// example for JWT-Token (must be optimized)
#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    user_id: u32,
}

async fn login() -> HttpResponse {
    let user_id = 123;
    let token = generate_jwt_token(user_id);
    HttpResponse::Ok().body(token)
}

async fn authorized_action(token: web::Json<String>) -> HttpResponse {
    match verify_jwt_token(&token) {
        Some(_) => HttpResponse::Ok().body("Authorized action"),
        None => HttpResponse::Unauthorized().body("Unauthorized action"),
    }
}

// connection to google-sql
pub async fn establish_connection() -> Result<MySqlPool, Error> {
    let db_url = "mysql://newvalueai:europe-west3:test1@34.159.175.13:3306/Rust_testdb";
    MySqlPoolOptions::new()
        .max_connections(5) // Max connections
        .connect(db_url)
        .await
}
