#[macro_use] extern crate rocket
use rocket::serde::{ json::Jsonm Serialize };
use rocket::form::Form;
use rocket_cors::{ AllowedOrigins, CorsOptions};

#[get("/")]
fn index() -> &'static str {
    "Guten Morgen"
}

#[derive(Serialize)]
struct Message {
    message: String,
}

#[get("/json")]
fn json_response() -> Json<Message> {
    Json(Message {
        message: "Hello from Rocket".to_string(),
    })
}

#[get("/hello?<name>")]
fn personalized_greeting(name: Option<String>) -> String  {
    match name {
        Some(name) => format!("Hello, {}!"),
        None => "Hello, stranger".to_string(),
    }
} 

#[derive(FromForm, Serialize, Debug)]
struct UserInput{
    name: String,
    age: u8,
}

#[post("/submit", data = "<input>")]
fn submit_form(input: Form<UserInput>) -> Json<Message>{
    Json(Message {
        message: format!("Received data - Name: {}, Age: {}",)
    })
}
