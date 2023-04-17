use std::result;

use crate::errors::MyError;
use crate::models::teacher::{Teacher,CreateTeacher,UpdateTeacher};
use sqlx::{mysql::MySqlPool,QueryBuilder,MySql};

pub async fn get_all_teachers_db(
    pool:&MySqlPool,
)->Result<Vec<Teacher>,MyError>{
    //这里用query!()，而不是query_as!()，因为实际生产环境中，数据库中的字段名和结构体中的字段名不一定一一对应
    let rows=sqlx::query!(
        "select id,name,picture_url,profile from teacher"
    ).fetch_all(pool)
    .await?;

    let teachers:Vec<Teacher>=rows
    .iter()
    .map(|row|{
        Teacher{
            id:row.id,
            name:row.name.clone(),
            picture_url:row.picture_url.clone().unwrap_or_default(),
            profile:row.profile.clone().unwrap_or_default(),
        }
    })
    .collect();

    match teachers.len(){
        0=>Err(MyError::NotFound("No teacher found".into())),
        _=>Ok(teachers),
    }
}

pub async fn get_teacher_details_db(
    pool:&MySqlPool,
    id:u32,
)->Result<Teacher,MyError>{
    let row=sqlx::query!(
        "select id,name,picture_url,profile from teacher where id=?",
        id
    ).fetch_one(pool)
    .await
    .map(|row|{
        Teacher{
            id:row.id,
            name:row.name,
            picture_url:row.picture_url.unwrap_or_default(),
            profile:row.profile.unwrap_or_default(),
        }
    })
    .map_err(|e|{
        MyError::NotFound(format!("Teacher with id {} not found",id))
    })?;//?得到Result中包含的Teacher

    Ok(row)
}

pub async fn post_new_teacher_db(
    pool:&MySqlPool,
    teacher:CreateTeacher,
)->Result<Teacher,MyError>{
    let mut qb:QueryBuilder<MySql>=QueryBuilder::new(
        "INSERT INTO teacher(name,picture_url,profile) VALUES("
    );
    let mut separate=qb.separated(", ");
    separate.push_bind(teacher.name)
    .push_bind(teacher.picture_url)
    .push_bind(teacher.profile);
    separate.push_unseparated(")");
    let mut tx=pool.begin().await?;
    let result1=qb.build().execute(&mut tx).await?;
    let result2=sqlx::query!(
        "SELECT LAST_INSERT_ID() as id from teacher"
    ).fetch_one(&mut tx)
    .await
    //这里使用？会在结果为Err时立刻返回错误，而不会继续执行后面的语句
    .map_err(|e| MyError::NotFound("Inserted teacher not found!".into()))?;
    let result3=sqlx::query!(
        "SELECT id,name,picture_url,profile from teacher where id=?",
        result2.id
    ).fetch_optional(&mut tx)
    .await
    .map_err(|e| MyError::NotFound("Inserted teacher not found!".into()))?;
    if result1.rows_affected()==1{
        tx.commit().await?;
        if let Some(teacher)=result3{
            Ok(Teacher{
                id:teacher.id,
                name:teacher.name,
                picture_url:teacher.picture_url.unwrap_or_default(),
                profile:teacher.profile.unwrap_or_default(),
            })
        }else{
            Err(MyError::NotFound("Insert failed!".into()))
        }
    }else{
        tx.rollback().await?;
        Err(MyError::NotFound("Insert failed!".into()))
    }   
}

pub async fn update_teacher_db(
    pool:&MySqlPool,
    id:u32,
    teacher:UpdateTeacher,
)->Result<Teacher,MyError>{
    let mut qb:QueryBuilder<MySql>=QueryBuilder::new(
        "UPDATE teacher SET "
    );
    let mut separate=qb.separated(", ");
    if let Some(name)=teacher.name{
        separate.push("name=").push_bind_unseparated(name);
    }
    if let Some(picture_url)=teacher.picture_url{
        separate.push("picture_url=").push_bind_unseparated(picture_url);
    }
    if let Some(profile)=teacher.profile{
        separate.push("profile=").push_bind_unseparated(profile);
    }
    separate.push_unseparated(" WHERE id=").push_bind_unseparated(id);
    let mut tx=pool.begin().await?;
    let result=qb.build().execute(&mut tx).await?;
    let result2=sqlx::query!(
        "SELECT id,name,picture_url,profile from teacher where id=?",
        id
    ).fetch_optional(&mut tx)
    .await
    .map_err(|e| MyError::NotFound("Updated teacher not found!".into()))?;
    if result.rows_affected()==1{
        tx.commit().await?;
        if let Some(teacher)=result2{
            Ok(Teacher{
                id:id,
                name:teacher.name,
                picture_url:teacher.picture_url.unwrap_or_default(),
                profile:teacher.profile.unwrap_or_default(),
            })
        }else{
            Err(MyError::NotFound("Update failed!".into()))
        }
    }else{
        tx.rollback().await?;
        Err(MyError::NotFound("Update failed!".into()))
    }   
}

pub async fn delete_teacher_db(
    pool:&MySqlPool,
    id:u32,
)->Result<String,MyError>{
    let result=sqlx::query!(
        "DELETE FROM teacher WHERE id=?",
        id
    ).execute(pool)
    .await
    .map_err(|e| MyError::DBError("Unable to delete teacher!".into()))?;

    Ok(format!("Deleted {:?} record",result))
}