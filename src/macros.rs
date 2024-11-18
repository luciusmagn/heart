#[macro_export]
macro_rules! filter {
    () => {
        impl heart::Filter<Extract = impl heart::Reply, Error = heart::Rejection> + Clone
    };
}
