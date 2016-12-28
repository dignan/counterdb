use std::time::Duration;
use std::time::Instant;

use zookeeper::ZooKeeper;
use zookeeper::Watcher;
use zookeeper::WatchedEvent;
use zookeeper::CreateMode;
use zookeeper::ZkError;

use zookeeper::acls;

use regex::Regex;
use regex::Captures;

use uuid::Uuid;

use server::table::Table;

pub struct Coordinator {
    hostname: String,
    start_time: Instant,
    run_id: Uuid,
    status: CoordinatorStatus,
    zk: ZooKeeper,
    lock_path: Option<String>,
}

pub enum CoordinatorStatus {
    ACTIVE,
    INACTIVE,
}

static COORDINATOR_REGISTRATION_PATH: &'static str = "/counterdb/coordinators/{}";
static COORDINATOR_LOCK_PATH: &'static str = "/counterdb/coordinator_lock/_locknode_/lock-";

impl Coordinator {
    pub fn new(hostname: String, zk_connect_string: String) -> Result<Coordinator, ZkError> {
        let zk: ZooKeeper = ZooKeeper::connect(&zk_connect_string,
                                               Duration::from_millis(15_000),
                                               CoordinatorWatcher)?;

        Ok(Coordinator {
            hostname: hostname,
            start_time: Instant::now(),
            run_id: Uuid::new_v4(),
            status: CoordinatorStatus::INACTIVE,
            zk: zk,
            lock_path: None,
        })
    }

    pub fn register(&mut self) -> Result<String, ZkError> {
        let mut registration_path = String::from(COORDINATOR_REGISTRATION_PATH);
        registration_path.push_str(&self.get_unique_hoststring());

        self.zk.create(&registration_path,
                       Vec::new(),
                       acls::OPEN_ACL_UNSAFE.clone(),
                       CreateMode::Ephemeral)
    }

    pub fn take_lock(&mut self) -> Result<(), ZkError> {
        let lock_path: String = self.zk
            .create(COORDINATOR_LOCK_PATH,
                    Vec::new(),
                    acls::OPEN_ACL_UNSAFE.clone(),
                    CreateMode::EphemeralSequential)?;

        loop {
            let children: Vec<String> = self.zk.get_children(COORDINATOR_LOCK_PATH, false)?;

            let this_seqid = get_sequence_number_from_path(&lock_path);

            let mut is_lowest = true;
            let mut next_lowest: Option<i32> = None;
            let mut next_lowest_path = None;

            for child in children {
                let child_seq_id = get_sequence_number_from_path(&child);

                if child_seq_id < this_seqid {
                    is_lowest = false;
                } else if next_lowest.is_none() ||
                          next_lowest.is_some() && next_lowest.unwrap() > child_seq_id {
                    next_lowest = Some(child_seq_id);
                    next_lowest_path = Some(child);
                }
            }

            if is_lowest {
                // We have the lock
                info!("We have obtained the lock");
                self.status = CoordinatorStatus::ACTIVE;
                break;
            } else {
                // We do not have the lock - watch the others
                match next_lowest_path {
                    Some(path) => {
                        match self.zk.exists(&path, true)? {
                            Some(stat) => info!("{} exists held by {}", path, stat.ephemeral_owner),
                            None => info!("{} does not exist!  Attempting to gain lock!", path),
                        };
                    }
                    None => warn!("No other children of the lock found, not watching for them"),
                }
            }
        }

        Ok(())
    }

    pub fn release_lock(&mut self) -> Result<(), ZkError> {
        match self.lock_path {
            Some(ref path) => {
                match self.zk.exists(&path, false)? {
                    Some(stat) => self.zk.delete(&path, stat.version),
                    None => {
                        println!("No lock held");
                        Ok(())
                    }
                }
            }
            None => {
                warn!("Released lock with no lock held");
                Ok(())
            }
        }
    }

    pub fn create_table(&self, table: Table) {}

    fn get_unique_hoststring(&self) -> String {
        format!("{}/{}", self.hostname, self.run_id)
    }
}

struct CoordinatorWatcher;

impl Watcher for CoordinatorWatcher {
    fn handle(&self, watched_event: WatchedEvent) {
        println!("{:?}", watched_event);
    }
}

fn get_sequence_number_from_path(lock_path: &String) -> i32 {
    let mut regex = String::from(COORDINATOR_LOCK_PATH);
    regex.push_str("(?P<sequence_number>\\d+)");

    let sequence_number_regex: Regex = Regex::new(&regex).unwrap();
    let caps_maybe: Option<Captures> = sequence_number_regex.captures(&lock_path);
    match caps_maybe {
        Some(caps) => {
            match caps.name("sequence_number") {
                Some(seq_num) => {
                    match i32::from_str_radix(seq_num, 10) {
                        Ok(val) => val,
                        Err(e) => panic!("Non-integer sequence number {}", e),
                    }
                }
                None => panic!("No match for sequence number"),
            }
        }
        None => {
            panic!("No captures found!");
        }
    }
}
