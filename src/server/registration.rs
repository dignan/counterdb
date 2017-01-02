use std::sync::Arc;

use zookeeper::ZooKeeper;
use zookeeper::CreateMode;
use zookeeper::acls;

use error::CounterDbResult;
use error::CounterDbError;
use error::CounterDbError::AlreadyInitialized;
use error::CounterDbError::NotInitialized;

pub struct Registrar {
    zk: Option<Arc<ZooKeeper>>,
    registration_root: String,
    name: String,
}

impl Registrar {
    pub fn new(registration_root: String, name: String) -> Registrar {
        Registrar {
            zk: None,
            registration_root: registration_root,
            name: name,
        }
    }

    pub fn start(&mut self, zk: Arc<ZooKeeper>) -> CounterDbResult<()> {
        if self.zk.is_some() {
            return Err(AlreadyInitialized);
        }

        self.zk = Some(zk);
        Ok(())
    }
    pub fn register(&self) -> CounterDbResult<()> {
        match self.zk {
            Some(ref zk) => {
                match (&zk).create(&self.registration_path(),
                                   Vec::new(),
                                   acls::OPEN_ACL_UNSAFE.clone(),
                                   CreateMode::Ephemeral) {
                    Ok(str) => Ok(()),
                    Err(e) => Err(CounterDbError::Zk(e)),
                }
            }
            None => Err(NotInitialized),
        }
    }

    pub fn unregister(&self) -> CounterDbResult<()> {
        match self.zk {
            Some(ref zk) => {
                let (_, stat) = (&zk).get_data(&self.registration_path(), false)?;
                let res = match (&zk).delete(&self.registration_path(), stat.version) {
                    Ok(()) => Ok(()),
                    Err(e) => Err(CounterDbError::Zk(e)),
                };

                info!("Unregistered");
                res
            }
            None => Err(NotInitialized),
        }
    }

    pub fn stop(&mut self) {
        info!("Stopping the registrar");
        self.unregister();
        info!("Registrar stopped");
    }

    fn registration_path(&self) -> String {
        format!("{}/{}", self.registration_root, self.name)
    }
}
