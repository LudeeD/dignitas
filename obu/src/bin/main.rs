#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;

use rocket::response::status;
use rocket::http::RawStr;

#[get("/vote")]
fn get_vote() -> status::Accepted<()> {
    status::Accepted::<()>(None)
}

#[post("/vote")]
fn post_vote() -> status::Accepted<()> {
    println!("Create a new Vote");

    obu::create_vote("client.key", 1);

    status::Accepted::<()>(None)
}

#[get("/vote/<id>")]
fn get_vote_detail(id: &RawStr) -> status::Accepted<()> {
    status::Accepted::<()>(None)
}

#[post("/vote/<id>")]
fn post_vote_update(id: &RawStr) -> status::Accepted<()> {
    println!("Vote in existing vote");

    obu::vote("client.key", 1, 2);

    status::Accepted::<()>(None)
}

fn main() {
    rocket::ignite()
        .mount("/api/v1", routes![get_vote, post_vote, get_vote_detail, post_vote_update])
        .launch();
}

