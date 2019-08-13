use crate::data::votes::{VoteResponse};
use crate::data::transactions::{Transaction, TransactionResponse};
use crate::data::balance::{BalanceResponse};

use crate::{
    retrieve_dignitas,
    get_list_votes,
    proxy_transaction,
};

use rocket_contrib::json::{Json};

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

#[post("/transaction", data= "<body>")]
fn post_transaction(body: Json<Transaction>)
    -> Json<TransactionResponse>
{
    println!("Received a Transaction");

    proxy_transaction(body.payload.clone());

    let response = TransactionResponse{status: "OK".to_string()};

    Json(response)
}

pub fn start_server(){
    rocket::ignite()
        .mount("/api/v1",
               routes![get_vote, post_transaction, get_balance])
        .launch();
}
