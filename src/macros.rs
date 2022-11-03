#[macro_export]
macro_rules! context {
    ($($k:ident = $v:expr),+) => (
        {
            let values: Vec<(String, $crate::context::Value)> = vec![$(
                (stringify!($k).to_string(), $crate::context::Value::from($v)),
            )*];

            $crate::context::Context::new(values, None)
        }
    );
    ($($k:ident = $v:expr),+, ? = $c:expr) => (
        {
            let values: Vec<(String, $crate::context::Value)> = vec![$(
                (stringify!($k).to_string(), $crate::context::Value::from($v)),
            )*];

            $crate::context::Context::new(values, Some($c))
        }
    );
    (? = $c:expr) => (
        {
            $crate::context::Context::new(vec![], Some($c))
        }
    );
    () => (
        {
            $crate::context::Context::new(vec![], None)
        }
    );
}
