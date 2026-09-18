use crate::{
    Error,
    Value,
    line::LmnoLine,
    scalar::{
        parse_number_or_bool,
        parse_raw_string,
        parse_string
    },
};

pub enum HeaderValue {
    Bool(bool),
    Integer(i128),
    Float(f64),
    String {
        quoted: bool,
        text: String,
    },
}

impl HeaderValue {
    pub fn into_value(self) -> Value {
        match self {
            HeaderValue::Bool(bool) => Value::Bool(bool),
            HeaderValue::Integer(integer) => Value::Integer(integer),
            HeaderValue::Float(float) => Value::Float(float),
            HeaderValue::String { text, .. } => Value::String(text),
        }
    }
}

pub struct Header {
    pub key: Option<String>,
    pub value: Option<HeaderValue>,
}

impl Header {
    pub fn keyless(value: Option<HeaderValue>) -> Self {
        Self { key: None, value }
    }
}

impl<'a> LmnoLine<'a> {
    pub fn parse_header(self) -> Result<Header, Error> {
        match self.text.chars().next().unwrap() {
            '=' | ':' => Ok(Header::keyless(parse_header_value(self.text)?)),
            '"' => {
                let (key, rest) = parse_string(self.text)?;
                let value = parse_header_value(rest.trim())?;
                Ok(Header {
                    key: Some(key),
                    value,
                })
            }
            '\'' => {
                let (key, rest) = parse_raw_string(self.text)?;
                let value = parse_header_value(rest.trim())?;
                Ok(Header {
                    key: Some(key),
                    value,
                })
            }
            _ => {
                let Some((i, _)) = self.text.char_indices().find(|(_, c)| matches!(c, '=' | ':')) else { return Err(Error::MissingValue) };
                let (key, rest) = self.text.split_at(i);
                let value = parse_header_value(rest.trim())?;

                Ok(Header {
                    key: Some(key.trim().to_owned()),
                    value,
                })
            }
        }
    }
}

fn parse_header_value(src: &str) -> Result<Option<HeaderValue>, Error> {
    match src.chars().next() {
        Some('=') => {
            let value = &src[1..].trim();
            match value.chars().next() {
                None | Some('#') => Ok(None),
                Some('"') => {
                    let (string, rest) = parse_string(value)?;
                    if !rest.starts_with('#') {
                        return Err(Error::TrailingContent);
                    }
                    Ok(Some(HeaderValue::String {
                        quoted: true,
                        text: string,
                    }))
                },
                Some('\'') => {
                    let (string, rest) = parse_raw_string(value)?;
                    if !rest.starts_with('#') {
                        return Err(Error::TrailingContent);
                    }
                    Ok(Some(HeaderValue::String {
                        quoted: true,
                        text: string,
                    }))
                },
                Some(_) => Ok(Some(parse_number_or_bool(value)?)),
            }
        }
        Some(':') => Ok(Some(HeaderValue::String {
            quoted: false,
            text: src[1..].trim().to_owned(),
        })),
        _ => Err(Error::MissingValue)
    }
}
