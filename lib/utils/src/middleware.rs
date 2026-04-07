use axum::{
    extract::{OriginalUri, Request},
    middleware::Next,
    response::Response,
};
use crate::{enums::Role, error::AppError, jwt::verify_jwt};

#[derive(Clone, Debug)]
pub struct AuthUser {
    pub id: i64,
}

pub async fn verify_token(mut req: Request, next: Next) -> Result<Response, AppError> {
    let auth_header = req
        .headers()
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

pub async fn verify_role(
    OriginalUri(uri): OriginalUri,
    req: Request,
    next: Next,
) -> Result<Response, AppError> {
    let path = uri.path();

    let auth_header = req
        .headers()
        .get("Authorization")
        .and_then(|h| h.to_str().ok())
        .ok_or(AppError::Unauthorized)?;

    let token = auth_header
        .strip_prefix("Bearer ")
        .ok_or(AppError::Unauthorized)?;

    let claims = verify_jwt(token)?;

    let role = claims.role;

    if path.starts_with("/client") && role != Role::Client {
        println!(
            "Unauthorized access attempt to client route with role: {:?}",
            role
        );
        return Err(AppError::Unauthorized);
    }

    if path.starts_with("/freelancer") && role != Role::Freelancer {
        println!(
            "Unauthorized access attempt to freelancer route with role: {:?}",
            role
        );
        return Err(AppError::Unauthorized);
    }

    Ok(next.run(req).await)
}
