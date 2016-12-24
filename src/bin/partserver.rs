extern crate counterdb;

#[macro_use] extern crate log;
extern crate log4rs;

extern crate clap;

use std::thread;

use log::LogLevelFilter;

use log4rs::append::file::FileAppender;

use log4rs::config::Appender;
use log4rs::config::Config;
use log4rs::config::Logger;
use log4rs::config::Root;

use clap::Arg;
use clap::App;
use clap::SubCommand;

use counterdb::configuration::server_config::PartServerConfig;
use counterdb::configuration::server_config::read_part_server_config;

use counterdb::protocol::client_grpc::PartServerServer;

use counterdb::server::partserverrpc::PartServerImpl;

fn create_parser<'a, 'b>() -> App<'a, 'b> {
    App::new("partserver")
        .version("0.0.1")
        .about("Host partitions for counterdb")
        .arg(Arg::with_name("config")
            .short("c")
            .long("config")
            .value_name("CONFIG")
            .help("where the partserver will read its config from.  Defaults will be used if this is unspecified")
            .takes_value(true))
}

fn main() {
    let cli_parser = create_parser();

    let matches = cli_parser.get_matches();

    let partserver_config: PartServerConfig<String> = match matches.value_of("config") {
        Some(config_filename) => {
            match read_part_server_config::<&str, String>(config_filename) {
                Ok(config) => config,
                Err(e) => panic!("Could not read config {}", e)
            }
        },
        None => PartServerConfig::default()
    };

    let file_appender = FileAppender::builder()
        .build(format!("{}/{}", partserver_config.log_dir, "partserver.log")).unwrap();

    let log_config = log4rs::config::Config::builder()
        .appender(Appender::builder().build("file", Box::new(file_appender)))
        .build(Root::builder().appender("file").build(LogLevelFilter::Info)).unwrap();

    log4rs::init_config(log_config).unwrap();
    info!("Starting partserver on port {}", partserver_config.port);

    let server_impl = PartServerImpl::new();

    PartServerServer::new(partserver_config.port, server_impl);

    info!("Partserver started");

    loop {
        thread::park();
    }
}
