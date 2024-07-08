use sqlx::types::chrono::{DateTime, Utc};

#[derive(Debug)]
pub struct Code {
    pub code_id: String,
    pub code_value: String,
    pub code_value_nm: Option<String>,
    pub code_desc: Option<String>,
    pub created_at: DateTime<Utc>,
    pub created_by: i32,
    pub updated_at: DateTime<Utc>,
    pub updated_by: i32,
    pub is_deleted: bool,
}

pub struct LoginTyCd {
    pub email: &'static str
}

pub struct UserSttCd {
    pub ok: &'static str,
    pub quit: &'static str,
}

pub struct UserTyCd {
    pub user: &'static str,
    pub admin: &'static str,
}

pub struct  DbCode {
    pub login_ty_cd: LoginTyCd,
    pub user_stt_cd: UserSttCd,
    pub user_ty_cd: UserTyCd,
}

pub const DB_CODE: DbCode = DbCode {
    login_ty_cd:  LoginTyCd {
        email: "EMAIL",
    },
    user_stt_cd: UserSttCd {
        ok: "OK",
        quit: "QUIT",
    },
    user_ty_cd: UserTyCd {
        admin: "ADMIN",
        user: "EMAIL",
    },
};