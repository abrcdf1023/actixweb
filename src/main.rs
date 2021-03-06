#[macro_use]
extern crate log;
extern crate env_logger;
extern crate actixweb;

use actix_web::middleware::errhandlers::{ErrorHandlerResponse, ErrorHandlers};
use actix_web::middleware::Logger;
use actix_web::{dev, http, web, App, HttpServer, Result, HttpResponse, get, post, patch, delete, middleware};
use listenfd::ListenFd;
use std::process::Command;

use self::actixweb::*;
use self::models::{Chinese, NewPost, UpdatePost};

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
fn exec_translate_from_python(value:&str) -> Vec<u8> {
    let mut _translate = Command::new("python3");
    _translate.arg("run_nn.py").arg("translate").arg(value);
    _translate.current_dir("/Users/liyanxin/Life/myprojects/Chinese2English_Seq2Seq_Attention");
    let output = _translate.output().expect("failed to execute process");
    output.stdout
}

#[get("/translate")]
fn translate(info: web::Query<Chinese>) -> Result<HttpResponse> {
    let output = exec_translate_from_python(&info.text);
    Ok(HttpResponse::Ok().json(Chinese {
        text: String::from_utf8_lossy(&output).to_string(),
    }))
}

#[post("/post")]
fn create_post_handler(pool: web::Data<PgPool>, params: web::Json<NewPost>) -> Result<HttpResponse> {
    let result = create_post(&pool.get().unwrap(), params.0);

    Ok(HttpResponse::Ok().json(result))
}

#[get("/posts")]
fn read_posts_handler(pool: web::Data<PgPool>) -> Result<HttpResponse> {
    let result = read_posts(&pool.get().unwrap());

    Ok(HttpResponse::Ok().json(result))
}

#[patch("/post/{id}")]
fn update_post_handler(pool: web::Data<PgPool>, params: web::Json<UpdatePost>, id: web::Path<String>) -> Result<HttpResponse> {
    let result = update_post(&pool.get().unwrap(), &id, params.0);

    Ok(HttpResponse::Ok().json(result))
}

#[delete("/post/{id}")]
fn delete_post_handler(pool: web::Data<PgPool>, id: web::Path<String>) -> Result<HttpResponse> {
    let result = delete_post(&pool.get().unwrap(), &id);

    Ok(HttpResponse::Ok().json(result))
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
            .data(establish_connection())
            .wrap(Logger::default())
            .wrap(Logger::new("%a %{User-Agent}i"))
            .wrap(middleware::DefaultHeaders::new().header("X-Version", "0.2"))
            .wrap(
                ErrorHandlers::new()
                    .handler(http::StatusCode::INTERNAL_SERVER_ERROR, render_500),
            )
            .service(
                web::scope("/api/v1")
                    .service(translate)
                    .service(create_post_handler)
                    .service(read_posts_handler)
                    .service(update_post_handler)
                    .service(delete_post_handler)
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