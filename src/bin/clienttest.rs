extern crate counterdb;
extern crate grpc;
#[macro_use] extern crate log;
extern crate log4rs;

use log4rs::file::Deserializers;

use counterdb::protocol::client::ReadRequest;
use counterdb::protocol::client::ReadResponse;
use counterdb::protocol::client::SetRequest;
use counterdb::protocol::client::SetResponse;

use counterdb::protocol::client_grpc::PartServerClient;
use counterdb::protocol::client_grpc::PartServer;

fn main() {
    log4rs::init_file("clienttest.log", Deserializers::default());

    let client: PartServerClient = PartServerClient::new("localhost", 50001, false).unwrap();
    let mut read_req = ReadRequest::new();

    let test_key = String::from("test-key").into_bytes();
    let test_key2 = test_key.clone();
    let test_key3 = test_key.clone();
    read_req.set_key(test_key);

    let response: ReadResponse = client.read(read_req).unwrap();
    info!("Value found: {:?}", response.get_is_value());

    let mut set_request = SetRequest::new();
    set_request.set_key(test_key2   );
    set_request.set_value(45);
    let set_response = client.set(set_request).unwrap();

    info!("Set request successful: {:?}", response.get_is_error());

    let mut read_request = ReadRequest::new();
    read_request.set_key(test_key3);
    let read_response = client.read(read_request).unwrap();

    info!("Current value: {:?}", read_response.get_value());
}
