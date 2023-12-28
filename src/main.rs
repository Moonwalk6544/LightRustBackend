mod models;

use actix_web::{Responder, get, patch, post, HttpResponse, HttpServer, App};

use std::io;






#[get("/")]
async fn get_main() -> impl Responder {
    HttpResponse::Ok().body("hello!")

}

#[actix_web::main]


async fn main() -> std::io::Result<()> {

    HttpServer::new(|| App::new().service(get_main)) //? operator is like try catch****
        .bind("127.0.0.1:8080")?.run()
        .await

}
