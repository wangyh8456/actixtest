use actix_web::{error,http::StatusCode,HttpResponse,Result};
use serde::Serialize;
use sqlx::error::Error as SqlxError;
use std::fmt;

#[derive(Debug,Serialize)]
pub enum MyError{
    DBError(String),
    ActixError(String),
    NotFound(String),
    InvalidError(String),
}

#[derive(Debug,Serialize)]
pub struct MyErrorResponse{
    error_message:String,
}

impl MyError{
    fn error_response(&self)->String{
        match self{
            MyError::DBError(msg)=>{
                println!("Database error occured: {:?}",msg);
                "Database error".into()
            }
            MyError::ActixError(msg)=>{
                println!("Server error occured: {:?}",msg);
                "Internal server error".into()
            }
            MyError::NotFound(msg)=>{
                println!("Not found error occured: {:?}",msg);
                msg.into()
            }
            MyError::InvalidError(msg)=>{
                println!("Invalid error occured: {:?}",msg);
                msg.into()
            }
        }
    }
}

impl error::ResponseError for MyError{
    fn status_code(&self)->StatusCode{
        match self{
            MyError::DBError(_)|MyError::ActixError(_)=>StatusCode::INTERNAL_SERVER_ERROR,
            MyError::NotFound(_)=>StatusCode::NOT_FOUND,
            MyError::InvalidError(_)=>StatusCode::BAD_REQUEST,
        }
    }
    fn error_response(&self)->HttpResponse{
        HttpResponse::build(self.status_code()).json(MyErrorResponse{
            //此处的error_response()方法是MyError的方法
            error_message:self.error_response(),
        })
    }
}

impl fmt::Display for MyError{
    fn fmt(&self,f:&mut fmt::Formatter)->Result<(),fmt::Error>{
        write!(f,"{}",self)
    }
}

impl From<SqlxError> for MyError{
    fn from(error:SqlxError)->Self{
        MyError::DBError(error.to_string())
    }
}

impl From<actix_web::error::Error> for MyError{
    fn from(error:actix_web::error::Error)->Self{
        MyError::ActixError(error.to_string())
    }
}