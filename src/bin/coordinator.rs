extern crate counterdb;

#[macro_use]
extern crate log;
extern crate log4rs;

extern crate clap;

use std::thread;

use log::LogLevelFilter;

use log4rs::append::file::FileAppender;

use log4rs::config::Appender;
use log4rs::config::Root;

use clap::Arg;
use clap::App;

use counterdb::configuration::server_config::PartServerConfig;
use counterdb::configuration::server_config::read_part_server_config;

use counterdb::protocol::client_grpc::PartServerServer;

use counterdb::server::partserverrpc::PartServerImpl;

fn create_parser<'a, 'b>() -> App<'a, 'b> {
    App::new("coordinator")
        .version("0.0.1")
        .about("Perform coordination tasks for counterdb")
        .arg(Arg::with_name("config")
            .short("c")
            .long("config")
            .value_name("CONFIG")
            .help("where the coordinator will read its config from.  Defaults will be used if \
                   this is unspecified")
            .takes_value(true))
}

fn main() {}
