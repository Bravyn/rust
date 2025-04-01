#[allow(unused)]

use url::{Url, ParseError, Host, Position};

fn main() {
    let sample_url = Url::parse("http://127.0.0.1").unwrap();
    let link1 = UrlChecker::new(&sample_url);

    println!("{:?}", link1.get_port());
    
    
}


struct UrlChecker<'a>{
    url: &'a Url
}


impl UrlChecker<'_> {
    
    fn new(url: &Url) -> UrlChecker {
        UrlChecker{
            url: url
        }
    }

    fn get_port(self) -> Option<u16> {
        //checks whether given url has a port
       if let Some(port) = self.url.port() {
        return Some(port)
       }
    }

   /* fn host(self) {
        match self.url.host(){
            Some(_) => {}
        }

    }*/
}


