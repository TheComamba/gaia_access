use std::hash::Hash;

pub trait Schema {
    fn string(&self) -> String;
}

pub trait Table {
    fn string(&self) -> String;
}

pub trait Column: ToString + Eq + Hash + Clone + Copy {}
