use crate::{
    Error,
    header::HeaderValue,
};

pub fn parse_number_or_bool(src: &str) -> Result<HeaderValue, Error> {
    let src = src.split_once('#').unwrap_or((src, "")).0.trim();
    Ok(match src {
        "true" => HeaderValue::Bool(true),
        "false" => HeaderValue::Bool(false),
        "infinity" => HeaderValue::Float(f64::INFINITY),
        "-infinity" => HeaderValue::Float(f64::NEG_INFINITY),
        "nan" => HeaderValue::Float(f64::NAN),
        mut src => {
            let positive = !src.starts_with('-');
            if !positive {
                src = &src[1..];
            }
            let mut radix = 10;
            if src.starts_with('0') {
                src = &src[1..];
                let Some(base) = src.chars().next() else {return Ok(HeaderValue::Integer(0))};
                radix = match base {
                    'b' | 'B' => 2,
                    'o' | 'O' => 8,
                    'x' | 'X' => 16,
                    _ => return Err(Error::InvalidNumericBase),
                };
                src = &src[1..];
            }
            
            let mut pair = (src, 1i128);
            if let Some((mantissa, exponent)) = src.split_once("+") {
                let mantissa = mantissa.trim();
                if !mantissa.ends_with('e') { return Err(Error::InvalidLiteral) }
                pair = (mantissa[..mantissa.len()-1].trim(), parse_integer(exponent, 10)?);
            } else if let Some((mantissa, exponent)) = src.split_once("-") {
                let mantissa = mantissa.trim();
                if !mantissa.ends_with('e') { return Err(Error::InvalidLiteral) }
                pair = (mantissa[..mantissa.len()-1].trim(), -parse_integer(exponent, 10)?);
            }
            let (src, exponent) = pair;

            if let Some((whole, fraction)) = src.split_once('.') {
                let whole = parse_integer(whole, radix)? as f64;
                let mut fraction = parse_integer(fraction, radix)? as f64;
                while fraction > 1. {
                    fraction /= radix as f64;
                }
                HeaderValue::Float((whole + fraction).powf(exponent as f64))
            } else {
                let whole = parse_integer(src, radix)?;
                if exponent < 0 {
                    HeaderValue::Float((whole as f64).powf(exponent as f64))
                } else {
                    HeaderValue::Integer(whole.pow(exponent as u32))
                }
            }
        }
    })
}

pub fn parse_integer(src: &str, radix: u32) -> Result<i128, Error> {
    i128::from_str_radix(&src.chars().filter(|c| !c.is_whitespace()).collect::<String>(), radix).map_err(Error::InvalidDigit)
}

pub fn parse_string(mut src: &str) -> Result<(String, &str), Error> {
    src = &src[1..];
    let mut string = String::new();
    while let Some(char) = src.chars().next() {
        src = &src[char.len_utf8()..];
        match char {
            '"' => break,
            '\\' => {
                let Some(next) = src.chars().next() else { return Err(Error::InvalidEscape) };
                match next {
                    '"' => string.push('"'),
                    '\\' => string.push('\\'),
                    '0' => string.push('\0'),
                    'n' => string.push('\n'),
                    'r' => string.push('\r'),
                    't' => string.push('\t'),
                    _ => return Err(Error::InvalidEscape),
                }
            }
            _ => string.push(char),
        }
    }

    Ok((string, src.trim()))
}

pub fn parse_raw_string(mut src: &str) -> Result<(String, &str), Error> {
    src = &src[1..];
    let mut string = String::new();
    while let Some(char) = src.chars().next() {
        src = &src[char.len_utf8()..];
        match char {
            '\'' => break,
            _ => string.push(char),
        }
    }

    Ok((string, src.trim()))
}
