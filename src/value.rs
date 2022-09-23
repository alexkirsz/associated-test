use associated::{associate, Associated};

use crate::{trait1::Trait1, trait2::Trait2};

pub struct Value;

pub struct ValueAssociated;

associate!(Value, ValueAssociated);

impl From<Value> for Associated<dyn Trait1> {
    fn from(_value: Value) -> Self {
        Self { trait_1: () }
    }
}

impl From<Value> for Associated<dyn Trait2> {
    fn from(_value: Value) -> Self {
        Self { trait_2: () }
    }
}
