use axum::{  
    extract::Request,  
    middleware::Next,  
    response::Response,  
};
use crate::{error::AppError, jwt::verify_jwt};
 
#[derive(Clone, Debug)]
pub struct AuthUser {
    pub id: i64,
}

pub async fn verify_token(
    mut req: Request,
    next: Next,
) -> Result<Response, AppError> {

    let auth_header = req.headers()
        .get("Authorization")
        .and_then(|h| h.to_str().ok())
        .ok_or(AppError::Unauthorized)?;

    let token = auth_header
        .strip_prefix("Bearer ")
        .ok_or(AppError::Unauthorized)?;

    let claims = verify_jwt(token)?;

    let current_user = AuthUser { id: claims.sub };

    req.extensions_mut().insert(current_user);
    
    Ok(next.run(req).await)
}