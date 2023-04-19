use crate::models::teacher::{CreateTeacher, UpdateTeacher};
use crate::dbaccess::teacher::*;
use crate::errors::MyError;
use crate::state::AppState;
use actix_web::{web, HttpResponse};

pub async fn get_all_teachers(
    app_state: web::Data<AppState>,
) -> Result<HttpResponse, MyError> {
    get_all_teachers_db(&app_state.mb_pool)
        .await
        .map(|teachers| HttpResponse::Ok().json(teachers))
}

pub async fn get_teacher_details(
    params: web::Path<u32>,
    app_state: web::Data<AppState>,
) -> Result<HttpResponse, MyError> {
    let teacher_id = params.into_inner();
    get_teacher_details_db(&app_state.mb_pool, teacher_id)
        .await
        //Teacher实现了serde::Serialize，所以可以直接用json方法
        .map(|teacher| HttpResponse::Ok().json(teacher))
}

pub async fn post_new_teacher(
    new_teacher: web::Json<CreateTeacher>,
    app_state: web::Data<AppState>,
) -> Result<HttpResponse, MyError> {
    post_new_teacher_db(&app_state.mb_pool, new_teacher.into())
        .await
        .map(|teacher| HttpResponse::Ok().json(teacher))
}

pub async fn update_teacher_details(
    app_state: web::Data<AppState>,
    params: web::Path<u32>,
    update_teacher: web::Json<UpdateTeacher>
)->Result<HttpResponse,MyError>{
    let teacher_id=params.into_inner();
    update_teacher_db(&app_state.mb_pool,teacher_id,update_teacher.into())
        .await
        .map(|teacher| HttpResponse::Ok().json(teacher))
}

pub async fn delete_teacher(
    app_state: web::Data<AppState>,
    params: web::Path<u32>,
)->Result<HttpResponse,MyError>{
    let teacher_id=params.into_inner();
    delete_teacher_db(&app_state.mb_pool,teacher_id)
        .await
        .map(|teacher| HttpResponse::Ok().json(teacher))
}

#[cfg(test)]
mod tests{
    use super::*;
    use actix_web::http::StatusCode;
    use dotenv::dotenv;
    use sqlx::mysql::MySqlPoolOptions;
    use std::env;
    use std::sync::Mutex;

    #[actix_rt::test]
    async fn get_all_teachers_test(){
        dotenv().ok();
        let database_url=env::var("DATABASE_URL").expect("DATABASE_URL must be set");
        let pool=MySqlPoolOptions::new()
            .max_connections(5)
            .connect(&database_url)
            .await
            .unwrap();
        let app_state=web::Data::new(AppState{
            mb_pool:pool,
            visit_count:Mutex::new(0),
            health_check_state:"".to_string(),
        });
        let response=get_all_teachers(app_state).await.unwrap();
        assert_eq!(response.status(),StatusCode::OK);
    }

    #[actix_rt::test]
    async fn get_teacher_details_test(){
        dotenv().ok();
        let database_url=env::var("DATABASE_URL").expect("DATABASE_URL must be set");
        let pool=MySqlPoolOptions::new()
            .max_connections(5)
            .connect(&database_url)
            .await
            .unwrap();
        let app_state=web::Data::new(AppState{
            mb_pool:pool,
            visit_count:Mutex::new(0),
            health_check_state:"".to_string(),
        });
        let response=get_teacher_details(web::Path::from(2),app_state).await.unwrap();
        assert_eq!(response.status(),StatusCode::OK);
    }

    #[ignore]
    #[actix_rt::test]
    async fn post_new_teacher_test(){
        dotenv().ok();
        let database_url=env::var("DATABASE_URL").expect("DATABASE_URL must be set");
        let pool=MySqlPoolOptions::new()
            .max_connections(5)
            .connect(&database_url)
            .await
            .unwrap();
        let app_state=web::Data::new(AppState{
            mb_pool:pool,
            visit_count:Mutex::new(0),
            health_check_state:"".to_string(),
        });
        let new_teacher=CreateTeacher{
            name:"New Teacher".into(),
            picture_url:"https://up.enterdesk.com/edpic_source/0e/54/54/0e5454f85bbc336d6db640aa8fb0be6d.jpg".into(),
            profile:"This is a new teacher".into(),
        };
        let response=post_new_teacher(web::Json(new_teacher),app_state).await.unwrap();
        assert_eq!(response.status(),StatusCode::OK);
    }

    #[actix_rt::test]
    async fn update_teacher_details_test(){
        dotenv().ok();
        let database_url=env::var("DATABASE_URL").expect("DATABASE_URL must be set");
        let pool=MySqlPoolOptions::new()
            .max_connections(5)
            .connect(&database_url)
            .await
            .unwrap();
        let app_state=web::Data::new(AppState{
            mb_pool:pool,
            visit_count:Mutex::new(0),
            health_check_state:"".to_string(),
        });
        let update_teacher=UpdateTeacher{
            name:Some("Updated Teacher".into()),
            picture_url:Some("https://up.enterdesk.com/edpic_source/0e/54/54/0e5454f85bbc336d6db640aa8fb0be6d.jpg".into()),
            profile:Some("This is a updated teacher".into()),
        };
        let response=update_teacher_details(app_state,web::Path::from(2),web::Json(update_teacher)).await.unwrap();
        assert_eq!(response.status(),StatusCode::OK);
    }

    #[ignore]
    #[actix_rt::test]
    async fn delete_teacher_test(){
        dotenv().ok();
        let database_url=env::var("DATABASE_URL").expect("DATABASE_URL must be set");
        let pool=MySqlPoolOptions::new()
            .max_connections(5)
            .connect(&database_url)
            .await
            .unwrap();
        let app_state=web::Data::new(AppState{
            mb_pool:pool,
            visit_count:Mutex::new(0),
            health_check_state:"".to_string(),
        });
        let response=delete_teacher(app_state,web::Path::from(1)).await.unwrap();
        assert_eq!(response.status(),StatusCode::OK);
    }
}


