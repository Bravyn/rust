#[macro_use] extern crate rocket;
use rocket::serde::{ json::Json, Serialize};
use rocket::form::Form;
use rocket::http::Status;

/// This function handles HTTP GET requests to the root URL ("/").
///
/// # Returns
///
/// A static string slice containing "Hello World". This string is returned
/// as the response to the HTTP GET request.
///

#[get("/")]
fn index() -> &'static str {
    "Hello World"
}

/// A struct representing a message with a single string field.
///
/// This struct is used to serialize a message into a format that can be
/// easily sent over the network or saved to a file. It implements the
/// `Serialize` trait, allowing it to be converted into a format like JSON.
///
/// # Fields
///
/// - `message`: A `String` containing the text of the message.
///
#[derive(Serialize)]
struct Message {
    message: String,
}

/// Handles HTTP GET requests to the "/json" URL and responds with a JSON representation of a `Message`.
///
/// This function constructs a `Message` instance with a predefined string and returns it
/// wrapped in a `Json` response. The `Json` type automatically serializes the `Message`
/// struct to JSON format.
///
/// # Returns
///
/// A `Json<Message>` response containing the serialized `Message` with the string "Hello from Rocket!".
///
/// # Example
///
/// ```rust
/// // Assume Rocket is running and this endpoint is hit with a GET request.
/// // The response will be a JSON object:
/// //
/// // {
/// //     "message": "Hello from Rocket!"
/// // }
/// ```

#[get("/json")]
fn json_response() -> Json<Message> {
    Json(Message {
        message: "Hello from Rocket!".to_string(),
    })

}

/// Handles HTTP GET requests to the "/hello" URL with an optional query parameter `name`.
///
/// This function generates a personalized greeting message based on the presence of
/// the `name` query parameter. If `name` is provided, it responds with a message
/// that includes the given name. If `name` is not provided, it responds with a
/// default message for a stranger.
///
/// # Parameters
///
/// - `name`: An optional `String` query parameter extracted from the URL. If provided,
///   it will be used in the greeting message.
///
/// # Returns
///
/// A `String` containing the greeting message. The message is either personalized with
/// the provided `name` or defaults to a general greeting for a stranger.
///
/// # Example
///
/// ```rust
/// // If the URL is "/hello?name=Alice":
/// let response = personalized_greeting(Some("Alice".to_string()));
/// assert_eq!(response, "Hello, Alice!");
///
/// // If the URL is "/hello" without a `name` parameter:
/// let response = personalized_greeting(None);
/// assert_eq!(response, "Hello, stranger");
/// ```
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
