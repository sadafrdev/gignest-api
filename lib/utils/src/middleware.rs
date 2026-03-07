use axum::{  
    extract::Request,  
    middleware::Next,  
    response::Response,  
};
use crate::{error::AppError, jwt::verify_jwt};  

pub async fn from_func(
    mut req: Request,
    next: Next,
) -> Result<Response, AppError> {
    let token = req
        .headers()
        .get("Authorization")
        .and_then(|v| v.to_str().ok())
        .and_then(|v| v.strip_prefix("Bearer "))
        .ok_or(AppError::Unauthorized)?;

    let claims = verify_jwt(token)
        .map_err(|_| AppError::InternalServerError)?;

    req.extensions_mut().insert(claims.sub);

    Ok(next.run(req).await)

}