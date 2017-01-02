use std::sync::Arc;

use std::time::Duration;
use std::time::Instant;

use uuid::Uuid;

use grpc::result::GrpcResult;

use zookeeper::ZooKeeper;
use zookeeper::Watcher;
use zookeeper::WatchedEvent;

use protocol::coordinator_grpc::Coordinator;

use protocol::coordinator::CreateTableRequest;
use protocol::coordinator::CreateTableResponse;
use protocol::coordinator::DeleteTableRequest;
use protocol::coordinator::DeleteTableResponse;

use configuration::server_config::CoordinatorConfig;

use error::CounterDbResult;

use server::registration::Registrar;

struct CoordinatorImpl {
    registrar: Registrar,
    run_state: CoordinatorRunState,
    state: CoordinatorState,
    config: CoordinatorConfig<String>,
    start_time: Instant,
    run_id: Uuid,
}

enum CoordinatorRunState {
    STARTING,
    RUNNING,
    STOPPING,
    STOPPED,
}


pub enum CoordinatorState {
    ACTIVE,
    INACTIVE,
}

impl Coordinator for CoordinatorImpl {
    fn create_table(&self, req: CreateTableRequest) -> GrpcResult<CreateTableResponse> {
        unimplemented!()
    }

    fn delete_table(&self, req: DeleteTableRequest) -> GrpcResult<DeleteTableResponse> {
        unimplemented!()
    }
}

impl CoordinatorImpl {
    pub fn new(coordinator_config: CoordinatorConfig<String>) -> CounterDbResult<CoordinatorImpl> {
        let run_id = Uuid::new_v4();
        Ok(CoordinatorImpl {
            registrar: Registrar::new(String::from("/counterdb/coordinators/"),
                                      get_unique_hoststring(coordinator_config.hostname.clone(),
                                                            run_id.clone())),
            run_state: CoordinatorRunState::STOPPED,
            state: CoordinatorState::INACTIVE,
            config: coordinator_config,
            start_time: Instant::now(),
            run_id: run_id,
        })
    }

    pub fn start(&mut self) -> CounterDbResult<()> {
        info!("Starting coordinator");
        self.run_state = CoordinatorRunState::STARTING;
        let zk: Arc<ZooKeeper> =
            Arc::new(ZooKeeper::connect(self.config.zk_connect_string.as_str(),
                                        Duration::from_millis(10_000),
                                        CoordinatorZkWatcher)?);

        info!("Starting registrar");
        self.registrar.start(zk.clone())?;
        let res = self.registrar.register();

        info!("Coordinator registered");
        self.run_state = CoordinatorRunState::RUNNING;
        res
    }

    pub fn register(&self) -> CounterDbResult<()> {
        self.registrar.register()
    }

    pub fn unregister(&self) -> CounterDbResult<()> {
        self.registrar.unregister()
    }
}

fn get_unique_hoststring(hostname: String, run_id: Uuid) -> String {
    format!("{}/{}", hostname.clone(), run_id.clone())
}

struct CoordinatorZkWatcher;

impl Watcher for CoordinatorZkWatcher {
    fn handle(&self, event: WatchedEvent) {
        info!("Got zk event {:?}", event)
    }
}
