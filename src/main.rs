use askama::Template;
use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};

#[derive(Template)]
#[template(path = "index.html")]
struct IndexTemplate<'a> {
    title: &'a str,
}

#[derive(Template)]
#[template(path = "hello.html")]

struct HelloTemplate<'a> {
    name: &'a str,
}

#[derive(Template)]
#[template(path = "button.html")]
struct ButtonTemplate<'a> {
    name: &'a str,
}

#[get("/")]
async fn hello() -> impl Responder {
    let hello = HelloTemplate { name: "world" };
    //println!("{}", hello.render().unwrap());
    HttpResponse::Ok().body(hello.render().unwrap())
}

#[post("/button")]
async fn button() -> impl Responder {
    let button = ButtonTemplate { name: "button" };
    HttpResponse::Ok().body(button.render().unwrap())
}

#[actix_web::main]
async fn main() -> std::io::Result<()>{
    
    HttpServer::new(|| {
        App::new()
        .service(hello)
        .service(button)
    })
    .bind(("127.0.0.1",42069))?
    .run()
    .await


    

}
