use actix_web::{web, App, HttpServer, Responder, HttpResponse};
use serde::Deserialize;

#[derive(Deserialize)]
struct MathQuery {
    a: f64,
    b: f64,
}

async fn add(query: web::Query<MathQuery>) -> impl Responder {
    let result = query.a + query.b;
    HttpResponse::Ok().body(format!("{}", result))
}

async fn subtract(query: web::Query<MathQuery>) -> impl Responder {
    let result = query.a - query.b;
    HttpResponse::Ok().body(format!("{}", result))
}

async fn multiply(query: web::Query<MathQuery>) -> impl Responder {
    let result = query.a * query.b;
    HttpResponse::Ok().body(format!("{}", result))
}

async fn divide(query: web::Query<MathQuery>) -> impl Responder {
    if query.b == 0.0 {
        HttpResponse::BadRequest().body("Error: Division by zero")
    } else {
        let result = query.a / query.b;
        HttpResponse::Ok().body(format!("{}", result))
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .route("/add", web::get().to(add))
            .route("/subtract", web::get().to(subtract))
            .route("/multiply", web::get().to(multiply))
            .route("/divide", web::get().to(divide))
    })
    .bind("127.0.0.1:8082")?
    .run()
    .await
}
