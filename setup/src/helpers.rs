use config::Config;
use std::collections::HashMap;

pub fn read_config_file() -> HashMap<String, String> {
    let file = concat!(env!("CARGO_MANIFEST_DIR"), "/config");
    let config = Config::builder()
        .add_source(config::File::with_name(file))
        .build()
        .unwrap();

    config.try_deserialize::<HashMap<String, String>>().unwrap()
}
