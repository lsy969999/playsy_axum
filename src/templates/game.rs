use askama::Template;

use crate::models::user_info::UserInfo;

#[derive(Template)]
#[template(path="pages/games/bevy_wasm_test.html")]
pub struct BevyWasmTestTemplate{
    pub user_info: Option<UserInfo>
}