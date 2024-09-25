#[allow(unused)]

use url::{Url, ParseError, Host, Position};

fn main() {
    let sample_url = Url::parse("https://www.youtube.com/watch?v=CfihYWRWRTQ&list=RDk3bQnX_sIXk&index=4").unwrap();
  //assert!(Url::parse("http://[:::1]") != Err(ParseError::InvalidIpv6Address))
  //println!("{:?}",sample_url.port().unwrap());
  check_url_port(sample_url)
}


fn check_url_port(url: Url){
    

    match url.port() {
        Some(value) => println!("Got it"),
        None => println!("!Got it")
    }

}