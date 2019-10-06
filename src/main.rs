#[macro_use]
extern crate log;
extern crate env_logger;

use actix_web::middleware::Logger;
use actix_web::{web, App, HttpServer, Result, HttpResponse, get};
use listenfd::ListenFd;
use serde::{Deserialize, Serialize};
use std::process::Command;

#[derive(Serialize, Deserialize)]
struct Chinese {
    text: String,
}

/**
 * Translate chinese to English with deep learning.
 */
fn exec_translate_from_python(value:&str) -> String {
    let mut _translate = Command::new("python3");
    _translate.arg("run_nn.py").arg("translate").arg(value);
    _translate.current_dir("/Users/liyanxin/Life/myprojects/Chinese2English_Seq2Seq_Attention");
    let output = _translate.output().expect("failed to execute process");
    String::from_utf8_lossy(&output.stdout).to_string()
}

#[get("/translate")]
fn translate(info: web::Query<Chinese>) -> Result<HttpResponse> {
    let output = exec_translate_from_python(&info.text);
    Ok(HttpResponse::Ok().json(Chinese {
        text: output,
    }))
}

fn main() {
    // init log
    std::env::set_var("RUST_LOG", "actix_web=info,info");
    env_logger::init();

    // settings
    let url = "127.0.0.1:3000";

    info!("Running server on {}", url);
    
    // auto reload
    let mut listenfd = ListenFd::from_env();

    // start server
    let mut server = HttpServer::new(
        || App::new()
            .service(
                web::scope("/api").service(translate)
            )
        );

    // auto reload listener
    server = if let Some(l) = listenfd.take_tcp_listener(0).unwrap() {
        server.listen(l).unwrap()
    } else {
        server.bind(url).unwrap()
    };

    server.run().unwrap();
}