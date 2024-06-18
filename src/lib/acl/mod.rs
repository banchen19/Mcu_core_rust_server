use serde::Serialize;
use sql_acl::{get_acl, Operation};

use super::config::ConnectionType;

pub mod sql_acl;
pub mod web_acl;

#[derive(Debug, Serialize)]
pub enum AclError {
    // 未知错误
    NotFound,
    // 无效的权限
    InvalidPermission,
    // 过期验证
    ExpiredVerification,
}

impl AclError {
    pub fn to_string(&self) -> String {
        match self {
            AclError::NotFound => "Not Found".to_string(),
            AclError::InvalidPermission => "Invalid Permission".to_string(),
            AclError::ExpiredVerification => "Expired Verification".to_string(),
        }
    }
}

/// 检查权限
/// # 参数
/// * `conn` - 数据库连接
/// * `token` - token
/// * `resource_id` - 资源id
/// * `acl` - 权限
/// # 返回
/// * `bool` - 是否有权限
pub async fn check_acl(
    conn: ConnectionType,
    token: &str,
    resource_id: i64,
    operation_str: &str,
) -> Result<(), AclError> {
    match crate::lib::key::gettoken_to_user_no_time(token) {
        Ok(user) => {
            let operation = Operation::from_string(operation_str);
            match get_acl(conn, user.claims.uid, resource_id).await {
                Ok(operations) => {
                    if Operation::get_operation(operations, &operation) {
                        return Ok(());
                    } else {
                        return Err(AclError::InvalidPermission);
                    }
                }
                Err(_) => {
                    return Err(AclError::NotFound);
                }
            }
        }
        Err(_) => {
            return Err(AclError::ExpiredVerification);
        }
    }
}
