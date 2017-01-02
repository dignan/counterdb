use std::sync::Arc;

use std::time::Duration;

use rocksdb::DB;

use grpc::result::GrpcResult;

use zookeeper::ZooKeeper;
use zookeeper::Watcher;
use zookeeper::WatchedEvent;

use protocol::client_grpc::PartServer;

use protocol::client::ReadRequest;
use protocol::client::ReadResponse;

use protocol::client::SetRequest;
use protocol::client::SetResponse;

use super::partition::read;
use super::partition::set;
use super::partition::get_db_options;

use super::registration::Registrar;

use error::CounterDbResult;

use configuration::server_config::PartServerConfig;

pub struct PartServerImpl {
    db: DB,
    registrar: Registrar,
    config: PartServerConfig<String>,
}

impl PartServer for PartServerImpl {
    fn read(&self, req: ReadRequest) -> GrpcResult<ReadResponse> {
        match read(&(self.db), req.get_key()) {
            Ok(maybe_val) => {
                match maybe_val {
                    Some(val) => {
                        let mut response = ReadResponse::new();
                        response.set_value(val);
                        response.set_is_value(true);
                        Ok(response)
                    }
                    None => Ok(ReadResponse::new()),
                }
            }
            Err(e) => panic!("we should probably handle this better ;) {}", e),
        }
    }

    fn set(&self, req: SetRequest) -> GrpcResult<SetResponse> {
        let mut response = SetResponse::new();

        match set(&(self.db), req.get_key(), req.get_value()) {
            Ok(()) => {
                Ok(response)
            }
            Err(e) => {
                response.set_is_error(true);
                response.set_error_message(String::from("surely i'll come back to this"));
                error!("Error on set request: {:?} error: {}", req, e);
                Ok(response)
            }
        }
    }
}

struct PartServerZkWatcher;
impl Watcher for PartServerZkWatcher {
    fn handle(&self, event: WatchedEvent) {
        info!("Got event: {:?}", event)
    }
}

impl PartServerImpl {
    pub fn new(partserver_config: PartServerConfig<String>) -> CounterDbResult<PartServerImpl> {
        Ok(PartServerImpl {
            db: match DB::open(&get_db_options(), "test-rdb") {
                Ok(db) => db,
                Err(e) => panic!("Freak out we don't know how to database!!!! error {}", e),
            },
            registrar: Registrar::new(String::from("/counterdb/partservers/"),
                                      partserver_config.hostname.clone()),
            config: partserver_config,
        })
    }

    pub fn start(&mut self) -> CounterDbResult<()> {
        let zk: Arc<ZooKeeper> = Arc::new(ZooKeeper::connect(&self.config.zk_connect_string,
                                                             Duration::from_millis(10_000),
                                                             PartServerZkWatcher)?);
        self.registrar.start(zk.clone())
    }

    pub fn register(&self) -> CounterDbResult<()> {
        self.registrar.register()
    }
}
