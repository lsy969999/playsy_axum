use serde::{Deserialize, Serialize};
use time::OffsetDateTime;
use validator::Validate;

#[derive(Debug, Deserialize, Validate)]
pub struct AuthPayload {
    #[validate(email(message="must email"))]
    pub email: String,
    #[validate(length(min = 8, message = "password min length is 8"))]
    pub password: String,
}

/**
 * sub (Subject): 토큰의 주체, 일반적으로 사용자 ID
 * iss (Issuer): 토큰을 발행한 주체
 * aud (Audience): 토큰의 수신자, 즉 이 토큰을 사용할 수 있는 대상
 * exp (Expiration Time): 토큰의 만료 시간
 * iat (Issued At): 토큰이 발행된 시간
 * nbf (Not Before): 토큰이 활성화되는 시간
 * jti (JWT ID): 토큰의 고유 식별자
 * scope: 권한 또는 역할 정보
 * email: 사용자 이메일 (선택적)
 * name: 사용자 이름 (선택적)
 */
#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,
    pub exp: usize, // Mandatory expiry time as UTC timestamp
    pub iat: usize,
    pub scope: Option<String>,
}

impl Claims {
    pub fn new(sub: String, exp: OffsetDateTime, iat: OffsetDateTime, scope: Option<String>) -> Self {
        Self {
            sub,
            exp: exp.unix_timestamp() as usize,
            iat: iat.unix_timestamp() as usize,
            scope,
        }
    }
}

#[derive(Debug, Serialize)]
pub struct AuthBody {
    pub access_token: String,
    pub token_type: String,
}

impl AuthBody {
    pub fn new(access_token: String) -> Self {
        Self {
            access_token,
            token_type: "Bearer".to_string(),
        }
    }
}
