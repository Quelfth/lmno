use std::collections::HashMap;

use crate::{
    Error,
    Value,
    line::LmnoLines,
    header::HeaderValue,
    scalar::{
        parse_raw_string,
        parse_string,
    },
};

pub fn parse(src: &str) -> Result<Value, Error> {

    enum Collection {
        Empty,
        List(Vec<Value>),
        Map(HashMap<String, Value>),
        String {
            quoted: bool,
            string: String,
        },
        Single(Value),
    }

    impl Collection {
        fn into_value(self) -> Value {
            match self {
                Collection::Empty => Value::Empty,
                Collection::List(values) => Value::List(values),
                Collection::Map(hash_map) => Value::Map(hash_map),
                Collection::String { string, .. } => Value::String(string),
                Collection::Single(value) => value,
            }
        }

        fn insert(&mut self, key: Option<String>, value: Value) -> Result<(), Error> {
            match self {
               Collection::Empty => {
                   if let Some(key) = key {
                       let mut map = HashMap::new();
                       map.insert(key, value);
                       *self = Collection::Map(map);
                   } else {
                       *self = Collection::List(vec![value]);
                   }
               },
               Collection::List(list) => {
                   if key.is_some() {
                       return Err(Error::MismatchedEntry)
                   }
                   list.push(value);
               },
               Collection::Map(map) => {
                   let Some(key) = key else {
                       return Err(Error::MismatchedEntry)
                   };
                   map.insert(key, value);
               },
               Collection::Single(_) => {
                   return Err(Error::ValueInScalar)
               }
               _ => panic!("cannot insert into a string collection"),
            };
            Ok(())
        }
    }

    let mut stack = Vec::new();
    stack.push((None, None, Collection::Empty));

    for line in LmnoLines::new(src) {
        let line = line?;
        while stack.last().unwrap().0.is_some_and(|i| line.indent <= i) {
             let (_, key, value) = stack.pop().unwrap();
             stack.last_mut().unwrap().2.insert(key, value.into_value())?;
        }
        if let (_, _, Collection::String { quoted, string }) = stack.last_mut().unwrap() {
            if *quoted {
                match line.text.chars().next().unwrap() {
                    '"' => {
                        let (content, rest) = parse_string(line.text)?;
                        if !rest.starts_with('#') {
                            return Err(Error::TrailingContent);
                        }
                        string.push_str(&content);
                    }
                    '\'' => {
                        let (content, rest) = parse_raw_string(line.text)?;
                        if !rest.starts_with('#') {
                            return Err(Error::TrailingContent);
                        }
                        string.push_str(&content);
                    }
                    _ => return Err(Error::InvalidLiteral),
                }
            } else {
                string.push_str(line.text);
            }
            continue;
        }

        let header = line.parse_header()?;
        match header.value {
            None => {
                stack.push((Some(line.indent), header.key, Collection::Empty))
            }
            Some(HeaderValue::String { quoted, text }) => {
                stack.push((Some(line.indent), header.key, Collection::String { quoted, string: text }))
            }
            Some(value) => stack.push((Some(line.indent), header.key, Collection::Single(value.into_value()))),
        }
    }

    while stack.last().unwrap().0.is_some() {
         let (_, key, value) = stack.pop().unwrap();
         stack.last_mut().unwrap().2.insert(key, value.into_value())?;
    }

    Ok(stack.pop().unwrap().2.into_value())
}