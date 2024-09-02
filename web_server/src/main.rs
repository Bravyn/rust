#[macro_use] extern crate rocket;
use rocket::serde::{ json::Json, Serialize};
use rocket::form::Form;
use rocket::http::Status;

#[get("/")]
fn index() -> &'static str {
    "Hello World"
}

//defining a custom structure for JSON response
#[derive(Serialize)]
struct Message {
    message: String,
}

//this route returns a json response
#[get("/json")]
fn json_response() -> Json<Message> {
    Json(Message {
        message: "Hello from Rocket!".to_string(),
    })

}

//this route has a query parameter
#[get("/hello?<name>")]
fn personalized_greeting(name: Option<String>) -> String {
    match name {
        Some(name) => format!("Hello, {}!", name),
        None => "Hello, stranger".to_string(),
    }
}

//define a struct to handle incoming JSON payload
#[derive(FromForm, Serialize, Debug)]
struct UserInput {
    name: String,
    age: u8,
}

//this route accepts POST requests with JSON data
#[post("/submit", data = "<input>")]
fn submit_form(input: Form<UserInput>) -> Json<Message> {
    Json(Message {
        message: format!("Received data - Name: {}, Age: {}",
    input.name, input.age),
    })
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index, json_response, personalized_greeting, submit_form])
}
