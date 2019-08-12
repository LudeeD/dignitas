use crate::data::votes::{VoteResponse, Location};
use crate::data::transactions::{Transaction, TransactionResponse};
use crate::data::balance::{BalanceResponse};

use crate::{
    retrieve_dignitas,
    get_list_votes,
    create_vote,
    vote
};

use rocket::response::status;
use rocket::http::RawStr;

use rocket_contrib::json::{Json, JsonValue};

use geohash_16::{encode, Coordinate};


#[get("/balance/<wallet>")]
fn get_balance(wallet : String) 
    -> Json<BalanceResponse> 
{

    let test = retrieve_dignitas(&wallet);

    let ret = Json(test.get_balance_response());

    println!("{:?}",ret);

    ret
}

#[get("/vote")]
fn get_vote() -> Json<VoteResponse>{
    println!("Getting Votes");

    let list = get_list_votes();

    let response = VoteResponse::new(list, "OK");

    Json(response)
}

#[post("/vote", data= "<body>")]
fn post_vote(body: Json<Transaction>)
    -> Json<TransactionResponse>
{
    println!("Create a new Vote");

    create_vote("client.key", body.payload.clone());
    println!("{:?}", body);

    let response = TransactionResponse{status: "OK".to_string()};
    Json(response)
}

#[get("/vote/<id>")]
fn get_vote_detail(id: &RawStr) -> status::Accepted<()> {
    status::Accepted::<()>(None)
}

#[post("/opinion", data= "<body>")]
fn post_vote_update(body: Json<Transaction>)
    ->  Json<TransactionResponse> {

    println!("Vote in existing vote");

    vote("client.key", body.payload.clone());

    let response = TransactionResponse{status: "OK".to_string()};
    Json(response)
}

#[get("/geoid", data = "<body>")]
fn get_geo_id(body: Json<Location>) -> String{

    let c = Coordinate {x: body.lng, y: body.lat};
    let encoded : String = encode(c, 12usize)
        .expect("Generating ID");
    encoded
}

pub fn start_server(){
    rocket::ignite()
        .mount("/api/v1",
               routes![ get_vote,
               post_vote,
               get_vote_detail,
               post_vote_update,
               get_balance,
               get_geo_id])
        .launch();
}
