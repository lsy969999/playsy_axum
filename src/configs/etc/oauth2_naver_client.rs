/**
 * 사용중인 Oauth2 라이브러리에서 제공하는 BasicClient는 
 * Oauth2 사앙에 맞게 개발되어있는데
 * 네이버의 경우 토큰발급 바디에 expires가 number가 아닌 string으로 넘겨서
 * Oauth2 사양을 준수하지 않기때문에 기존 BasicClient를 생으로 쓸수가 없다.
 * 따라서 expires 를 string 으로 파싱되도록 Client 재정의를 하엿고
 * 
 * 참고 깃헙은 아래와 같다.
 * https://github.com/ramosbugs/oauth2-rs/issues/191
 */

use std::time::Duration;

use oauth2::{Client, basic::{BasicErrorResponse, BasicRevocationErrorResponse, BasicTokenIntrospectionResponse, BasicTokenType}, helpers, AccessToken, EmptyExtraTokenFields, ExtraTokenFields, RefreshToken, Scope, StandardRevocableToken, StandardTokenResponse, TokenResponse, TokenType};

use serde::{Deserialize, Serialize};
///
/// Custom Token Response type to replace the StandardTokenResponse provided by oauth2-rs. This is required because Microsoft is not in compliance with the RFC spec for oauth2.0
///
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct NaverTokenResponse<EF, TT>
where
	EF: ExtraTokenFields,
	TT: TokenType,
{
	access_token: AccessToken,
	#[serde(bound = "TT: TokenType")]
	#[serde(deserialize_with = "helpers::deserialize_untagged_enum_case_insensitive")]
	token_type: TT,
	#[serde(skip_serializing_if = "Option::is_none")]
	expires_in: Option<String>,
	#[serde(skip_serializing_if = "Option::is_none")]
	refresh_token: Option<RefreshToken>,
	#[serde(rename = "scope")]
	#[serde(deserialize_with = "helpers::deserialize_space_delimited_vec")]
	#[serde(serialize_with = "helpers::serialize_space_delimited_vec")]
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(default)]
	scopes: Option<Vec<Scope>>,

	#[serde(bound = "EF: ExtraTokenFields")]
	#[serde(flatten)]
	extra_fields: EF,
}

impl<EF, TT> NaverTokenResponse<EF, TT>
where
	EF: ExtraTokenFields,
	TT: TokenType,
{
	pub fn new(access_token: AccessToken, token_type: TT, extra_fields: EF) -> Self {
		Self {
			access_token,
			token_type,
			expires_in: None,
			refresh_token: None,
			scopes: None,
			extra_fields,
		}
	}

	pub fn set_access_token(&mut self, access_token: AccessToken) {
		self.access_token = access_token;
	}

	pub fn set_token_type(&mut self, token_type: TT) {
		self.token_type = token_type;
	}

	pub fn set_expires_in(&mut self, expires_in: Option<&Duration>) {
		self.expires_in = expires_in.map(|exp| Duration::as_secs(exp).to_string());
	}

	pub fn set_refresh_token(&mut self, refresh_token: Option<RefreshToken>) {
		self.refresh_token = refresh_token;
	}

	pub fn set_scopes(&mut self, scopes: Option<Vec<Scope>>) {
		self.scopes = scopes;
	}

	pub fn extra_fields(&self) -> &EF {
		&self.extra_fields
	}

	pub fn set_extra_fields(&mut self, extra_fields: EF) {
		self.extra_fields = extra_fields;
	}
}

impl<EF, TT> TokenResponse<TT> for NaverTokenResponse<EF, TT>
where
	EF: ExtraTokenFields,
	TT: TokenType,
{
	///
	/// The access token issued by the Naver authentication server
	///
	fn access_token(&self) -> &AccessToken {
		&self.access_token
	}
	fn token_type(&self) -> &TT {
		&self.token_type
	}
	fn expires_in(&self) -> std::option::Option<std::time::Duration> {
		self.expires_in.as_ref().map(|exp| {
			let expires_in_number: u64 = exp.parse::<u64>().unwrap();

			Duration::from_secs(expires_in_number)
		})
	}
	fn refresh_token(&self) -> Option<&RefreshToken> {
		self.refresh_token.as_ref()
	}
	fn scopes(&self) -> Option<&Vec<Scope>> {
		self.scopes.as_ref()
	}
}

impl<EF, TT> From<StandardTokenResponse<EF, TT>> for NaverTokenResponse<EF, TT>
where
	EF: ExtraTokenFields + Clone,
	TT: TokenType,
{
	fn from(st: StandardTokenResponse<EF, TT>) -> Self {
		let expire_time_string = st
			.expires_in()
			.map(|exp| Duration::as_secs(&exp).to_string());

		let extra_fields: EF = st.extra_fields().clone();

		NaverTokenResponse {
			access_token: st.access_token().clone(),
			token_type: st.token_type().clone(),
			expires_in: expire_time_string,
			refresh_token: st.refresh_token().map(|r| r.clone()),
			scopes: st.scopes().map(|s| s.clone()),
			extra_fields: extra_fields,
		}
	}
}

///
/// alias for NaverTokenResponse type
///
pub type BasicNaverTokenResponse = NaverTokenResponse<EmptyExtraTokenFields, BasicTokenType>;

///
/// Alias for Client that makes use of the NaverTokenResponse custom type
///
pub type NaverClient = Client<
	BasicErrorResponse,
	BasicNaverTokenResponse,
	BasicTokenType,
	BasicTokenIntrospectionResponse,
	StandardRevocableToken,
	BasicRevocationErrorResponse,
>;