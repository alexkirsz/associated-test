use associated::associate;

pub trait Trait2 {}

pub struct Trait2Associated {
    pub trait_2: (),
}

associate!(dyn Trait2, Trait2Associated);
