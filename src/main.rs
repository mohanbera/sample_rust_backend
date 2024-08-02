use actix_web::{get, App, HttpResponse, HttpServer, Responder};
use serde::Serialize;

#[derive(Serialize)]
struct User {
    id: i32,
    name: String,
    email: String
}
#[get("/")]
async fn hello() -> impl Responder {
    let user = User {
        id: 1,
        name: "name".to_string(),
        email: "op@gmail.com".to_string()
    };
    HttpResponse::Ok().json(user)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().service(hello))
        .bind(("0.0.0.0", 8082))?
        .run()
        .await
}
