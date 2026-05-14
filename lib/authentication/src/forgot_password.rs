use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use sqlx::FromRow;
use utils::{
    db::DB,
    encryption::ResetTokenClaims,
    encryption::{hashing, otp, send_email},
    error::AppError,
};

#[derive(Deserialize, Debug, Serialize, FromRow)]
pub struct SendOtp {
    pub email: String,
}

impl SendOtp {
    pub async fn send_otp(
        self,
        db: DB,
        sendgrid_api_key: &str,
        from_email: &str,
    ) -> Result<(), AppError> {
        let hashed_otp = hashing(otp());
        let email = &self.email.clone();

        sqlx::query!(" SELECT email FROM users WHERE email = $1 ", self.email)
            .fetch_optional(&db)
            .await?
            .ok_or(AppError::NotFound("EMAIL".to_string()))?;

        sqlx::query!(
            "
                INSERT INTO otps (email, otp_hash, purpose, created_at, expires_at)
                VALUES (
                    $1, $2, 'password_reset', NOW(), NOW() + INTERVAL '10 minutes'
                )
            ",
            email,
            hashed_otp
        )
        .execute(&db)
        .await?;

        send_email(email, otp(), sendgrid_api_key, from_email).await;

        Ok(())
    }
}

#[derive(Deserialize, Debug, Serialize, FromRow)]
pub struct VerifyOtp {
    pub otp: i32,
    pub email: String,
}

#[derive(Debug, Serialize)]
pub struct VerifyOtpResponse {
    pub reset_token: String,
}

impl VerifyOtp {
    pub async fn verify_otp(self, db: DB) -> Result<VerifyOtpResponse, AppError> {
        let email = self.email.clone();

        let otp_str = format!("{:06}", self.otp);
        let otp_hash = format!("{:x}", Sha256::digest(otp_str.as_bytes()));

        sqlx::query!(
            "
                SELECT email
                FROM otps
                WHERE email = $1
                    AND otp_hash = $2
                    AND purpose = 'password_reset'
                    AND expires_at > now()
            ",
            self.email,
            otp_hash
        )
        .fetch_optional(&db)
        .await?
        .ok_or(AppError::Unauthorized)?;

        sqlx::query!(
            " DELETE FROM otps WHERE email = $1 AND purpose = 'password_reset' ",
            self.email
        )
        .execute(&db)
        .await?;

        let reset_token = ResetTokenClaims::generate_reset_token(&email)
            .map_err(|_| AppError::InternalServerError)?;

        return Ok(VerifyOtpResponse { reset_token });
    }
}

#[derive(Deserialize, Debug, Serialize, FromRow)]
pub struct UpdatePassword {
    pub email: String,
    pub new_password: String,
    pub token: String,
}

#[derive(Debug, Serialize)]
pub struct UpdatePasswordResponse {
    pub message: &'static str,
}

impl UpdatePassword {
    pub async fn update_password(self, db: DB) -> Result<UpdatePasswordResponse, AppError> {
        //Verifying Token
        ResetTokenClaims::verify_reset_token(&self.token).await?;

        //Updating Password
        sqlx::query!(
            " UPDATE users SET password = $1 WHERE email = $2 ",
            self.new_password,
            self.email
        )
        .execute(&db)
        .await?;

        Ok(UpdatePasswordResponse {
            message: "Password updated successfully",
        })
    }
}
