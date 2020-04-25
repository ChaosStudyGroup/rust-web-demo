#[macro_export]
macro_rules! hashmap {
    ($($key: expr => $val: expr),*) => (
        {
            let mut _map = std::collections::HashMap::new();
            $(_map.insert($key, $val);)*
            _map
        }
    );
    ($($key: expr => $val: expr,)*) => (crate::hashmap!{$($key => $val),*});
}
