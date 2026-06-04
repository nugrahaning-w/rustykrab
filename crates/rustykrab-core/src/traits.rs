pub trait Validate {
    fn validate(&self) -> bool;
}

pub trait Identifiable {
    fn id(&self) -> String;
}
