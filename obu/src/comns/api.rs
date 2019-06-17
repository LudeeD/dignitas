use crate::data::votes::{VoteResponse};

use crate::{
    retrieve_dignitas,
    get_list_votes,
    create_vote,
    vote
};

use rocket::response::status;
use rocket::http::RawStr;

use rocket_contrib::json::{Json, JsonValue};

#[get("/balance")]
fn get_balance() -> status::Accepted<()> {

    retrieve_dignitas("client.key");

    status::Accepted::<()>(None)
}

#[get("/vote")]
fn get_vote() -> Json<VoteResponse>{
    println!("Getting Votes");

    let list = get_list_votes();
    let response = VoteResponse::new(list, "OK");

    Json(response)
}

#[post("/vote")]
fn post_vote() -> status::Accepted<()> {
    println!("Create a new Vote");

    create_vote("client.key", 1);

    status::Accepted::<()>(None)
}

#[get("/vote/<id>")]
fn get_vote_detail(id: &RawStr) -> status::Accepted<()> {
    status::Accepted::<()>(None)
}

#[post("/vote/<id>")]
fn post_vote_update(id: &RawStr) -> status::Accepted<()> {
    println!("Vote in existing vote");

    vote("client.key", 1, 2);

    status::Accepted::<()>(None)
}

pub fn start_server(){
    rocket::ignite()
        .mount("/api/v1",
               routes![ get_vote,
               post_vote,
               get_vote_detail,
               post_vote_update,
               get_balance,])
        .launch();
}
