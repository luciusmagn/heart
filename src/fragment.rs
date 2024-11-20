use crate::maud::{html, Markup, PreEscaped};
use tracing::error;

pub struct FragmentBuilder {
    main: Markup,
    oobs: Vec<Markup>,
}

impl FragmentBuilder {
    pub fn new<T: FragmentFinalizer>(main: T) -> Self {
        Self {
            main: main.to_markup(),
            oobs: vec![],
        }
    }

    pub fn oob<F>(mut self, oob: F) -> Self
    where
        F: FnOnce(bool) -> Markup,
    {
        self.oobs.push(oob(true));
        self
    }

    pub fn build(self) -> Markup {
        let oob_strings = self.oobs
            .into_iter()
            .map(|oob| oob.into_string())
            .enumerate()
            .inspect(|(i, o)| if !o.contains("hx-swap-oob") {
                error!("Fragment OOB {} does not contain any OOB swap: {}", i, o);
            })
            .map(|(_, o)| o);

        pub use crate::maud;
        html! {
            (self.main)
            @for oob in oob_strings {
                (PreEscaped(oob))
            }
        }
    }
}

pub fn compose<T: FragmentFinalizer>(
    main: T,
) -> FragmentBuilder {
    FragmentBuilder::new(main)
}

#[macro_export]
macro_rules! fragment {
    () => {
        impl ::std::ops::FnOnce(bool) -> ::heart::Markup
    };
}

#[macro_export]
macro_rules! htmx {
    ($swap:ident, $($toks:tt)*) => {{
        use heart::maud as maud;

        move |$swap: bool| {{
            let $swap = $swap.then_some("true");

            maud::html! {
                $($toks)*
            }
        }}
    }};
    ($($toks:tt)*) => {
        $crate::htmx!(__swap_unused, $($toks)*)
    };
}

pub trait FragmentFinalizer {
    fn into_string(self) -> String;
    fn to_markup(self) -> Markup;
}

impl<F> FragmentFinalizer for F
where
    F: FnOnce(bool) -> Markup,
{
    fn into_string(self) -> String {
        let markup = self(false);
        markup.into_string()
    }

    fn to_markup(self) -> Markup {
        self(false)
    }
}

impl FragmentFinalizer for Markup {
    fn into_string(self) -> String {
        self.into_string()
    }
    fn to_markup(self) -> Markup {
        self
    }
}
