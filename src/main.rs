use std::env;

pub mod ast_node;
mod config;
mod parser;
mod scanner;
pub mod token_vector;
mod wyrm;

fn main() {
    let args: Vec<String> = env::args().collect();
    let config = config::Config::build(args.into_iter());
    let wyrm = wyrm::Wyrm::new(config.get_verbose());
    wyrm.run();
}
