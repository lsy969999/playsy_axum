pub trait SocaliLoginValidateProcess {
    fn get_id(&self) -> String;
    fn get_email(&self) -> Option<String>;
    fn get_nick_name(&self) -> Option<String>;
    fn get_avatar_url(&self) -> Option<String>;
}