use std::fmt::Debug;

use std::fs::File;

use std::io::Error;
use std::io::Read;

use std::path::Path;

use toml::Parser;
use toml::Table;

pub struct PartServerConfig<P: AsRef<Path>> {
    pub data_dirs: Vec<P>,
    pub log_dir: P,
    pub hostname: String,
    pub port: u16,
    pub zk_connect_string: String,
}

pub struct CoordinatorConfig<P: AsRef<Path>> {
    pub meta_dir: P,
    pub log_dir: P,
    pub hostname: String,
    pub port: u16,
    pub zk_connect_string: String,
}

impl Default for PartServerConfig<String> {
    fn default() -> Self {
        let mut data_dirs_default = Vec::new();
        data_dirs_default.push(String::from("/server/partserver/data/"));

        PartServerConfig {
            data_dirs: data_dirs_default,
            log_dir: String::from("/srv/partserver/logs/"),
            hostname: String::from("localhost"),
            port: 50001,
            zk_connect_string: String::from("localhost:2181"),
        }
    }
}

impl Default for CoordinatorConfig<String> {
    fn default() -> Self {
        CoordinatorConfig {
            meta_dir: String::from("/srv/coordinator/data/"),
            log_dir: String::from("/srv/coordinator/logs/"),
            hostname: String::from("localhost"),
            port: 60001,
            zk_connect_string: String::from("localhost"),
        }
    }
}

pub fn read_part_server_config<P, T>(path: P) -> Result<PartServerConfig<String>, Error>
    where P: AsRef<Path> + Debug
{
    let mut config = PartServerConfig::default();

    match get_toml_table(&path)? {
        Some(table) => {
            for (key, value) in table {
                match key.as_str() {
                    "data_dirs" => {
                        match value.as_str() {
                            Some(val) => {
                                config.data_dirs = val.split(",").map(|s| s.to_string()).collect()
                            }
                            None => warn!("Couldn't parse {} as a string", value),
                        }
                    }
                    "log_dir" => {
                        match value.as_str() {
                            Some(val) => config.log_dir = String::from(val),
                            None => warn!("Couldn't parse logdir value {} as a string", value),
                        }
                    }
                    "hostname" => {
                        match value.as_str() {
                            Some(val) => config.hostname = String::from(val),
                            None => warn!("Couldn't parse hostname value {} as a string", value),
                        }
                    }
                    "port" => {
                        match value.as_integer() {
                            Some(val) => config.port = val as u16,
                            None => warn!("Couldn't parse port value {} as an integer", value),
                        }
                    }
                    "zk_connect_string" => {
                        match value.as_str() {
                            Some(val) => config.zk_connect_string = String::from(val),
                            None => {
                                warn!("Couldn't parse zk_connect_string value {} as an integer",
                                      value)
                            }
                        }
                    }
                    &_ => warn!("Unknown pattern {}", key),
                };
            }
        }
        None => panic!("No configuration found in {:?}", path),
    }

    Ok(config)
}

fn get_toml_table<P: AsRef<Path> + Debug>(path: P) -> Result<Option<Table>, Error> {
    let mut file: File = File::open(path)?;
    let mut config_string = String::new();

    file.read_to_string(&mut config_string)?;

    Ok(Parser::new(&config_string).parse())
}
