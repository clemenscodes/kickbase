use askama::Template;

#[derive(Template)]
#[template(path = "pages/home.html")]
pub struct Home<'a> {
  pub api: &'a str,
}
