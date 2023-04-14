use crate::models::course::*;
use chrono::{NaiveDateTime,DateTime,Local,Utc};
use sqlx::{mysql::MySqlPool,MySql,QueryBuilder};
use crate::errors::MyError;


pub async fn get_courses_for_teacher_db(
    pool:&MySqlPool,
    teacher_id:i32,
)->Result<Vec<Course>,MyError>{
    //如果调用的是query_as!，需要为Row手动实现FromRow而不是为MySqlRow实现FromRow    
    let rows:Vec<Course>=sqlx::query_as::<_,Course>(
        r#"
        SELECT * FROM course WHERE teacher_id=?
        "#
    ).bind(teacher_id)
    .fetch_all(pool)
    .await?;
    
    Ok(rows)
}

pub async fn get_course_detail_db(
    pool:&MySqlPool,
    teacher_id:i32,
    course_id:i32,
)->Result<Course,MyError>{
    let row=sqlx::query_as::<_,Course>(
        r#"
        SELECT * FROM course WHERE teacher_id=? AND id=?
        "#
    )
    .bind(teacher_id)
    .bind(course_id)
    .fetch_optional(pool)
    .await?;
    
    if let Some(course)=row{
        Ok(course)
    }else{
        Err(MyError::NotFound("Course Id not found".into()))
    }
}

pub async fn post_new_course_db(
    pool:&MySqlPool,
    course:CreateCourse,
)->Result<Course,MyError>{
    let mut qb:QueryBuilder<MySql>=QueryBuilder::new(
        "INSERT INTO course(teacher_id,name,description,format,structure,duration,price,language,level) VALUES("
    );
    let mut separate=qb.separated(", ");
    separate.push_bind(course.teacher_id)
    .push_bind(course.name)
    .push_bind(course.description)
    .push_bind(course.format)
    .push_bind(course.structure)
    .push_bind(course.duration)
    .push_bind(course.price)
    .push_bind(course.language)
    .push_bind(course.level);
    separate.push_unseparated(")");
    let query_result=qb.build()
    .execute(pool)
    .await?;

    //last_insert_id()在高并发下可能会出现问题，因为它是从连接中获取的，而不是从事务中获取的
    //如果需要保证准确可以采用事务，事务最后select语句查询Last_Insert_Id，一般不在插入后直接返回插入的这条数据
    let insert_id=query_result.last_insert_id();
    let row=sqlx::query_as::<_,Course>(
        r#"
        SELECT * FROM course WHERE id=?
        "#,
    )
    .bind(insert_id)
    .fetch_optional(pool)
    .await?;

    if let Some(course)=row{
        Ok(course)
    }else{
        Err(MyError::NotFound("Course Id not found".into()))
    }
}

pub async fn delete_course_db(
    pool:&MySqlPool,
    teacher_id:i32,
    course_id:i32,
)->Result<String,MyError>{
    let row=sqlx::query!(
        "delete from course where teacher_id=? and id=?",teacher_id,course_id
    ).execute(pool)
    .await?;

    Ok(format!("deleted {:?} record",row))
}

pub async fn update_course_db(
    pool:&MySqlPool,
    teacher_id:i32,
    course_id:i32,
    update_course:UpdateCourse,
)->Result<Course,MyError>{
    let current_course_row=sqlx::query_as::<_,Course>(
        r#"
        SELECT * FROM course WHERE teacher_id=? AND id=?
        "#
    )
    .bind(teacher_id)
    .bind(course_id)
    .fetch_optional(pool)
    .await
    .map_err(|_err| MyError::NotFound("Course Id not found".into()))?;

    let mut tx=pool.begin().await?;
    let current_course_now;
    match current_course_row{
        Some(v)=>current_course_now=v,
        None=>return Err(MyError::NotFound("Can't find this course!".into())),
    } 
    let name:String=if let Some(name)=update_course.name{
        name
    }else{
        //unwrap_or_default()方法返回Option<T>的值，如果是None则返回默认值,默认值为空时用原来的值
        current_course_now.name
    };
    let description:String=if let Some(description)=update_course.description{
        description
    }else{
        current_course_now.description.unwrap_or_default()
    };
    let format:String=if let Some(format)=update_course.format{
        format
    }else{
        current_course_now.format.unwrap_or_default()
    };
    let structure:String=if let Some(structure)=update_course.structure{
        structure
    }else{
        current_course_now.structure.unwrap_or_default()
    };
    let duration:String=if let Some(duration)=update_course.duration{
        duration
    }else{
        current_course_now.duration.unwrap_or_default()
    };
    let price:i32=if let Some(price)=update_course.price{
        price
    }else{
        current_course_now.price.unwrap_or_default()
    };
    let language:String=if let Some(language)=update_course.language{
        language
    }else{
        current_course_now.language.unwrap_or_default()
    };
    let level:String=if let Some(level)=update_course.level{
        level
    }else{
        current_course_now.level.unwrap_or_default()
    };

    let mut qb:QueryBuilder<MySql>=QueryBuilder::new(
        "UPDATE course SET "
    );
    qb.push("name=").push_bind(name).push(",")
    .push("description=").push_bind(description).push(",")
    .push("format=").push_bind(format).push(",")
    .push("structure=").push_bind(structure).push(",")
    .push("duration=").push_bind(duration).push(",")
    .push("price=").push_bind(price).push(",")
    .push("language=").push_bind(language).push(",")
    .push("level=").push_bind(level).push(" WHERE teacher_id=").push_bind(teacher_id).push(" AND id=").push_bind(course_id);

    let result1=qb.build()
    .execute(&mut tx)
    .await?;

    let row=sqlx::query_as::<_,Course>(
        r#"
        SELECT * FROM course WHERE id=?
        "#,
    )
    .bind(course_id)
    .fetch_optional(&mut tx)
    .await?;

    if result1.rows_affected()>0{
        tx.commit().await?;
        if let Some(course)=row{
            Ok(course)
        }else{
            Err(MyError::NotFound("Course Id not found".into()))
        }
    }else{
        tx.rollback().await?;
        Err(MyError::NotFound("Update failed!".into()))
    }
}