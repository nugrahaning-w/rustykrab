use rustykrab_core::*;

fn main() {
    let err = RustyKrabError::Parsing("invalid widget".into());

    println!("{}", err);
}
