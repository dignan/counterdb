use std::sync::Arc;

use zookeeper::ZooKeeper;
use zookeeper::CreateMode;
use zookeeper::acls;

use error::CounterDbResult;
use error::CounterDbError;

pub struct Registrar {
    zk: Arc<ZooKeeper>,
    registration_root: String,
    name: String,
}

impl Registrar {
    pub fn new(zk: Arc<ZooKeeper>, registration_root: String, name: String) -> Registrar {
        Registrar {
            zk: zk,
            registration_root: registration_root,
            name: name,
        }
    }

    pub fn register(&self) -> CounterDbResult<()> {
        match self.zk.create(&self.registration_path(), Vec::new(), acls::OPEN_ACL_UNSAFE.clone(), CreateMode::Ephemeral) {
            Ok(str) => Ok(()),
            Err(e) => Err(CounterDbError::Zk(e))
        }
    }

    pub fn unregister(&self) -> CounterDbResult<()> {
        let (_, stat) = self.zk.get_data(&self.registration_path(), false)?;
        match self.zk.delete(&self.registration_path(), stat.version) {
            Ok(()) => Ok(()),
            Err(e) => Err(CounterDbError::Zk(e))
        }
    }

    fn registration_path(&self) -> String {
        format!("{}/{}", self.registration_root, self.name)
    }
}