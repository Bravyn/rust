use rocket::Request;
use rocket::response::Redirect;

use tera::Tera;

use rocket_dyn_templates::{Template, context};

#[get("/")]
pub fn index() -> Redirect {
    Redirect::to(uri!("/tera", hello(name = "Droid")))
}

#[get("/hello/<name>")]
pub fn hello(name: &str) -> Template {
    Template::render("tera/index", context!{
        title: "Hello",
        name: Some(name),
        items: vec!["Moja", "Mbili", "Tatu"]
    })
}

#[get("/about")]
pub fn about() -> Template {
    Template::render("tera/about.html", context! {
        title: "About"
    })
}

#[catch(404)]
pub fn not_found(req: &Request<'_>) -> Template {
    Template::render("tera/error/404", context! {
        uri: req.uri()
    })
}

pub fn customize(tera: &mut Tera) {
    tera.add_raw_template("tera/about.html", r#"
    {% extends "tera/base" %}

    {% block content % }
    <section id = "about">
    <h1> About - Here is another raw page </h1>
    </section>

    {% endblock content %}
    
    "#).expect("Valid Tera template")
}