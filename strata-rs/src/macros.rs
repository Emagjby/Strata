#[macro_export]
macro_rules! null {
    () => {
        $crate::value::Value::Null
    };
}

#[macro_export]
macro_rules! bool {
    ($val:expr) => {
        $crate::value::Value::Bool($val)
    };
}

#[macro_export]
macro_rules! int {
    ($val:expr) => {
        $crate::value::Value::Int($val)
    };
}

#[macro_export]
macro_rules! string {
    ($val:expr) => {
        $crate::value::Value::String($val.to_string())
    };
}

#[macro_export]
macro_rules! bytes {
    // Explicit byte array literal form: bytes!([0xde, 0xad])
    ([$($val:expr),* $(,)?]) => {
        $crate::value::Value::Bytes(vec![$($val as u8),*])
    };
    // Generic expression form: bytes!(some_expression)
    ($val:expr) => {
        $crate::value::Value::Bytes(($val).into())
    };
}

#[macro_export]
macro_rules! list {
    // empty list![]
    () => {
        $crate::value::Value::List(Vec::new())
    };
    // list![a, b, c]
    ($($val:expr),* $(,)?) => {{
        $crate::value::Value::List(vec![$($val),*])
    }};
}
#[macro_export]
macro_rules! map {
    // empty map!{}
    () => {{
        $crate::value::Value::Map(::std::collections::BTreeMap::new())
    }};
    // map!{ "k" => v, ... }
    ($($key:literal => $val:expr),* $(,)?) => {{
        let mut m = ::std::collections::BTreeMap::new();
        $(
            // last-write-wins naturally via insert overwrite
            m.insert(($key).to_owned(), $val);
        )*
        $crate::value::Value::Map(m)
    }};
}
