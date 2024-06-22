// 对acl进行操作
// 添加资源->Result<ConnectionType, sqlx::Error>
// 添加用户对资源的操作->Result<ConnectionType, sqlx::Error>
// 移除用户对资源的操作->Result<ConnectionType, sqlx::Error>
// 获取用户对资源的操作->Vec<Operation>
// 获取资源id->i64

use std::collections::HashMap;

use actix_web::{http::header::AUTHORIZATION, web, HttpRequest, HttpResponse};

use crate::lib::{
    acl::sql_acl::remove_resource,
    config::{get_conn, HttpServerConfig, ResponseMessage},
    key::gettoken_to_user_no_time,
    user::sql_user::get_user_id,
};

use super::{
    check_acl,
    sql_acl::{
        add_acl, add_resource, get_acl, get_all_resource, get_resource_id, remove_acl,
        remove_acl_by_resource_id, Operation, Resource,
    },
};

// 获取所有资源
pub async fn acl_get_all_resource(
    config: web::Data<HttpServerConfig>,
    req: HttpRequest,
) -> HttpResponse {
    let token = req.headers().get(AUTHORIZATION).unwrap().to_str().unwrap();
    let conn = get_conn(&config).await.unwrap();
    let resource_id = get_resource_id(conn, &Resource::default()).await.unwrap();
    let conn = get_conn(&config).await.unwrap();
    match check_acl(
        conn,
        token,
        resource_id,
        &crate::lib::acl::Operation::Check.to_string(),
    )
    .await
    {
        Ok(_) => {
            let conn = get_conn(&config).await.unwrap();
            let resources = get_all_resource(conn).await.unwrap();
            HttpResponse::Ok().json(resources)
        }
        Err(_err) => match _err {
            super::AclError::NotFound => HttpResponse::Ok().json(ResponseMessage {
                code: 404,
                message: "Not Found",
            }),
            super::AclError::InvalidPermission => HttpResponse::Ok().json(ResponseMessage {
                code: 403,
                message: "Invalid Permission",
            }),
            super::AclError::ExpiredVerification => HttpResponse::Ok().json(ResponseMessage {
                code: 401,
                message: "Expired Verification",
            }),
        },
    }
}

// 添加资源
pub async fn acl_add_resource(
    config: web::Data<HttpServerConfig>,
    req: HttpRequest,
    resource_query: web::Query<HashMap<String, String>>,
) -> HttpResponse {
    let token = req.headers().get(AUTHORIZATION).unwrap().to_str().unwrap();
    let resource = &Resource::default();
    let name = &resource_query.get(resource).unwrap();

    let conn = get_conn(&config).await.unwrap();
    let resource_id = get_resource_id(conn, resource).await.unwrap();
    let conn = get_conn(&config).await.unwrap();
    match check_acl(
        conn,
        token,
        resource_id,
        &crate::lib::acl::Operation::Add.to_string(),
    )
    .await
    {
        Ok(_) => {
            let conn = get_conn(&config).await.unwrap();
            match add_resource(conn, name).await {
                Ok(_) => HttpResponse::Ok().json(ResponseMessage {
                    code: 200,
                    message: "Success",
                }),
                Err(_err) => HttpResponse::Ok().json(ResponseMessage {
                    code: 500,
                    message: "请勿重复添加",
                }),
            }
        }
        Err(_err) => HttpResponse::Ok().json(_err),
    }
}

