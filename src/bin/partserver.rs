extern crate counterdb;
extern crate grpc;
extern crate rocksdb;

use std::thread;

use rocksdb::DB;
use rocksdb::Options;

use grpc::result::GrpcResult;

use counterdb::protocol::client_grpc::PartServer;
use counterdb::protocol::client_grpc::PartServerServer;

use counterdb::protocol::client::ReadRequest;
use counterdb::protocol::client::ReadResponse;

use counterdb::protocol::client::SetRequest;
use counterdb::protocol::client::SetResponse;

use counterdb::read;
use counterdb::set;
use counterdb::get_db_options;

struct PartServerImpl {
    db: DB
}

impl PartServer for PartServerImpl {
    fn read (&self, req: ReadRequest) -> GrpcResult<ReadResponse> {
        match read(&(self.db), req.get_key()) {
            Ok(maybe_val) => {
                match maybe_val {
                    Some(val) => {
                        let mut response = ReadResponse::new();
                        response.set_value(val);
                        response.set_is_value(true);
                        Ok(response)
                    },
                    None => Ok(ReadResponse::new())
                }
            },
            Err(e) => panic!("we should probably handle this better ;)")
        }
    }

    fn set(&self, req: SetRequest) -> GrpcResult<SetResponse> {
        let mut response = SetResponse::new();

        match set(&(self.db), req.get_key(), req.get_value()) {
            Ok(()) => {
                Ok(response)
            },
            Err(e) => {
                response.set_is_error(true);
                response.set_error_message(String::from("surely i'll come back to this"));
                Ok(response)
            }
        }
    }
}

impl PartServerImpl {
    pub fn new() -> PartServerImpl {
        PartServerImpl {
            db: match DB::open(&get_db_options(), "test-rdb") {
                Ok(db) => db,
                Err(e) => panic!("Freak out we don't know how to database!!!!")
            }
        }
    }
}

fn main() {
    let server_impl = PartServerImpl::new();
    let server = PartServerServer::new(50001, server_impl);

    loop {
        thread::park();
    }
}
