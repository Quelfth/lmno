use std::collections::HashMap;

#[derive(Clone, PartialEq, Debug)]
pub enum Value {
    Empty,
    Bool(bool),
    Integer(i128),
    Float(f64),
    String(String),
    List(Vec<Value>),
    Map(HashMap<String, Value>),
}

impl Value {
    pub fn is_empty(&self) -> bool {
        matches!(self, Self::Empty)
    }

    pub fn is_bool(&self) -> bool {
        matches!(self, Self::Bool(_))
    }

    pub fn is_integer(&self) -> bool {
        matches!(self, Self::Integer(_))
    }

    pub fn is_float(&self) -> bool {
        matches!(self, Self::Float(_))
    }

    pub fn is_string(&self) -> bool {
        matches!(self, Self::String(_))
    }

    pub fn is_list(&self) -> bool {
        matches!(self, Self::List(_))
    }
    
    pub fn is_map(&self) -> bool {
        matches!(self, Self::Map(_))
    }

    pub fn to_bool(&self) -> Option<bool> {
        if let Self::Bool(value) = self {
            Some(*value)
        } else {
            None
        }
    }

    pub fn to_integer(&self) -> Option<i128> {
        if let Self::Integer(value) = self {
            Some(*value)
        } else {
            None
        }
    }

    pub fn to_float(&self) -> Option<f64> {
        if let Self::Float(value) = self {
            Some(*value)
        } else {
            None
        }
    }

    pub fn to_string(&self) -> Option<String> {
        if let Self::String(value) = self {
            Some(value.clone())
        } else {
            None
        }
    }

    pub fn to_list(&self) -> Option<Vec<Value>> {
        if let Self::List(value) = self {
            Some(value.clone())
        } else {
            None
        }
    }

    pub fn to_map(&self) -> Option<HashMap<String, Value>> {
        if let Self::Map(value) = self {
            Some(value.clone())
        } else {
            None
        }
    }

    pub fn as_string(&self) -> Option<&str> {
        if let Self::String(value) = self {
            Some(value)
        } else {
            None
        }
    }

    pub fn as_list(&self) -> Option<&[Value]> {
        if let Self::List(value) = self {
            Some(value)
        } else {
            None
        }
    }

    pub fn as_map(&self) -> Option<&HashMap<String, Value>> {
        if let Self::Map(value) = self {
            Some(value)
        } else {
            None
        }
    }

    pub fn into_string(self) -> Result<String, Self> {
        if let Self::String(value) = self {
            Ok(value)
        } else {
            Err(self)
        }
    }

    pub fn into_list(self) -> Result<Vec<Value>, Self> {
        if let Self::List(value) = self {
            Ok(value)
        } else {
            Err(self)
        }
    }

    pub fn into_map(self) -> Result<HashMap<String, Value>, Self> {
        if let Self::Map(value) = self {
            Ok(value)
        } else {
            Err(self)
        }
    }

    pub fn as_bool_mut(&mut self) -> Option<&mut bool> {
        if let Self::Bool(value) = self {
            Some(value)
        } else {
            None
        }
    }

    pub fn as_integer_mut(&mut self) -> Option<&mut i128> {
        if let Self::Integer(value) = self {
            Some(value)
        } else {
            None
        }
    }

    pub fn as_float_mut(&mut self) -> Option<&mut f64> {
        if let Self::Float(value) = self {
            Some(value)
        } else {
            None
        }
    }

    pub fn as_string_mut(&mut self) -> Option<&mut String> {
        if let Self::String(value) = self {
            Some(value)
        } else {
            None
        }
    }

    pub fn as_list_mut(&mut self) -> Option<&mut Vec<Value>> {
        if let Self::List(value) = self {
            Some(value)
        } else {
            None
        }
    }

    pub fn as_map_mut(&mut self) -> Option<&mut HashMap<String, Value>> {
        if let Self::Map(value) = self {
            Some(value)
        } else {
            None
        }
    }
}
