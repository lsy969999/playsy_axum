use serde::{Deserialize, Serialize};
use time::OffsetDateTime;

use super::entities::user::{UserSttEnum, UserTyEnum};

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
 * 
 * struct Claims {
    aud: String,         // Optional. Audience
    exp: usize,          // Required (validate_exp defaults to true in validation). Expiration time (as UTC timestamp)
    iat: usize,          // Optional. Issued at (as UTC timestamp)
    iss: String,         // Optional. Issuer
    nbf: usize,          // Optional. Not Before (as UTC timestamp)
    sub: String,         // Optional. Subject (whom token refers to)
}
 */
// #[derive(Debug, Serialize, Deserialize)]
// pub struct Claims {
//     pub sub: String,
//     pub exp: usize, // Mandatory expiry time as UTC timestamp
//     pub iat: usize,
//     pub scope: Option<String>,
// }

// impl Claims {
//     pub fn new(sub: String, exp: OffsetDateTime, iat: OffsetDateTime, scope: Option<String>) -> Self {
//         Self {
//             sub,
//             exp: exp.unix_timestamp() as usize,
//             iat: iat.unix_timestamp() as usize,
//             scope,
//         }
//     }
// }

#[derive(Debug, Serialize, Deserialize)]
pub struct AccessClaims {
    pub sub: String,
    pub exp: usize, 
    pub iat: usize,
    pub scope: Option<String>,
    pub nick_name: String,
    pub avatar_url: Option<String>,
    pub user_stt: UserSttEnum,
    pub user_ty: UserTyEnum
}

impl AccessClaims {
    pub fn new(sub: String, exp: OffsetDateTime, iat: OffsetDateTime, scope: Option<String>,  nick_name: String, avatar_url: Option<String>, user_stt: UserSttEnum, user_ty: UserTyEnum) -> Self {
        Self {
            sub,
            exp: exp.unix_timestamp() as usize,
            iat: iat.unix_timestamp() as usize,
            scope,
            nick_name,
            avatar_url,
            user_stt,
            user_ty,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RefreshClaims {
    pub sub: String,
    pub exp: usize, 
    pub iat: usize,
    pub scope: Option<String>,
    // refresh 토큰 검증 디비 식별자
    pub chk: usize,
}

impl RefreshClaims {
    pub fn new(sub: String, exp: OffsetDateTime, iat: OffsetDateTime, scope: Option<String>, chk: usize) -> Self {
        Self {
            sub,
            exp: exp.unix_timestamp() as usize,
            iat: iat.unix_timestamp() as usize,
            scope,
            chk
        }
    }
}