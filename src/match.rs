use std::fmt::Debug;

pub trait Match: Debug {
    fn message(&self) -> String;
}
