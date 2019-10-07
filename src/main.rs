#[macro_use]
extern crate log;
extern crate env_logger;
extern crate diesel;
extern crate actixweb;

use actixweb::*;
use self::models::*;
use diesel::prelude::*;
use actix_web::middleware::errhandlers::{ErrorHandlerResponse, ErrorHandlers};
use actix_web::middleware::Logger;
use actix_web::{dev, http, web, App, HttpServer, Result, HttpResponse, get, middleware};
use listenfd::ListenFd;
use serde::{Deserialize, Serialize};
use std::process::Command;

#[derive(Serialize, Deserialize)]
struct Chinese {
    text: String,
}

fn render_500<B>(mut res: dev::ServiceResponse<B>) -> Result<ErrorHandlerResponse<B>> {
    res.response_mut().headers_mut().insert(
        http::header::CONTENT_TYPE,
        http::HeaderValue::from_static("Error"),
    );
    Ok(ErrorHandlerResponse::Response(res))
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
    // establish db connection
    use self::schema::posts::dsl::*;

    let connection = establish_connection();
    let results = posts.filter(published.eq(true))
        .limit(5)
        .load::<Post>(&connection)
        .expect("Error loading posts");

    println!("Displaying {} posts", results.len());
    for post in results {
        println!("{}", post.title);
        println!("----------\n");
        println!("{}", post.body);
    }

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
            .wrap(Logger::default())
            .wrap(Logger::new("%a %{User-Agent}i"))
            .wrap(middleware::DefaultHeaders::new().header("X-Version", "0.2"))
            .wrap(
                ErrorHandlers::new()
                    .handler(http::StatusCode::INTERNAL_SERVER_ERROR, render_500),
            )
            .service(
                web::scope("/api/v1").service(translate)
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