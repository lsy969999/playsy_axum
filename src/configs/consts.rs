use crate::repositories::entities::code::{DbCode, LoginTyCd, UserSttCd, UserTyCd};

pub const ACCESS_TOKEN: &'static str = "access_token";
pub const REFRESH_TOKEN: &'static str = "refresh_token";

// pub const DB_CODE: DbCode = DbCode {
//     login_ty_cd:  LoginTyCd {
//         email: "EMAIL",
//     },
//     user_stt_cd: UserSttCd {
//         ok: "OK",
//         quit: "QUIT",
//     },
//     user_ty_cd: UserTyCd {
//         admin: "ADMIN",
//         user: "USER",
//     },
// };