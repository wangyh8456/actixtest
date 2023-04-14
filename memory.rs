// use super::state::AppState;
// use actix_web::{web,HttpResponse};

// pub async fn health_check_handler(app_state:web::Data<AppState>)->HttpResponse{
//     let health_check_response=&app_state.health_check_state;
//     //lock防止多个线程同时访问visit_count更新
//     let mut visit_count=app_state.visit_count.lock().unwrap();
//     let response=format!("{} {} times",health_check_response,visit_count);
//     *visit_count+=1;
//     HttpResponse::Ok().json(&response)
// }

// use chrono::Utc;
// use super::models::Course;

// pub async fn new_course(
//     new_course:web::Json<Course>,
//     app_state:web::Data<AppState>,
// )->HttpResponse{
//     let course_count=app_state.courses.lock().unwrap().clone().into_iter()
//     .filter(|course| course.teacher_id==new_course.teacher_id)
//     .collect::<Vec<Course>>()
//     .len();
//     let new_course=Course{
//         teacher_id:new_course.teacher_id,
//         id:Some(course_count+1),
//         name:new_course.name.clone(),
//         time:Some(Utc::now().naive_utc()),
//     };
//     app_state.courses.lock().unwrap().push(new_course.clone());
//     HttpResponse::Ok().json(&new_course)
// }

// pub async fn get_courses_for_teacher(
//     //web::Path<>中放的是带一个usize类型的元组
//     params:web::Path<(usize)>,
//     app_state:web::Data<AppState>,
// )->HttpResponse{
//     let teacher_id=params.0;
//     let filter_courses=app_state.courses.lock().unwrap().clone().into_iter()
//                        .filter(|course| course.teacher_id==teacher_id)
//                        .collect::<Vec<Course>>();
//     if filter_courses.len()>0 {
//         HttpResponse::Ok().json(filter_courses)
//     }else{
//         HttpResponse::Ok().json("No courses found for teacher".to_string())
//     }
// }

// pub async fn get_course_detail(
//     params:web::Path<(usize,usize)>,
//     app_state:web::Data<AppState>,
// )->HttpResponse{
//     let (teacher_id,course_id)=params.0;
//     let filter_courses=app_state.courses.lock().unwrap().clone().into_iter()
//                        .find(|course| course.teacher_id==teacher_id&&course.id==Some(course_id))
//                        //ok_or:如果Option是Some，则返回Ok，否则返回Err
//                         .ok_or("Course not found");
//     if let Ok(course)=filter_courses{
//         HttpResponse::Ok().json(course)
//     }else{
//         HttpResponse::Ok().json("Course not found".to_string())
//     }
// }

// #[cfg(test)]
// mod tests{
//     use super::*;
//     use actix_web::http::StatusCode;
//     use std::sync::Mutex;

//     #[actix_rt::test]
//     async fn post_course_test(){
//         let course=web::Json(Course{
//             teacher_id:1,
//             id:None,
//             name:String::from("test"),
//             time:None,
//         });
//         let app_state=web::Data::new(AppState{
//             health_check_state:String::from("I am OK"),
//             visit_count:Mutex::new(0),
//             courses:Mutex::new(vec![]),
//         });
//         let resp=new_course(course,app_state).await;
//         assert_eq!(resp.status(),StatusCode::OK);
//     }
//     #[actix_rt::test]
//     async fn get_all_courses_success(){
//         let app_state=web::Data::new(AppState{
//             health_check_state:String::from("I am OK"),
//             visit_count:Mutex::new(0),
//             courses:Mutex::new(vec![
//                 Course{
//                     teacher_id:1,
//                     id:Some(1),
//                     name:String::from("Math"),
//                     time:Some(Utc::now().naive_utc()),
//                 },
//                 Course{
//                     teacher_id:1,
//                     id:Some(2),
//                     name:String::from("Chinese"),
//                     time:Some(Utc::now().naive_utc()),
//                 },
//             ]),
//         });
//         let teacher_id:web::Path<(usize)>=web::Path::from((1));
//         let resp=get_courses_for_teacher(teacher_id,app_state).await;
//         assert_eq!(resp.status(),StatusCode::OK);
//     }
//     #[actix_rt::test]
//     async fn get_one_course_success(){
//         let app_state=web::Data::new(AppState{
//             health_check_state:String::from("I am OK"),
//             visit_count:Mutex::new(0),
//             courses:Mutex::new(vec![
//                 Course{
//                     teacher_id:1,
//                     id:Some(1),
//                     name:String::from("Math"),
//                     time:Some(Utc::now().naive_utc()),
//                 },
//                 Course{
//                     teacher_id:1,
//                     id:Some(2),
//                     name:String::from("Chinese"),
//                     time:Some(Utc::now().naive_utc()),
//                 },
//             ]),
//         });
//         let params:web::Path<(usize,usize)>=web::Path::from((1,1));
//         let resp=get_course_detail(params,app_state).await;
//         assert_eq!(resp.status(),StatusCode::OK);
//     }
// }