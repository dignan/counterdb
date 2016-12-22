extern crate rocksdb;
extern crate byteorder;

// necessary for grpc
extern crate protobuf;
extern crate grpc;
extern crate futures;
extern crate futures_cpupool;

use rocksdb::DB;
use rocksdb::MergeOperands;
use rocksdb::Error;
use rocksdb::Options;

use byteorder::BigEndian;
use byteorder::ByteOrder;

static I64_BYTES: usize = 8;

pub mod client_protocol_grpc;
pub mod client_protocol;

/**
 * A merge operator that increments the input key
 */
fn increment_merge(key: &[u8], existing_val: Option<&[u8]>, operands: &mut MergeOperands) -> Vec<u8> {
    let starting_val = match existing_val {
        Some(val) => BigEndian::read_i64(val),
        None => 0
    };

    let mut operand_counter = 0;
    let mut buf = get_i64_buf();

    for operand in operands {
        BigEndian::write_i64(&mut buf, starting_val + BigEndian::read_i64(operand));
        operand_counter += 1;
    }

    if operand_counter == 0 {
        panic!("Did not receive a merge operand in increment_merge!  How did we get here?  Time to panic");
    }

    buf
}

pub fn increment(db: &DB, key: &[u8], add: i64) -> Result<(), Error> {
    let mut buf = get_i64_buf();
    BigEndian::write_i64(&mut buf, add);
    db.merge(key, &buf)
}

pub fn read(db: &DB, key: &[u8]) -> Result<Option<i64>, Error> {
    match db.get(key) {
        Ok(db_vec_maybe) => match db_vec_maybe {
            Some(vec) => {
                Ok(Some(BigEndian::read_i64(&vec)))
            },
            None => Ok(None)
        },
        Err(e) => Err(e)
    }
}

pub fn set(db: &DB, key: &[u8], value: i64) -> Result<(), Error> {
    let mut buf = get_i64_buf();
    BigEndian::write_i64(&mut buf, value);
    db.put(key, &buf)
}

pub fn get_db_options() -> Options {
    let mut opts = Options::default();
    opts.set_merge_operator("increment", increment_merge);
    opts.create_if_missing(true);
    opts
}

fn get_i64_buf() -> Vec<u8> {
    let mut buf = Vec::with_capacity(I64_BYTES);
    for _ in 0..I64_BYTES {
        buf.push(0);
    }
    buf
}

#[cfg(test)]
mod tests {
    extern crate rocksdb;

    use std::fs::remove_dir_all;

    use rocksdb::DB;
    use rocksdb::Options;

    use super::set;
    use super::increment;
    use super::increment_merge;
    use super::read;
    use super::get_db_options;

    #[test]
    fn test_increment_with_no_existing_value() {
        let test_key = "test-key".as_bytes();
        let test_db_path = "test-db.rdb";

        { // Create a new scope so the DB closes and we can delete the file
            let opts = get_db_options();
            let db: DB = DB::open(&opts, test_db_path).unwrap();
            set(&db, test_key, 5).unwrap();
            assert_eq!(5 as i64, read(&db, test_key).unwrap().unwrap());
            increment(&db, test_key, 4).unwrap();
            assert_eq!(9 as i64, read(&db, test_key).unwrap().unwrap());
        }

        remove_dir_all(test_db_path).unwrap();
    }
}
