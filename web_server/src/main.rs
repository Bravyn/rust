#[macro_use] extern crate rocket;
use rocket::serde::{ json::Json, Serialize};
use rocket::form::Form;
//use rocket::http::Status;
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
        message: "Hello from Rocket!".to_string(),
    })

}


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
    let allowed_origins = AllowedOrigins::some_exact(
        &["http://localhost:3000"]
    );
    //configure CORS
    let cors = CorsOptions{
        allowed_origins,
        allow_credentials: true,
        ..Default::default()

    }
    .to_cors()
    .expect("Error creating CORS");

    rocket::build().attach(cors).mount("/", routes![index, json_response, personalized_greeting, submit_form])
    
}
