use uuid::Uuid;

use error::CounterDbResult;

use protocol::coordinator::PartitionProto;
use protocol::coordinator::PartitionLocationProto;
use protocol::coordinator::KeyRangeProto;

#[derive(Clone)]
pub struct Table {
    name: String,
    partitions: Vec<Partition>
}

impl Table {
    pub fn with_name(name: String) -> Table {
        Table {
            name: name,
            partitions: Vec::new()
        }
    }

    pub fn get_name(&self) -> String {
        self.name.clone()
    }

    pub fn add_partition(&mut self, partition: Partition) {
        self.partitions.push(partition);
    }
}

#[derive(Clone)]
pub struct KeyRange {
    start_key: Option<Vec<u8>>,
    end_key: Option<Vec<u8>>
}

impl From<KeyRangeProto> for KeyRange {
    fn from(proto: KeyRangeProto) -> KeyRange {
        let skey = match proto.has_start_key() {
            true => Some(proto.get_start_key().to_vec()),
            false => None
        };

        let ekey = match proto.has_end_key() {
            true => Some(proto.get_end_key().to_vec()),
            false => None
        };

        KeyRange {
            start_key: skey,
            end_key: ekey
        }
    }
}

#[derive(Clone)]
pub struct Partition {
    id: Uuid,
    location: PartitionLocation,
    key_range: KeyRange
}

impl Partition {
    pub fn from_proto(mut proto: PartitionProto) -> CounterDbResult<Partition> {
        let id = Uuid::from_bytes(proto.get_id())?;

        Ok(Partition {
            id: id,
            location: PartitionLocation::from_proto(proto.take_partition_location()),
            key_range: KeyRange::from(proto.take_key_range())
        })
    }
}

#[derive(Clone)]
pub struct PartitionLocation {
    hostname: String,
    port: u16
}

impl PartitionLocation {
    fn from_proto(mut proto: PartitionLocationProto) -> PartitionLocation {
        PartitionLocation {
            port: proto.get_port() as u16,
            hostname: proto.take_hostname()
        }
    }
}