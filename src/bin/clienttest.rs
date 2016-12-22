extern crate counterdb;
extern crate grpc;

use counterdb::client_protocol::ReadRequest;
use counterdb::client_protocol::ReadResponse;

use counterdb::client_protocol_grpc::PartServerClient;
use counterdb::client_protocol_grpc::PartServer;

fn main() {
    let client: PartServerClient = PartServerClient::new("localhost", 50001, false).unwrap();
    let mut read_req = ReadRequest::new();

    let test_key = String::from("test-key").into_bytes();
    read_req.set_key(test_key);

    let response: ReadResponse = client.read(read_req).unwrap();
    println!("Value found: {:?}", response.get_is_value());
}
