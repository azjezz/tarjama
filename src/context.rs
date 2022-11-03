use std::fmt::Display;

#[derive(Clone, PartialEq, Debug, Default)]
pub struct Context {
    pub values: Vec<(String, Value)>,
    pub count: Option<i64>,
}

impl Context {
    pub fn new(values: Vec<(String, Value)>, count: Option<i64>) -> Context {
        Context { values, count }
    }
}

impl From<Option<i64>> for Context {
    fn from(value: Option<i64>) -> Self {
        Context { values: vec![], count: value }
    }
}

#[derive(Clone, PartialEq, Debug)]
pub enum Value {
    String(String),
    Integer(i64),
    Double(f64),
}

impl Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Value::String(s) => write!(f, "{}", s),
            Value::Integer(i) => write!(f, "{}", i),
            Value::Double(d) => write!(f, "{}", d),
        }
    }
}

macro_rules! implement_string {
    ($t:ty) => {
        impl From<$t> for Value {
            fn from(value: $t) -> Self {
                Value::String(value.into())
            }
        }
    };
}

macro_rules! implement_integer {
    ($t:ty) => {
        impl From<$t> for Value {
            fn from(value: $t) -> Self {
                Value::Integer(value.into())
            }
        }

        impl From<$t> for Context {
            fn from(value: $t) -> Self {
                Context { values: vec![], count: Some(value.into()) }
            }
        }
    };
}

macro_rules! implement_double {
    ($t:ty) => {
        impl From<$t> for Value {
            fn from(value: $t) -> Self {
                Value::Double(value.into())
            }
        }
    };
}

implement_string!(String);
implement_string!(&str);

implement_integer!(i64);
implement_integer!(i32);
implement_integer!(i16);
implement_integer!(i8);
implement_integer!(u32);
implement_integer!(u16);
implement_integer!(u8);

implement_double!(f64);
implement_double!(f32);
