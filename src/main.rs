use std::env;

mod wyrm;





fn main() {

    let args: Vec<String> = env::args().collect();

    let wyrm = wyrm::Wyrm::new();
    wyrm.run();
}