// 删除资源
pub async fn acl_delete_resource(
    config: web::Data<HttpServerConfig>,
    req: HttpRequest,
    resource_query: web::Query<HashMap<String, String>>,
) -> HttpResponse {
    let token = req.headers().get(AUTHORIZATION).unwrap().to_str().unwrap();
    let resource = &Resource::default();

    let name = &resource_query.get(resource).unwrap();
    let conn = get_conn(&config).await.unwrap();
    let remove_resource_id = match get_resource_id(conn, name).await {
        Ok(id) => id,
        Err(_) => {
            return HttpResponse::Ok().json(ResponseMessage {
                code: 404,
                message: "Not Found",
            });
        }
    };

    let conn = get_conn(&config).await.unwrap();
    let resource_id = get_resource_id(conn, resource).await.unwrap();

    let conn = get_conn(&config).await.unwrap();
    match check_acl(
        conn,
        token,
        resource_id,
        &crate::lib::acl::Operation::Remove.to_string(),
    )
    .await
    {
        Ok(_) => {
            let conn = get_conn(&config).await.unwrap();
            remove_resource(conn, name).await.err();
            let conn = get_conn(&config).await.unwrap();
            match remove_acl_by_resource_id(conn, remove_resource_id).await {
                Ok(_) => HttpResponse::Ok().json(ResponseMessage {
                    code: 200,
                    message: "Success",
                }),
                Err(_err) => HttpResponse::Ok().json(ResponseMessage {
                    code: 500,
                    message: "删除失败",
                }),
            }
        }
        Err(_err) => HttpResponse::Ok().json(ResponseMessage {
            code: 500,
            message: &_err.to_string(),
        }),
    }
}

// 添加用户对资源的操作
pub async fn acl_add_user_operation(
    config: web::Data<HttpServerConfig>,
    req: HttpRequest,
    resource_query: web::Query<HashMap<String, String>>,
) -> HttpResponse {
    let token = req.headers().get(AUTHORIZATION).unwrap().to_str().unwrap();
    let resource = &Resource::default();
    let name = &resource_query.get(resource).unwrap();
    let conn = get_conn(&config).await.unwrap();
    let name_resource_id = get_resource_id(conn, name).await.unwrap();

    let email = resource_query.get("email").unwrap();
    let conn = get_conn(&config).await.unwrap();
    let uid = get_user_id(conn, email).await.unwrap();

    let operation = resource_query.get("operation").unwrap();

    let conn = get_conn(&config).await.unwrap();
    let resource_id = get_resource_id(conn, resource).await.unwrap();

    let conn = get_conn(&config).await.unwrap();
    match check_acl(
        conn,
        token,
        resource_id,
        &crate::lib::acl::Operation::Remove.to_string(),
    )
    .await
    {
        Ok(_) => {
            let conn = get_conn(&config).await.unwrap();
            match add_acl(conn, uid, name_resource_id, &Operation::from_string(operation)).await {
                Ok(_) => HttpResponse::Ok().json(ResponseMessage {
                    code: 200,
                    message: "Success",
                }),
                Err(_err) => HttpResponse::Ok().json(ResponseMessage {
                    code: 500,
                    message: "请勿重复添加",
                }),
            }
        }
        Err(_err) => HttpResponse::Ok().json(ResponseMessage {
            code: 500,
            message: &_err.to_string(),
        }),
    }
}

// 移除用户对资源的操作
pub async fn acl_remove_user_operation(
    config: web::Data<HttpServerConfig>,
    req: HttpRequest,
    resource_query: web::Query<HashMap<String, String>>,
) -> HttpResponse {
    let token = req.headers().get(AUTHORIZATION).unwrap().to_str().unwrap();
    let resource = &Resource::default();
    let name = &resource_query.get(resource).unwrap();

    let email = resource_query.get("email").unwrap();
    let conn = get_conn(&config).await.unwrap();
    let uid = get_user_id(conn, email).await.unwrap();

    let operation = resource_query.get("operation").unwrap();

    let conn = get_conn(&config).await.unwrap();
    let resource_id = get_resource_id(conn, resource).await.unwrap();

    let conn = get_conn(&config).await.unwrap();
    match check_acl(
        conn,
        token,
        resource_id,
        &crate::lib::acl::Operation::Remove.to_string(),
    )
    .await
    {
        Ok(_) => {
            let conn = get_conn(&config).await.unwrap();
            let name_resource_id = get_resource_id(conn, name).await.unwrap();
            let conn = get_conn(&config).await.unwrap();
            match remove_acl(conn, uid, name_resource_id, operation).await {
                Ok(_) => HttpResponse::Ok().json(ResponseMessage {
                    code: 200,
                    message: "Success",
                }),
                Err(_err) => HttpResponse::Ok().json(ResponseMessage {
                    code: 500,
                    message: "删除失败",
                }),
            }
        }
        Err(_err) => HttpResponse::Ok().json(ResponseMessage {
            code: 500,
            message: &_err.to_string(),
        }),
    }
}
