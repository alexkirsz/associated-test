use associated::associate;

pub trait Trait1 {}

pub struct Trait1Associated {
    pub trait_1: (),
}

associate!(dyn Trait1, Trait1Associated);
