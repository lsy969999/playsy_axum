use jsonwebtoken::{DecodingKey, EncodingKey, Header, Validation};
use serde::{de::DeserializeOwned, Serialize};
use time::{Duration, OffsetDateTime};

use crate::models::{claims::{AccessClaims, RefreshClaims}, fn_args::token::{GenAccessTokenArgs, GenRefreshTokenArgs}};

pub fn generate_jwt<T>(
    claims: &T,
    encoding_key: &EncodingKey
) -> Result<String, jsonwebtoken::errors::Error>
    where T: Serialize {
    jsonwebtoken::encode(&Header::default(), claims, encoding_key)
}

pub fn decode_jwt<T>(
    token: &str,
    decoding_key: &DecodingKey
) -> Result<T, jsonwebtoken::errors::Error>
    where T: DeserializeOwned {
    let token_data = jsonwebtoken::decode::<T>(token, decoding_key, &Validation::default())?;
    Ok(token_data.claims)
}

pub fn generate_access_token(args: GenAccessTokenArgs) -> Result<String, jsonwebtoken::errors::Error> {
    let now: OffsetDateTime = OffsetDateTime::now_utc();
    let acc_exp = *super::config::get_config_jwt_access_time();
    let access_claims = AccessClaims::new(args.user_sn, now + Duration::seconds(acc_exp), now, None, args.nick_name, args.avatar_url);
    let acc = super::config::get_config_jwt_access_keys();
    let access_token = generate_jwt(&access_claims, &acc.encoding)?;
    Ok(access_token)
}

pub fn generate_refresh_token(args: GenRefreshTokenArgs) -> Result<String, jsonwebtoken::errors::Error> {
    let now: OffsetDateTime = OffsetDateTime::now_utc();
    let refr_exp = *super::config::get_config_jwt_refresh_time();
    let refresh_claims = RefreshClaims::new(args.user_sn, now + Duration::seconds(refr_exp), now, None, args.chk);
    let refr = super::config::get_config_jwt_refresh_keys();
    let refresh_token = generate_jwt(&refresh_claims, &refr.encoding)?;
    Ok(refresh_token)
}