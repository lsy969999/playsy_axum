// #[derive(Debug)]
// pub enum UserJoinError {
//     NickNameExists, // 닉네임이 이미 존재
//     InsertFail, // 생성 에러
//     PassGenFail, // 비밀번호 생성 에러
//     Db(sqlx::error::Error) // 기타 디비 에러
// }

// impl std::fmt::Display for UserJoinError {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         match self {
//             Self::NickNameExists => write!(f, "user nickname exists"),
//             Self::InsertFail => write!(f, "user insert fail"),
//             Self::PassGenFail => write!(f, "password generate fail"),
//             Self::Db(_) => write!(f, "db problem"),
//         }
//     }
// }

// impl std::error::Error for UserJoinError {}

// impl From<sqlx::error::Error> for UserJoinError {
//     fn from(value: sqlx::error::Error) -> Self {
//         Self::Db(value)
//     }
// }