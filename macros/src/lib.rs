#[macro_export]
macro_rules! hashmap {
    () => {{
        ::std::collections::HashMap::new()
    }};
    ( $( $k:expr => $v:expr ),+  $(,)? ) => {
        {
            let mut map = ::std::collections::HashMap::new();
            $(
                map.insert($k, $v);
            )*
            map
        }
    };
    
}
