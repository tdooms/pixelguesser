// TODO: this macro will accept raw true/false statements
// TODO: Is it possible to return a statically sized [] instead of vec?
#[macro_export]
macro_rules! classify {
    ( $( $x:ident ),* ) => {
        {
            let mut temp_vec = Vec::new();
            $(
                if($x) {
                    temp_vec.push(concat!("is-", stringify!($x)));
                }
            )*
            temp_vec
        }
    };
}
