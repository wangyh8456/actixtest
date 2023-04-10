use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use std::io;

//配置route
pub fn general_routes(cfg:&mut web::ServiceConfig){
    cfg.route("/health",web::get().to(health_check_handler));
}

//配置handler
pub async fn health_check_handler() -> impl Responder {
    HttpResponse::Ok().json("actix web service is running!")
}

//实例化server并运行
#[actix_rt::main]
async fn main()->io::Result<()>{
    //构建app，配置route
    //Actix Http Server默认开启多个线程，如果不用move方式的闭包捕捉general_routes，则可能出现错误
    let app=move || App::new().configure(general_routes);

    //运行http server
    HttpServer::new(app).bind("127.0.0.1:3000")?.run().await   
}