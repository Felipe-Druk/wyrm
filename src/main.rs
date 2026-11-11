use std::env;
use wyrm_lib::{config, wyrm};

fn main() {
    let args: Vec<String> = env::args().collect();
    let config = config::Config::build(args.into_iter());
    let wyrm = wyrm::Wyrm::new(config.get_verbose(), config.get_file_path().cloned());
    wyrm.run();
}
