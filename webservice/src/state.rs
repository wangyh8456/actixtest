use std::sync::Mutex;
// use super::models::Course;
use sqlx::mysql::MySqlPool;

pub struct AppState{
    //两个字段均可在线程之间共享
    pub health_check_state:String,
    //Mutex保证了线程在修改数据之前必须获得控制权，确保安全性
    pub visit_count:Mutex<u32>,
    // pub courses:Mutex<Vec<Course>>,
    pub mb_pool:MySqlPool,
}