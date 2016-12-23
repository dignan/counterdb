extern crate counterdb;
extern crate grpc;
extern crate rocksdb;
#[macro_use] extern crate log;
extern crate log4rs;

use std::thread;

use rocksdb::DB;

use grpc::result::GrpcResult;

use log4rs::file::Deserializers;

use counterdb::protocol::client_grpc::PartServer;
use counterdb::protocol::client_grpc::PartServerServer;

use counterdb::protocol::client::ReadRequest;
use counterdb::protocol::client::ReadResponse;

use counterdb::protocol::client::SetRequest;
use counterdb::protocol::client::SetResponse;

use counterdb::server::partition::read;
use counterdb::server::partition::set;
use counterdb::server::partition::get_db_options;

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
            Err(e) => panic!("we should probably handle this better ;) {}", e)
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
                error!("Error on set request: {:?} error: {}", req, e);
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
                Err(e) => panic!("Freak out we don't know how to database!!!! error {}", e)
            }
        }
    }
}

fn main() {
    log4rs::init_file("partserver.log", Deserializers::default()).unwrap();
    info!("Starting partserver");

    let server_impl = PartServerImpl::new();

    PartServerServer::new(50001, server_impl);

    info!("Part server started");

    loop {
        thread::park();
    }
}
