use rustykrab_core::*;

struct User;

impl Validate for User {
    fn validate(&self) -> bool {
        true
    }
}

impl Identifiable for User {
    fn id(&self) -> String {
        "123".into()
    }
}

fn main() {
    let user = User;

    println!("{}", user.validate());
    println!("{}", user.id());
}