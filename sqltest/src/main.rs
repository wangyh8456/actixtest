use chrono::{NaiveDateTime, DateTime, Utc,Local,FixedOffset};
use dotenv::dotenv;
use sqlx::{mysql::MySqlPoolOptions, Row,MySql,QueryBuilder,Execute};
use std::{env,io};

#[derive(Debug,Clone)]
pub struct Course{
    pub id:u64,
    pub teacher_id:i32,
    pub name:String,
    pub time:Option<NaiveDateTime>,
}

#[actix_rt::main]
async fn main()->io::Result<()>{
    //ok():如果dotenv::dotenv()返回的是Err，则忽略该错误,否则返回Option
    dotenv().ok();
    let database_url=env::var("DATABASE_URL").expect("DATABASE_URL must be set!!!");
    let pool=MySqlPoolOptions::new().max_connections(5).connect(&database_url).await.unwrap();
    // let course_rows=sqlx::query("select * from course where id=?").bind(2).fetch_all(&pool).await.unwrap();
    let mut qb:QueryBuilder<MySql>=QueryBuilder::new(
        "select * from course where id="
    );
    qb.push_bind(2).push(" and teacher_id=").push_bind(1);
    let course_rows=qb.build().fetch_all(&pool).await.unwrap();
    let beijing_offset=FixedOffset::east_opt(8*3600).unwrap();
    let mut course_list=vec![];
    for course_row in course_rows{
        course_list.push(Course{
            id:course_row.get(0),
            teacher_id:course_row.get(1),
            name:course_row.get(2),
            time:Some(course_row.try_get::<DateTime<Utc>,usize>(3).unwrap().with_timezone(&beijing_offset).naive_local()),
        });
        // println!("{:?}",chrono::NaiveDateTime::from(course_row.try_get::<DateTime<Utc>,usize>(3).unwrap().naive_utc()));
        print!("{}\r\n",course_row.try_get::<DateTime<Utc>,usize>(3).unwrap().with_timezone(&beijing_offset).format("%Y-%m-%d %H:%M:%S").to_string());
    }
    println!("{:?}",course_list);
    Ok(())
}

// async fn main()->io::Result<()>{
//     //ok():如果dotenv::dotenv()返回的是Err，则忽略该错误,否则返回Option
//     dotenv().ok();
//     let database_url=env::var("DATABASE_URL").expect("DATABASE_URL must be set!!!");
//     let pool=MySqlPoolOptions::new().max_connections(5).connect(&database_url).await.unwrap();
//     let course_rows=sqlx::query!(r#"select * from course where id=? and teacher_id=?"#,2,"3;select * from course;").fetch_all(&pool).await.unwrap();
//     // let course_rows=sqlx::query("select * from course where id=?").bind(2).fetch_all(&pool).await.unwrap();
//     let mut course_list=vec![];
//     for course_row in course_rows{
//         course_list.push(Course{
//             id:course_row.id,
//             teacher_id:course_row.teacher_id,
//             name:course_row.name,
//             time:Some(chrono::NaiveDateTime::from(course_row.time.unwrap().naive_utc())),
//         });
//     }
//     println!("{:?}",course_list);
//     Ok(())
// }
