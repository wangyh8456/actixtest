use crate::handlers::{course::*,general::*};
use actix_web::web;

pub fn general_routes(cfg:&mut web::ServiceConfig){
    cfg.route("/health",web::get().to(health_check_handler));
}

pub fn course_routes(cfg:&mut web::ServiceConfig){
    //scope:定义一个新的作用域，可以在作用域内定义路由和各种资源
    cfg.service(web::scope("/courses")
    .route("/",web::post().to(new_course))
    .route("/{teacher_id}",web::get().to(get_courses_for_teacher))
    .route("/{teacher_id}/{course_id}",web::get().to(get_course_detail))
    .route("/{teacher_id}/{course_id}", web::delete().to(delete_course))
    .route("/{teacher_id}/{course_id}", web::put().to(update_course_details))
    );
}