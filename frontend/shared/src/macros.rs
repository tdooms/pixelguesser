#[macro_export]
macro_rules! spawn {
    ( $y:expr ) => {
        wasm_bindgen_futures::spawn_local($y)
    };
    ( $( $x:ident ),*; $y:expr ) => {
        {
            $(let $x = $x.clone();)*
            wasm_bindgen_futures::spawn_local($y)
        }
    };
}

#[macro_export]
macro_rules! callback {
    ( $y:expr ) => {
        yew::Callback::from($y)
    };
    ( $( $x:ident ),*; $y:expr ) => {
        {
            $(let $x = $x.clone();)*
            yew::Callback::from($y)
        }
    };
}

#[macro_export]
macro_rules! clone(
    ( $( $x:ident ),* ) => {
        $(let $x = $x.clone();)*
    }
);
