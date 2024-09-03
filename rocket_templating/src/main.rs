#[macro_use]
extern crate rocket;

extern crate tera as Tera;



mod tera;

use rocket::response::content::RawHtml;
use rocket_dyn_templates::Template;

#[get("/")]
fn index() -> RawHtml<&'static str> {
    RawHtml(
        r#" Helloo, I am Ian. A rust web app dev."#,
    )
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/" , routes![index])
        .mount("/tera", routes![tera::index, tera::hello, tera::about])
        .register("/tera", catchers![tera::not_found])
        .attach(Template::fairing())


    }
