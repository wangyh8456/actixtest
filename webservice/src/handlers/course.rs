use crate::state::AppState;
use actix_web::{web,HttpResponse};
use crate::dbaccess::course::*;
use crate::errors::MyError;

use crate::models::course::{Course,CreateCourse,UpdateCourse};

pub async fn new_course(
    new_course:web::Json<CreateCourse>,
    app_state:web::Data<AppState>,
)->Result<HttpResponse,MyError>{
    post_new_course_db(&app_state.mb_pool, new_course.try_into()?).await
    .map(|course|HttpResponse::Ok().json(course))
}

pub async fn get_courses_for_teacher(
    //web::Path<>中放的是带一个usize类型的元组
    params:web::Path<(usize, )>,
    app_state:web::Data<AppState>,
)->Result<HttpResponse,MyError>{
    let teacher_id=i32::try_from(params.0).unwrap();
    get_courses_for_teacher_db(&app_state.mb_pool, teacher_id).await
    .map(|courses|HttpResponse::Ok().json(courses))
}

pub async fn get_course_detail(
    params:web::Path<(usize,usize)>,
    app_state:web::Data<AppState>,
)->Result<HttpResponse,MyError>{
    let teacher_id=i32::try_from(params.0).unwrap();
    let course_id=i32::try_from(params.1).unwrap();
    get_course_detail_db(&app_state.mb_pool, teacher_id, course_id).await
    .map(|course|HttpResponse::Ok().json(course))
}

pub async fn delete_course(
    params:web::Path<(usize,usize)>,
    app_state:web::Data<AppState>,
)->Result<HttpResponse,MyError>{
    let teacher_id=i32::try_from(params.0).unwrap();
    let course_id=i32::try_from(params.1).unwrap();
    delete_course_db(&app_state.mb_pool, teacher_id, course_id).await
    .map(|resp|HttpResponse::Ok().json(resp))
}

pub async fn update_course_details(
    params:web::Path<(usize,usize)>,
    update_course:web::Json<UpdateCourse>,
    app_state:web::Data<AppState>,
)->Result<HttpResponse,MyError>{
    let teacher_id=i32::try_from(params.0).unwrap();
    let course_id=i32::try_from(params.1).unwrap();
    update_course_db(&app_state.mb_pool, teacher_id, course_id, update_course.into()).await
    .map(|resp|HttpResponse::Ok().json(resp))
}

#[cfg(test)]
mod tests{
    use super::*;
    use actix_web::http::StatusCode;
    use std::sync::Mutex;
    use chrono::NaiveDateTime;
    use dotenv::dotenv;
    use sqlx::mysql::MySqlPoolOptions;
    use std::env;

    // #[ignore]
    // #[actix_rt::test]
    // async fn post_course_test(){
    //     dotenv().ok();
    //     let db_url=env::var("DATABASE_URL").expect("DATABASE_URL must be set!!!");
    //     let db_pool=MySqlPoolOptions::new().connect(&db_url).await.unwrap();
    //     let app_state=web::Data::new(AppState{
    //         health_check_state:String::from("I am OK"),
    //         visit_count:Mutex::new(0),
    //         mb_pool:db_pool,
    //     });
    //     let course=web::Json(Course{
    //         teacher_id:1,
    //         id:Some(13),
    //         name:String::from("English"),
    //         time:None,
    //     });
    //     let resp=new_course(course,app_state).await.unwrap();
    //     assert_eq!(resp.status(),StatusCode::OK);
    // }
    #[actix_rt::test]
    async fn get_all_courses_success(){
        dotenv().ok();
        let db_url=env::var("DATABASE_URL").expect("DATABASE_URL must be set!!!");
        let db_pool=MySqlPoolOptions::new().connect(&db_url).await.unwrap();
        let app_state=web::Data::new(AppState{
            health_check_state:String::from("I am OK"),
            visit_count:Mutex::new(0),
            mb_pool:db_pool,
        });
        let teacher_id:web::Path<(usize,)>=web::Path::from((1,));
        let resp=get_courses_for_teacher(teacher_id,app_state).await.unwrap();
        assert_eq!(resp.status(),StatusCode::OK);
    }
    // #[actix_rt::test]
    // async fn get_one_course_success(){
    //     dotenv().ok();
    //     let db_url=env::var("DATABASE_URL").expect("DATABASE_URL must be set!!!");
    //     let db_pool=MySqlPoolOptions::new().connect(&db_url).await.unwrap();
    //     let app_state=web::Data::new(AppState{
    //         health_check_state:String::from("I am OK"),
    //         visit_count:Mutex::new(0),
    //         mb_pool:db_pool,
    //     });
    //     let params:web::Path<(usize,usize)>=web::Path::from((1,1));
    //     let resp=get_course_detail(params,app_state).await.unwrap();
    //     assert_eq!(resp.status(),StatusCode::OK);
    // }
}