use std::convert::From;
use std::error::Error;

use std::string::ParseError;
use std::string::FromUtf8Error;

use std::fmt;

use rocksdb::Error as RocksDbError;

use uuid::ParseError as UuidParseError;

use protobuf::ProtobufError;

use zookeeper::ZkError;

pub type CounterDbResult<T> = Result<T, CounterDbError>;

#[derive(Debug)]
pub enum CounterDbError {
    Parse(ParseError),
    FromUtf8(FromUtf8Error),
    RocksDb(RocksDbError),
    UuidParse(UuidParseError),
    Protobuf(ProtobufError),
    TableExists(String),
    Zk(ZkError),
    AlreadyInitialized,
    NotInitialized,
}

impl From<ParseError> for CounterDbError {
    fn from(e: ParseError) -> CounterDbError {
        CounterDbError::Parse(e)
    }
}

impl From<FromUtf8Error> for CounterDbError {
    fn from(e: FromUtf8Error) -> CounterDbError {
        CounterDbError::FromUtf8(e)
    }
}

impl From<RocksDbError> for CounterDbError {
    fn from(e: RocksDbError) -> CounterDbError {
        CounterDbError::RocksDb(e)
    }
}

impl From<UuidParseError> for CounterDbError {
    fn from(e: UuidParseError) -> CounterDbError {
        CounterDbError::UuidParse(e)
    }
}

impl From<ProtobufError> for CounterDbError {
    fn from(e: ProtobufError) -> CounterDbError {
        CounterDbError::Protobuf(e)
    }
}

impl From<ZkError> for CounterDbError {
    fn from(e: ZkError) -> CounterDbError {
        CounterDbError::Zk(e)
    }
}
impl Error for CounterDbError {
    fn description(&self) -> &str {
        match *self {
            CounterDbError::Parse(ref e) => e.description(),
            CounterDbError::FromUtf8(ref e) => e.description(),
            CounterDbError::RocksDb(ref e) => e.description(),
            CounterDbError::UuidParse(ref e) => e.description(),
            CounterDbError::Protobuf(ref e) => e.description(),
            CounterDbError::TableExists(_) => "Table already exists",
            CounterDbError::Zk(ref e) => e.description(),
            CounterDbError::AlreadyInitialized => "The registrar is already initialized",
            CounterDbError::NotInitialized => "The registrar is not initialized",
        }
    }

    fn cause(&self) -> Option<&Error> {
        match *self {
            CounterDbError::Parse(ref e) => e.cause(),
            CounterDbError::FromUtf8(ref e) => e.cause(),
            CounterDbError::RocksDb(ref e) => e.cause(),
            CounterDbError::UuidParse(ref e) => e.cause(),
            CounterDbError::Protobuf(ref e) => e.cause(),
            CounterDbError::TableExists(_) => None,
            CounterDbError::Zk(ref e) => e.cause(),
            CounterDbError::AlreadyInitialized => None,
            CounterDbError::NotInitialized => None,
        }
    }
}

impl fmt::Display for CounterDbError {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        match *self {
            CounterDbError::Parse(ref e) => e.fmt(formatter),
            CounterDbError::FromUtf8(ref e) => e.fmt(formatter),
            CounterDbError::RocksDb(ref e) => e.fmt(formatter),
            CounterDbError::UuidParse(ref e) => e.fmt(formatter),
            CounterDbError::Protobuf(ref e) => e.fmt(formatter),
            CounterDbError::TableExists(ref table) => write!(formatter, "TableExists: {}", table),
            CounterDbError::Zk(ref e) => e.fmt(formatter),
            CounterDbError::AlreadyInitialized => {
                write!(formatter, "The registrar is already initialized")
            }
            CounterDbError::NotInitialized => write!(formatter, "The registrar is not initialized"),
        }
    }
}
