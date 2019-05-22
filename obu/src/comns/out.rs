extern crate reqwest;

pub fn send( data: Vec<u8> ){
    let client = reqwest::Client::new();
    let res = client
        .post("http://localhost:8008/batches")
        .header("Content-Type", "application/octet-stream")
        .body(data)
        .send();
}
