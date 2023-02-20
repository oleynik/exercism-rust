#[macro_export]
macro_rules! hashmap {
    () => {
        ::std::collections::HashMap::new()
    };
    ($($key:expr => $value:expr),+ $(,)?) => {
        {
            use ::std::collections::HashMap;
            let mut map = HashMap::new();
            $(
                map.insert($key, $value);
            )+
            map
        }
    };
}
