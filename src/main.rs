use actix_web::{web, App, HttpServer, Result, HttpResponse, Responder, get};
use listenfd::ListenFd;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct MyObj {
    name: String,
}

fn index(info: web::Path<(u32, String)>) -> impl Responder {
    format!("Hello {}! id:{}", info.1, info.0)
}

#[get("/hello/{name}")]
fn hello(obj: web::Path<MyObj>) -> Result<HttpResponse> {
    Ok(HttpResponse::Ok().json(MyObj {
        name: obj.name.to_string(),
    }))
}

fn main() {
    let mut listenfd = ListenFd::from_env();
    let mut server = HttpServer::new(
        || App::new()
            .service(web::resource("/{id}/{name}/index.html").to(index))
            .service(
                web::scope("/api").service(hello)
            )
        );

    server = if let Some(l) = listenfd.take_tcp_listener(0).unwrap() {
        server.listen(l).unwrap()
    } else {
        server.bind("127.0.0.1:3000").unwrap()
    };

    server.run().unwrap();
}