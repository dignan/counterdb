use uuid::Uuid;

use error::CounterDbResult;

use protocol::coordinator::PartitionProto;
use protocol::coordinator::PartitionLocationProto;
use protocol::coordinator::KeyRangeProto;

#[derive(Clone)]
pub struct Table {
    name: String,
    partitions: Vec<Partition>,
}

impl Table {
    pub fn with_name(name: String) -> Table {
        Table {
            name: name,
            partitions: Vec::new(),
        }
    }

    pub fn name(&self) -> &String {
        &self.name
    }

    pub fn partitions(&self) -> &Vec<Partition> {
        &self.partitions
    }

    pub fn add_partition(&mut self, partition: Partition) {
        self.partitions.push(partition);
    }
}

#[derive(Clone)]
pub struct KeyRange {
    start_key: Option<Vec<u8>>,
    end_key: Option<Vec<u8>>,
}

impl KeyRange {
    fn start_key(&self) -> &Option<Vec<u8>> {
        &self.start_key
    }

    fn end_key(&self) -> &Option<Vec<u8>> {
        &self.end_key
    }
}

impl From<KeyRangeProto> for KeyRange {
    fn from(proto: KeyRangeProto) -> KeyRange {
        let skey = match proto.has_start_key() {
            true => Some(proto.get_start_key().to_vec()),
            false => None,
        };

        let ekey = match proto.has_end_key() {
            true => Some(proto.get_end_key().to_vec()),
            false => None,
        };

        KeyRange {
            start_key: skey,
            end_key: ekey,
        }
    }
}

impl<'a> From<&'a KeyRange> for KeyRangeProto {
    fn from(key_range: &'a KeyRange) -> KeyRangeProto {
        let mut proto = KeyRangeProto::new();

        if key_range.start_key.is_some() {
            proto.set_start_key(key_range.start_key()
                .clone()
                .unwrap()
                .to_vec())
        }

        if key_range.end_key.is_some() {
            proto.set_end_key(key_range.end_key()
                .clone()
                .unwrap()
                .to_vec())
        }

        proto
    }
}

#[derive(Clone)]
pub struct Partition {
    id: Uuid,
    location: PartitionLocation,
    key_range: KeyRange,
}

impl Partition {
    pub fn from_proto(mut proto: PartitionProto) -> CounterDbResult<Partition> {
        let id = Uuid::from_bytes(proto.get_id())?;

        Ok(Partition {
            id: id,
            location: PartitionLocation::from_proto(proto.take_partition_location()),
            key_range: KeyRange::from(proto.take_key_range()),
        })
    }

    pub fn id(&self) -> &Uuid {
        &self.id
    }

    pub fn location(&self) -> &PartitionLocation {
        &self.location
    }

    pub fn key_range(&self) -> &KeyRange {
        &self.key_range
    }
}

impl<'a> From<&'a Partition> for PartitionProto {
    fn from(partition: &'a Partition) -> PartitionProto {
        let mut proto = PartitionProto::new();

        proto.set_id(partition.id().as_bytes().clone().to_vec());
        proto.set_key_range(KeyRangeProto::from(partition.key_range()));
        proto.set_partition_location(PartitionLocationProto::from(partition.location()));

        proto
    }
}

#[derive(Clone)]
pub struct PartitionLocation {
    hostname: String,
    port: u16,
}

impl PartitionLocation {
    fn from_proto(mut proto: PartitionLocationProto) -> PartitionLocation {
        PartitionLocation {
            port: proto.get_port() as u16,
            hostname: proto.take_hostname(),
        }
    }

    fn hostname(&self) -> &String {
        &self.hostname
    }

    fn port(&self) -> u16 {
        self.port
    }
}

impl<'a> From<&'a PartitionLocation> for PartitionLocationProto {
    fn from(location: &'a PartitionLocation) -> PartitionLocationProto {
        let mut proto = PartitionLocationProto::new();

        proto.set_hostname(location.hostname().clone());
        proto.set_port(location.port() as u32);

        proto
    }
}
