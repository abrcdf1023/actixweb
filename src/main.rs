use actix_web::{web, App, HttpServer, Result, HttpResponse, Responder, get};
use listenfd::ListenFd;
use serde::{Deserialize, Serialize};
use std::process::Command;

#[derive(Serialize, Deserialize)]
struct MyObj {
    name: String,
}

fn call_python(value:&str) -> String {
    let mut translate = Command::new("python3");
    translate.arg("run_nn.py").arg("translate").arg(value);
    translate.current_dir("/Users/liyanxin/Life/myprojects/Chinese2English_Seq2Seq_Attention");
    let output = translate.output().expect("failed to execute process");
    String::from_utf8_lossy(&output.stdout).to_string()
}

fn index(info: web::Path<(u32, String)>) -> impl Responder {
    format!("Hello {}! id:{}", info.1, info.0)
}

#[get("/hello/{name}")]
fn hello(obj: web::Path<MyObj>) -> Result<HttpResponse> {
    let output = call_python(&obj.name);
    Ok(HttpResponse::Ok().json(MyObj {
        name: output,
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