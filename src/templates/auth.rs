use askama::Template;

#[derive(Template)]
#[template(path="pages/auth.html")]
pub struct AuthTemplate {
    pub authenticity_token: String,
}

#[derive(Template)]
#[template(path="fragments/auth_error.html")]
pub struct AuthErrorFragment {
    pub msg: String
}

#[derive(Template)]
#[template(path="pages/signup.html")]
pub struct SignupPage;