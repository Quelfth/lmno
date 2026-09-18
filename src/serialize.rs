use std::collections::HashMap;

use crate::Value;

const INDENT: u32 = 4;

impl Value {
    pub fn to_lmno(&self) -> String {
        let mut cx = SerializeContext::default();

        match self {
            Value::Empty => return "=".to_owned(),
            Value::Bool(value) => return format!("= {value}"),
            Value::Integer(value) => return format!("= {value}"),
            Value::Float(value) => return format!("= {value}"),
            Value::String(value) => serialize_string(&mut cx, 0, value),
            Value::List(list) => serialize_list(&mut cx, list),
            Value::Map(map) => serialize_map(&mut cx, map),
        }

        cx.string
    }
}


#[derive(Default)]
struct SerializeContext {
    string: String,
    indent: u32,
}

impl SerializeContext {
    pub fn write_indent(&mut self) {
        self.align(self.indent * INDENT)
    }

    pub fn write(&mut self, string: &str) {
        self.string += string;
    }

    pub fn align(&mut self, amount: u32) {
        for _ in 0..amount {
            self.string += " ";
        }
    }

    pub fn newline(&mut self) {
        self.string += "\n";
    }
}

fn serialize_string(cx: &mut SerializeContext, inset: u32, string: &str) {
    let mut lines = string.lines();
    let Some(first_line) = lines.next() else {
        cx.write(":");
        return
    };
    cx.write(&format!(": {first_line}\n"));
    for line in lines {
        cx.write_indent();
        cx.align(inset + 2);
        cx.write(line);
        cx.newline();
    }
}

fn serialize_list(cx: &mut SerializeContext, values: &[Value]) {
    serialize_kv_list(cx, 0, values.iter().map(|value| ("", value)))
}

fn serialize_map(cx: &mut SerializeContext, map: &HashMap<String, Value>) {
    let inset = map.keys().map(|k| k.len()).max().unwrap_or_default() as u32;
    serialize_kv_list(cx, inset + 1, map.iter().map(|(k, v)| (&**k, v)));
}

fn serialize_kv_list<'a>(cx: &mut SerializeContext, inset: u32, list: impl IntoIterator<Item = (&'a str, &'a Value)>) {
    for (k, v) in list {
        cx.write_indent();
        cx.write(k);
        let k_columns = k.chars().count() as u32;
        cx.align(inset - k_columns);
        match v {
            Value::Empty => cx.write("=\n"),
            Value::Bool(bool) => cx.write(&format!("= {bool}\n")),
            Value::Integer(number) => cx.write(&format!("= {number}\n")),
            Value::Float(number) => cx.write(&format!("= {number}\n")),
            Value::String(string) => {
                serialize_string(cx, inset, string);
            },
            Value::List(values) => {
                cx.write("=\n");
                cx.indent += 1;
                serialize_list(cx, values);
                cx.indent -= 1;
            },
            Value::Map(map) => {
                cx.write("=\n");
                cx.indent += 1;
                serialize_map(cx, map);
                cx.indent -= 1;
            },
        }
    }
}
