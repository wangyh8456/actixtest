//旧版
use crate::models::course::Course;
use chrono::{NaiveDateTime,DateTime,Local,Utc};
use sqlx::{mysql::MySqlPool,MySql,QueryBuilder,Row};
use crate::errors::MyError;


pub async fn get_courses_for_teacher_db(
    pool:&MySqlPool,
    teacher_id:i32,
)->Result<Vec<Course>,MyError>{
    let rows=sqlx::query!(
        r#"
        SELECT id,teacher_id,name,time FROM course WHERE teacher_id=?
        "#,
        teacher_id
    )
    .fetch_all(pool)
    .await?;
    
    let courses:Vec<Course>=rows.iter().map(|r| Course{
        id:Some(r.id),
        teacher_id:r.teacher_id,
        name:r.name.clone(),
        time:Some(r.time.unwrap().naive_local()),
    }).collect();

    match courses.len(){
        0=>Err(MyError::NotFound("Courses not found for teacher".into())),
        _=>Ok(courses),
    }
}

pub async fn get_course_detail_db(
    pool:&MySqlPool,
    teacher_id:i32,
    course_id:i32,
)->Result<Course,MyError>{
    let row=sqlx::query!(
        r#"
        SELECT id,teacher_id,name,time FROM course WHERE teacher_id=? AND id=?
        "#,
        teacher_id,
        course_id
    )
    .fetch_one(pool)
    .await;
    
    //由于上面await后没有添加?，所以这里需要使用if let语句来处理，出现错误时走else分支
    if let Ok(row)=row{
        Ok(Course{
            id:Some(row.id),
            teacher_id:row.teacher_id,
            name:row.name.clone(),
            time:Some(row.time.unwrap().naive_local()),
        })
    }else{
        Err(MyError::NotFound("Course Id not found".into()))
    }
}

pub async fn post_new_course_db(
    pool:&MySqlPool,
    course:Course,
)->Result<Course,MyError>{
    let mut qb:QueryBuilder<MySql>=QueryBuilder::new(
        "INSERT INTO course(id,teacher_id,name) VALUES("
    );
    let mut separate=qb.separated(", ");
    separate.push_bind(course.id.unwrap()).push_bind(course.teacher_id).push_bind(course.name);
    separate.push_unseparated(")");
    qb.build()
    .execute(pool)
    .await?;

    let row=sqlx::query!(
        r#"
        SELECT id,teacher_id,name,time FROM course WHERE id=?
        "#,
        course.id.unwrap()
    )
    .fetch_one(pool)
    .await?;

    Ok(Course{
        id:Some(row.id),
        teacher_id:row.teacher_id,
        name:row.name.clone(),
        time:Some(row.time.unwrap().naive_local()),
    })
}