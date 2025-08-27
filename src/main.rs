use rocket::fs::{FileServer, Options};
use rocket::launch;

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", FileServer::new(rocket::fs::relative!("static"), Options::Index))
}
