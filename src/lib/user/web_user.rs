use std::collections::HashMap;

use actix::Addr;
use actix_web::{web, HttpResponse, Responder};

use crate::lib::config::{get_conn, HttpServerConfig, ResponseMessage};
use crate::lib::key::{create_token_time_h, gettoken_to_user_no_time};
use crate::lib::user::email_code::GenerateCode;
use crate::lib::user::sql_user;

use super::email_code::{EmaiCodeManager, EmailCodeSend, EmailManager, VerifyCode};
// 注册用户
#[derive(serde::Deserialize,Debug)]
pub struct RegisterUser {
    pub email: String,
    pub password: String,
}

// 获取验证码
pub async fn get_code(
    emailmanager: web::Data<Addr<EmailManager>>,
    email_code_manager: web::Data<Addr<EmaiCodeManager>>,
    path_email: web::Query<HashMap<String, String>>,
) -> HttpResponse {
    let to_email_str = match path_email.get("email") {
        Some(email) => email,
        None => {
            return HttpResponse::Ok().json(ResponseMessage {
                code: 500,
                message: "null",
            });
        }
    };

    let to_email = match to_email_str.parse() {
        Ok(email) => email,
        Err(_) => {
            return HttpResponse::Ok().json(ResponseMessage {
                code: 500,
                message: "邮箱格式错误",
            })
        }
    };

    let code = email_code_manager
        .send(GenerateCode {
            email: to_email_str.to_string(),
        })
        .await
        .unwrap();

    let email_code_send = EmailCodeSend {
        code: code.clone(),
        to_email,
    };

    match emailmanager.send(email_code_send).await {
        Ok(_result) => HttpResponse::Ok().json(ResponseMessage {
            code: 200,
            message: "已发送验证码",
        }),
        Err(_) => HttpResponse::Ok().json(ResponseMessage {
            code: 500,
            message: "发送验证码失败",
        }),
    }
}

// 注册账号
pub async fn register(
    user: web::Json<RegisterUser>,
    config: web::Data<HttpServerConfig>,
    query_data: web::Query<HashMap<String, String>>,
    email_code_manager: web::Data<Addr<EmaiCodeManager>>,
) -> HttpResponse {
    let result = email_code_manager
        .send(VerifyCode {
            email: user.email.clone(),
            code: query_data.get("code").unwrap().to_string(),
        })
        .await
        .unwrap();

    match result {
        true => {
            let conn = get_conn(&config).await.unwrap();
            match sql_user::register_user(conn, &user).await {
                Ok(uid) => {
                    let token = create_token_time_h(uid, 12);
                    HttpResponse::Ok().json(ResponseMessage {
                        code: 200,
                        message: &token,
                    })
                }
                Err(_) => HttpResponse::Ok().json(ResponseMessage {
                    code: 500,
                    message: "已被注册",
                }),
            }
        }
        false => HttpResponse::Ok().json(ResponseMessage {
            code: 404,
            message: "验证码错误",
        }),
    }
}

// 登录账号
pub async fn login(
    user: web::Json<RegisterUser>,
    config: web::Data<HttpServerConfig>,
) -> HttpResponse {
    let conn = get_conn(&config).await.unwrap();

    match sql_user::login_user(conn, &user).await {
        Ok(uid) => {
            let token = create_token_time_h(uid.try_into().unwrap(), 12);
            HttpResponse::Ok().json(ResponseMessage {
                code: 200,
                message: &token,
            })
        }
        Err(_) => HttpResponse::InternalServerError().json(ResponseMessage {
            code: 500,
            message: "账号或密码错误",
        }),
    }
}

// token验证
pub async fn token_verify(query_data: web::Query<HashMap<String, String>>) -> impl Responder {
    let token = query_data.get("token").unwrap();
    match gettoken_to_user_no_time(token) {
        Ok(_) => HttpResponse::Ok().json(ResponseMessage {
            code: 200,
            message: token,
        }),
        Err(_) => HttpResponse::Ok().json(ResponseMessage {
            code: 500,
            message: token,
        }),
    }
}

// 忘记密码
pub async fn forget_password(
    user: web::Json<RegisterUser>,
    config: web::Data<HttpServerConfig>,
    query_data: web::Query<HashMap<String, String>>,
    email_code_manager: web::Data<Addr<EmaiCodeManager>>,
) -> HttpResponse {
    let result = email_code_manager
        .send(VerifyCode {
            email: user.email.clone(),
            code: query_data.get("code").unwrap().to_string(),
        })
        .await
        .unwrap();
    if result {
        let conn = get_conn(&config).await.unwrap();
        match sql_user::change_password(conn, &user).await {
            Ok(_) => HttpResponse::Ok().json(ResponseMessage {
                code: 200,
                message: "修改成功",
            }),
            Err(_) => HttpResponse::Ok().json(ResponseMessage {
                code: 500,
                message: "修改失败",
            }),
        }
    } else {
        return HttpResponse::Ok().json(ResponseMessage {
            code: 500,
            message: "验证码错误",
        });
    }
}
