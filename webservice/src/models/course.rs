use actix_web::web;
use chrono::{NaiveDateTime,Local,DateTime,Utc};
use serde::{Deserialize, Serialize};
use sqlx::{FromRow,mysql::MySqlRow,Row};
//use crate::models::course::Course;
use crate::errors::MyError;
use std::convert::TryFrom;

//FromRow:从数据库中读取数据，转换为结构体
//Deserialize:从json中读取数据，转换为结构体,Serialize:将结构体转换为json
#[derive( Serialize, Debug, Clone)]
pub struct Course{
    pub teacher_id:i32,
    pub id:u64,
    pub name:String,
    pub time:Option<NaiveDateTime>,
    //都是Option类型，因为有些字段可能为空  
    pub description:Option<String>,
    pub format:Option<String>,
    pub structure:Option<String>,
    pub duration:Option<String>,
    pub price:Option<i32>,
    pub language:Option<String>,
    pub level:Option<String>,
}

impl<'a> FromRow<'a,MySqlRow> for Course{
    fn from_row(row: &MySqlRow) -> sqlx::Result<Self> {
        Ok(Course{
            teacher_id:row.get("teacher_id"),
            id:row.get("id"),
            name:row.get::<String,&str>("name").clone(),
            time:Some(row.get::<DateTime<Utc>,&str>("time").with_timezone(&Local).naive_local()),
            //当查询结果为Null时，Ok(None)会被？解析为None，否则直接返回值
            //这里不需要使用clone()，因为Option类型的字段都是Copy类型
            description:row.try_get::<Option<String>,&str>("description")?,
            format:row.try_get::<Option<String>,&str>("format")?,
            structure:row.try_get::<Option<String>,&str>("structure")?,
            duration:row.try_get::<Option<String>,&str>("duration")?,
            price:row.try_get::<Option<i32>,&str>("price")?,
            language:row.try_get::<Option<String>,&str>("language")?,
            level:row.try_get::<Option<String>,&str>("level")?,
        })
    }
}

#[derive(Deserialize, Debug, Clone)]
pub struct CreateCourse{
    pub teacher_id:i32,
    pub name:String,
    //都是Option类型，因为有些字段可能为空  
    pub description:Option<String>,
    pub format:Option<String>,
    pub structure:Option<String>,
    pub duration:Option<String>,
    pub price:Option<i32>,
    pub language:Option<String>,
    pub level:Option<String>,
}

// impl From<web::Json<Course>> for CreateCourse{
//     fn from(course:web::Json<Course>) -> Self {
//         CreateCourse{
//             teacher_id:course.teacher_id,
//             name:course.name.clone(),
//             description:course.description.clone(),
//             format:course.format.clone(),
//             structure:course.structure.clone(),
//             duration:course.duration.clone(),
//             price:course.price,
//             language:course.language.clone(),
//             level:course.level.clone(),
//         }
//     }
// }

impl TryFrom<web::Json<CreateCourse>> for CreateCourse{
    type Error=MyError;

    fn try_from(course:web::Json<CreateCourse>) -> Result<Self, Self::Error> {
        Ok(CreateCourse{
            teacher_id:course.teacher_id,
            name:course.name.clone(),
            description:course.description.clone(),
            format:course.format.clone(),
            structure:course.structure.clone(),
            duration:course.duration.clone(),
            price:course.price,
            language:course.language.clone(),
            level:course.level.clone(),
        })
    }
}

#[derive(Deserialize, Debug, Clone)]
pub struct UpdateCourse{
    //修改课程信息，不修改teacher_id
    pub name:Option<String>, 
    pub description:Option<String>,
    pub format:Option<String>,
    pub structure:Option<String>,
    pub duration:Option<String>,
    pub price:Option<i32>,
    pub language:Option<String>,
    pub level:Option<String>,
}

impl From<web::Json<UpdateCourse>> for UpdateCourse{
    fn from(course:web::Json<UpdateCourse>) -> Self {
        UpdateCourse{
            name:course.name.clone(),
            description:course.description.clone(),
            format:course.format.clone(),
            structure:course.structure.clone(),
            duration:course.duration.clone(),
            price:course.price,
            language:course.language.clone(),
            level:course.level.clone(),
        }
    }
}