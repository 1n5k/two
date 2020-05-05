#![feature(proc_macro_hygiene, decl_macro)]
#[macro_use] extern crate rocket;
#[macro_use] extern crate rocket_contrib;

use rocket_contrib::serve::StaticFiles;

fn rocket() -> rocket::Rocket {
    rocket::ignite().mount("/", StaticFiles::from("static"))
}

fn main() {
    rocket().launch();
}

#[cfg(test)]
mod test {
    use super::rocket;
    use rocket::local::Client;
    use rocket::http::Status;
    use rocket::http::ContentType;
    
    #[test]
    fn test_index() {
        let client = Client::new(rocket()).unwrap();
        let response = client.get("/").dispatch();
        assert_eq!(response.status(), Status::Ok);        
    }

    #[test]
    fn test_files() {
        let client = Client::new(rocket()).unwrap();
        let mut response = client.get("/favicon.png").dispatch();
        assert_eq!(response.headers().iter().next(), Some(ContentType::PNG.into()));
        let mut response = client.get("/keybase.txt").dispatch();
        assert_eq!(response.headers().iter().next(), Some(ContentType::Plain.into()));
    }
}
