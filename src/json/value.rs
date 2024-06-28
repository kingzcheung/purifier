use std::collections::HashMap;

#[derive(Debug, PartialEq,Clone)]
pub enum Json {
    Null,
    String(String),
    Number(f64),
    Boolean(bool),
    Array(Vec<Json>),
    Object(HashMap<String, Json>),
}

impl Json {
    pub fn is_null(&self) -> bool {
        matches!(*self, Json::Null)
    }

    pub fn is_string(&self) -> bool {
        matches!(*self, Json::String(_))
    }

    pub fn is_number(&self) -> bool {
        matches!(*self, Json::Number(_))
    }

    pub fn is_boolean(&self) -> bool {
        matches!(*self, Json::Boolean(_))
    }

    pub fn is_array(&self) -> bool {
        matches!(*self, Json::Array(_))
    }

    pub fn is_object(&self) -> bool {
        matches!(*self, Json::Object(_))
    }
}