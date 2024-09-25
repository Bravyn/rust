#[allow(unused)]

use url::{Url, ParseError, Host, Position};

fn main() {
    let sample_url = Url::parse("http://127.0.0.1:8080").unwrap();
    println!("{}",check_url_port(sample_url));
}


fn check_url_port(url: Url) -> bool {
    //checks whether given url has a port
    match url.port() {
        Some(_) => return true,
        None => return false
    }
}