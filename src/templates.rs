use askama::Template;

#[derive(Template)]
#[template(path = "pages/home.html")]
pub struct Home<'a> {
  pub api: &'a str,
}

#[derive(Template)]
#[template(path = "pages/login/get.html")]
pub struct GetLogin;

#[derive(Template)]
#[template(path = "pages/login/post.html")]
pub struct PostLogin {
  pub message: String,
}
