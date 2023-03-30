#[macro_use]
extern crate rocket;

mod service;
mod state;
mod models;
mod error;
mod routes;

use routes::{index, authorize, refund, queue};

#[launch]
async fn rocket() -> _ {
    // setup app state
    let service = service::Service::new();
    let app_state = state::AppState::new(service);

    rocket::build()
        .mount("/", routes![index, authorize, refund, queue])
        .manage(app_state)
}
