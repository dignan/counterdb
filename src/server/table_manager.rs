use std::path::Path;

use rocksdb::DB;
use rocksdb::DBIterator;
use rocksdb::Direction;
use rocksdb::Options;
use rocksdb::IteratorMode;

use protobuf;

use super::table::Table;
use super::table::Partition;

use protocol::coordinator::PartitionProto;

use error::CounterDbResult;

pub struct TableManager {
    db: DB
}

impl TableManager {
    pub fn new<P: AsRef<Path>>(path: P) -> CounterDbResult<TableManager> {
        let db = DB::open(&get_db_opts(), path)?;

        Ok(TableManager {
            db: db
        })
    }

    pub fn create_table(&self, table: Table, num_splits: i8) -> CounterDbResult<Table> {
        unimplemented!();
    }

    pub fn get_table(&self, table_name: String) -> CounterDbResult<Option<Table>> {
        let tables_result: CounterDbResult<Vec<Table>> = self.get_tables_internal(Some(table_name.as_bytes()), true);
        match tables_result {
            Ok(tables) => match tables.len() {
                0 => Ok(None),
                1 => Ok(Some(tables.get(0).expect("No tables despite matching on the length of the tables vec being 1.  This is a bug somehow, or hardware failure").clone())),
                _ => panic!("The internal table search should have stopped after the first table, but didn't")
            },
            Err(e) => Err(e)
        }
    }

    pub fn get_tables(&self) -> CounterDbResult<Vec<Table>> {
        self.get_tables_internal(None, false)
    }

    fn get_tables_internal(&self, start_key: Option<&[u8]>, find_single_table: bool) -> CounterDbResult<Vec<Table>> {
        let mut iterator: DBIterator = match start_key {
            Some(skey) => self.db.iterator(IteratorMode::From(skey, Direction::Forward)),
            None => self.db.iterator(IteratorMode::Start)
        };

        let mut tables: Vec<Table> = Vec::new();
        let mut current_table: Option<Table> = None;
        let mut temp_table = None;

        for (key, value) in iterator {
            match current_table {
                Some(ref table) => {
                    let table_name = String::from_utf8(Vec::from(&key[0..key.len()-16]))?;

                    if table_name != table.get_name() {
                        tables.push(table.clone());

                        let mut new_table = Table::with_name(table_name);
                        new_table.add_partition(get_partition_from_bytes(&value)?);

                        temp_table = Some(new_table);

                        if find_single_table {
                            break;
                        }
                    }
                },
                None => {
                    let table_name = String::from_utf8(Vec::from(&key[0..key.len()-16]))?;
                    let mut table = Table::with_name(table_name);
                    table.add_partition(get_partition_from_bytes(&value)?);

                    temp_table = Some(table);

                    if find_single_table {
                        break;
                    }
                }
            }

            current_table = temp_table.clone();
        }

        match current_table {
            Some(table) => tables.push(table),
            None => {}
        }

        Ok(tables)
    }
}

fn get_partition_from_bytes(bytes: &[u8]) -> CounterDbResult<Partition> {
    Partition::from_proto(protobuf::parse_from_bytes::<PartitionProto>(bytes)?)
}

fn get_db_opts() -> Options {
    let mut opts = Options::default();
    opts.create_if_missing(true);
    opts
}