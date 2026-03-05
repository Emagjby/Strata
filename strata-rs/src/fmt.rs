use std::collections::BTreeMap;

use crate::value::Value;

#[derive(Clone, Copy)]
pub enum FormatOptions {
    PRETTY,
    AST,
}

pub fn fmt(opts: FormatOptions, value: &Value) -> String {
    match opts {
        FormatOptions::PRETTY => {
            let mut f = PrettyFmt {
                out: String::new(),
                depth: 0,
                indent: 2,
            };
            f.value(value);
            f.out
        }
        FormatOptions::AST => format!("{:#?}", value),
    }
}

struct PrettyFmt {
    out: String,
    depth: usize,
    indent: usize,
}

impl PrettyFmt {
    fn pad(&self) -> String {
        " ".repeat(self.depth * self.indent)
    }

    fn value(&mut self, value: &Value) {
        match value {
            Value::Null => self.out.push_str("null"),
            Value::Bool(b) => self.out.push_str(&b.to_string()),
            Value::Int(i) => self.out.push_str(&i.to_string()),
            Value::String(s) => {
                self.out.push('"');
                self.out.push_str(s);
                self.out.push('"')
            }
            Value::Bytes(b) => self.bytes(b),
            Value::List(l) => self.list(l),
            Value::Map(m) => self.map(m),
        }
    }

    fn bytes(&mut self, bytes: &[u8]) {
        self.out.push('[');
        for byte in bytes {
            self.out.push_str(&format!("{:02x}", byte));
        }
        self.out.push(']');
    }

    fn list(&mut self, list: &[Value]) {
        self.out.push('[');

        for (i, v) in list.iter().enumerate() {
            self.value(v);
            if i + 1 != list.len() {
                self.out.push_str(", ");
            }
        }

        self.out.push(']');
    }

    fn map(&mut self, map: &BTreeMap<String, Value>) {
        self.out.push_str("{\n");
        self.depth += 1;

        for (k, v) in map.iter() {
            self.out.push_str(&self.pad());
            self.out.push_str(k);
            self.out.push_str(": ");
            self.value(v);
            self.out.push('\n');
        }

        self.depth -= 1;
        self.out.push_str(&self.pad());
        self.out.push('}');
    }
}
