pub trait WithAssociatedType {
    type Associated;
}

pub type Associated<T> = <T as WithAssociatedType>::Associated;

#[macro_export]
macro_rules! associate {
    ($ty:ty, $assoc:path) => {
        impl associated::WithAssociatedType for $ty {
            type Associated = $assoc;
        }
    };
}
