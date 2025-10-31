use chrono::{Duration, Utc};
use jsonwebtoken::{DecodingKey, EncodingKey, Header, Validation, decode, encode};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Claims {
    pub sub: uuid::Uuid,
    pub token_type: String,
    pub exp: usize,
    pub iat: usize,
}

#[derive(Clone)]
pub struct JwtService {
    secret: String,
    access_expiry_minutes: i64,
    refresh_expiry_days: i64,
}

impl JwtService {
    pub const JWT_SESSION_KEY: &'static str = "jwt_token";

    pub fn new(secret: String, access_expiry_minutes: i64, refresh_expiry_days: i64) -> Self {
        Self {
            secret,
            access_expiry_minutes,
            refresh_expiry_days,
        }
    }

    pub fn from_env() -> Self {
        use std::env;

        let secret = env::var("JWT_SECRET").expect("JWT_SECRET must be set");
        let access_expiry_minutes = env::var("JWT_ACCESS_EXPIRY_MINUTES")
            .unwrap_or_else(|_| "15".to_string())
            .parse()
            .expect("JWT_ACCESS_EXPIsRY_MINUTES must be a number");

        let refresh_expiry_days = env::var("JWT_REFRESH_EXPIRY_DAYS")
            .unwrap_or_else(|_| "7".to_string())
            .parse()
            .expect("JWT_REFRESH_EXPIRY_DAYS must be a number");

        tracing::info!("JWT service initialized!");
        Self::new(secret, access_expiry_minutes, refresh_expiry_days)
    }

    pub fn generate_tokens(
        &self,
        user_id: uuid::Uuid,
    ) -> Result<(String, String), jsonwebtoken::errors::Error> {
        let now = Utc::now();

        let access_claims = Claims {
            sub: user_id.to_owned(),
            token_type: "access".to_string(),
            iat: now.timestamp() as usize,
            exp: (now + Duration::minutes(self.access_expiry_minutes)).timestamp() as usize,
        };

        let refresh_claims = Claims {
            sub: user_id.to_owned(),
            token_type: "refresh".into(),
            iat: now.timestamp() as usize,
            exp: (now + Duration::days(self.refresh_expiry_days)).timestamp() as usize,
        };

        let access_token = encode(
            &Header::default(),
            &access_claims,
            &EncodingKey::from_secret(self.secret.as_ref()),
        )?;

        let refresh_token = encode(
            &Header::default(),
            &refresh_claims,
            &EncodingKey::from_secret(self.secret.as_ref()),
        )?;

        tracing::trace!("Generated access, and refresh token for user: {}", user_id);
        Ok((access_token, refresh_token))
    }

    pub fn verify_token(&self, token: &str) -> Result<Claims, jsonwebtoken::errors::Error> {
        let token_data = decode::<Claims>(
            token,
            &DecodingKey::from_secret(self.secret.as_ref()),
            &Validation::default(),
        )?;

        tracing::trace!("verified jwt token for the user: {}", token_data.claims.sub);
        Ok(token_data.claims)
    }

    pub fn generate_access_token_from_refresh(
        &self,
        refresh_token: String,
    ) -> Result<String, jsonwebtoken::errors::Error> {
        let decoded = decode::<Claims>(
            refresh_token,
            &DecodingKey::from_secret(self.secret.as_ref()),
            &Validation::default(),
        )?;

        let claims = decoded.claims;

        let now = Utc::now();
        let exp = now + Duration::minutes(self.access_expiry_minutes);

        let new_access_claims = Claims {
            sub: claims.sub.clone(),
            token_type: "access".to_string(),
            iat: now.timestamp() as usize,
            exp: exp.timestamp() as usize,
        };

        let new_access_token = encode(
            &Header::default(),
            &new_access_claims,
            &EncodingKey::from_secret(self.secret.as_ref()),
        )?;

        tracing::info!(
            "Generated new access token from refresh token for user: {}",
            claims.sub
        );

        Ok(new_access_token)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const JWT_SECRET: &'static str = "test_secret";
    const JWT_ACCESS_EXPIRY_MINUTES: i64 = 10;
    const JWT_REFRESH_EXPIRY_DAYS: i64 = 1;
    const USER_ID_1: uuid::Uuid = uuid::Uuid::from_u128(1);

    #[test]
    fn test_generate_and_verify_access_token() {
        let jwt_service = JwtService::new(
            JWT_SECRET.to_string(),
            JWT_ACCESS_EXPIRY_MINUTES,
            JWT_REFRESH_EXPIRY_DAYS,
        );

        let (access_token, refresh_token) = jwt_service
            .generate_tokens(USER_ID_1)
            .expect("failed to generate tokens");

        let claims = jwt_service.verify_token(&access_token).unwrap();
        assert_eq!(claims.sub, USER_ID_1);
        assert_eq!(claims.token_type, "access");

        let refresh_claims = jwt_service.verify_token(&refresh_token).unwrap();
        assert_eq!(refresh_claims.sub, USER_ID_1);
        assert_eq!(refresh_claims.token_type, "refresh");
    }

    #[test]
    fn test_invalid_token() {
        let jwt_service = JwtService::new(
            JWT_SECRET.to_string(),
            JWT_ACCESS_EXPIRY_MINUTES,
            JWT_REFRESH_EXPIRY_DAYS,
        );

        let invalid_token = "invalid_token";

        let result = jwt_service.verify_token(invalid_token);
        assert!(result.is_err());
    }

    #[test]
    fn test_generate_access_from_refresh_token() {
        let jwt_service = JwtService::new(
            JWT_SECRET.to_string(),
            JWT_ACCESS_EXPIRY_MINUTES,
            JWT_REFRESH_EXPIRY_DAYS,
        );

        let (_access_token, refresh_token) = jwt_service
            .generate_tokens(USER_ID_1)
            .expect("failed to generate tokens");

        let new_access_token = jwt_service
            .generate_access_token_from_refresh(refresh_token)
            .unwrap();

        let claims = jwt_service.verify_token(&new_access_token).unwrap();
        assert_eq!(claims.sub, USER_ID_1);
        assert_eq!(claims.token_type, "access");
    }
}
