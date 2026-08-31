use std::env;

mod config;
mod wyrm;




fn main() {

    let args: Vec<String> = env::args().collect();
    let config = config::Config::build(args.into_iter());
    let wyrm = wyrm::Wyrm::new(config.get_verbose());
    wyrm.run();
}
