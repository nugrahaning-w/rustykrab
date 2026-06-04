use rustykrab_core::*;

fn main() {
    let config = RustyKrabConfig {
        target_ios: true,
        target_android: false,
    };

    println!("{:?}", config);
}