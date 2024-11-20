#[macro_export]
macro_rules! filter {
    () => {
        impl heart::Filter<Extract = impl heart::Reply, Error = heart::Rejection> + Clone
    };
}

#[macro_export]
macro_rules! html {
    ($($tok:tt)*) => {{
        use $crate::maud as maud;

        maud::html! {
            $($tok)*
        }
    }}
}

#[test]
pub fn test_html() {
    let m = html! {
        div {}
    };
}
