#[cfg(feature = "fluent")]
mod inner {
    use crate::fluent::FluentState;

    pub trait State: Send + Sync + FluentState {}

    impl<T> State for T where T: Send + Sync + FluentState {}
}

#[cfg(not(feature = "fluent"))]
mod inner {
    pub trait State: Send + Sync {}

    impl<T> State for T where T: Send + Sync {}
}

pub use inner::State;
