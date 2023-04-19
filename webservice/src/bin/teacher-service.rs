use actix_web::{web,App,HttpServer,http};
use errors::MyError;
use std::io;
use std::sync::Mutex;
use dotenv::dotenv;
use std::env;
use sqlx::mysql::MySqlPoolOptions;

#[path = "../handlers/mod.rs"]
mod handlers;
#[path = "../routers.rs"]
mod routers;
#[path = "../state.rs"]
mod state;
#[path = "../models/mod.rs"]
mod models;
#[path = "../dbaccess/mod.rs"]
mod dbaccess;
#[path = "../errors.rs"]
mod errors;

use routers::*;
use actix_cors::Cors;
use state::AppState;

#[actix_rt::main]
async fn main()->io::Result<()>{
    dotenv().ok();
    let database_url=env::var("DATABASE_URL").expect("DATABASE_URL must be set!!!");
    let pool=sqlx::mysql::MySqlPoolOptions::new().max_connections(5).connect(&database_url).await.unwrap();
    let shared_data=web::Data::new(AppState{
        health_check_state:String::from("I am OK"),
        visit_count:Mutex::new(0),
        // courses:Mutex::new(vec![]),
        mb_pool:pool,
    });
    let app=move || {
        let cors=Cors::default()
            .allowed_origin("http://localhost:8000/")
            .allowed_origin_fn(|origin,_req_head|{
                //允许所有以http://localhost开头的域名
                origin.as_bytes().starts_with(b"http://localhost")
            })
            .allowed_methods(vec!["GET","POST","DELETE"])
            .allowed_headers(vec![http::header::AUTHORIZATION,http::header::ACCEPT])
            .allowed_header(http::header::CONTENT_TYPE)
            .max_age(3600);

        App::new().app_data(shared_data.clone())
        .app_data(web::JsonConfig::default().error_handler(|_err,_req|{
            MyError::InvalidError("Please check your input!".to_string()).into()
        }))
        .configure(general_routes)
        .configure(course_routes)
        .configure(teacher_routes)
        .wrap(cors)
    };
    HttpServer::new(app).bind("127.0.0.1:3001")?.run().await
}