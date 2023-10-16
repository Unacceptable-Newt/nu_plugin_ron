use std::vec;

use nu_plugin::LabeledError;
use nu_protocol::{Span, Spanned, Value, Record};
use ron;
use serde::{Deserializer, de::Visitor};

pub fn ron_do_something(
    param: Option<Spanned<String>>,
    val: &str,
    value_span: Span,
) -> Result<Value, LabeledError> {
    let a_val = match param {
        Some(p) => format!("Hello, {}! with value: {}", p.item, val),
        None => format!("Hello, Default! with value: {}", val),
    };
    Ok(Value::Record {
        val: Record{cols: vec!["wow".to_string()],
                    vals: vec![Value::Record { val: Record {cols: vec!["this is another label".to_string(),"time for the supprize".to_string()],
                                       vals: vec![Value::String { val: "()".to_string(), internal_span: value_span },
                                                  Value::String { val: a_val, internal_span: value_span }],
                    },internal_span: value_span}],
    },
        internal_span: value_span,
    })
}

struct RonRecord{
    rec: Record,
    span: Span, 
}

impl RonRecord {
    fn new(span: Span) -> RonRecord{
        RonRecord { rec: Record::new(), span }
    }
}

impl<'de> Visitor<'de> for RonRecord {
    type Value = Record;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(formatter, "A record representing a RON string")
    }

    fn visit_bool<E>(mut self, v: bool) -> Result<Self::Value, E>
        where
            E: serde::de::Error, {
        self.rec.push("Boolean", Value::Bool { val: v, internal_span: self.span });
        Ok(self.rec)
    }

    fn visit_none<E>(mut self) -> Result<Self::Value, E>
        where
            E: serde::de::Error, {
        self.rec.push("Option", Value::Nothing { internal_span: self.span });
        Ok(self.rec)
    }

    fn visit_unit<E>(mut self) -> Result<Self::Value, E>
        where
            E: serde::de::Error, {
        self.rec.push("Unit", Value::String { val: "()".into(), internal_span: self.span });
        Ok(self.rec)
    }

    fn visit_f32<E>(mut self, v: f32) -> Result<Self::Value, E>
        where
            E: serde::de::Error, {
        self.rec.push("f32", Value::float(v.into(), self.span));
        Ok(self.rec)
    }

    fn visit_f64<E>(mut self, v: f64) -> Result<Self::Value, E>
        where
            E: serde::de::Error, {
        self.rec.push("f32", Value::float(v, self.span));
        Ok(self.rec) 
    }

    fn visit_i8<E>(mut self, v: i8) -> Result<Self::Value, E>
        where
            E: serde::de::Error, {
        self.rec.push("i8", Value::int(v.into(), self.span));
        Ok(self.rec) 
    }

    fn visit_i16<E>(mut self, v: i16) -> Result<Self::Value, E>
        where
            E: serde::de::Error, {
        self.rec.push("i16", Value::int(v.into(), self.span));
        Ok(self.rec) 
    }

    fn visit_i32<E>(mut self, v: i32) -> Result<Self::Value, E>
        where
            E: serde::de::Error, {
        self.rec.push("i32", Value::int(v.into(), self.span));
        Ok(self.rec) 
    }

    fn visit_i64<E>(mut self, v: i64) -> Result<Self::Value, E>
        where
            E: serde::de::Error, {
        self.rec.push("i64", Value::int(v, self.span));
        Ok(self.rec) 
    }

    fn visit_i128<E>(mut self, v: i128) -> Result<Self::Value, E>
        where
            E: serde::de::Error, {
        self.rec.push("i128", Value::int(v as i64, self.span));
        Ok(self.rec) 
    }

    fn visit_u8<E>(mut self, v: u8) -> Result<Self::Value, E>
        where
            E: serde::de::Error, {
                self.rec.push("u8", Value::int(v.into(), self.span));
                Ok(self.rec)
    }
    fn visit_u16<E>(mut self, v: u16) -> Result<Self::Value, E>
        where
            E: serde::de::Error, {
                self.rec.push("u16", Value::int(v.into(), self.span));
                Ok(self.rec)
    }
    fn visit_u32<E>(mut self, v: u32) -> Result<Self::Value, E>
        where
            E: serde::de::Error, {
                self.rec.push("u32", Value::int(v.into(), self.span));
                Ok(self.rec)
    }
    fn visit_u128<E>(mut self, v: u128) -> Result<Self::Value, E>
        where
            E: serde::de::Error, {
                self.rec.push("u128", Value::int(v as i64, self.span));
                Ok(self.rec)
    }

    fn visit_char<E>(mut self, v: char) -> Result<Self::Value, E>
        where
            E: serde::de::Error, {
                self.rec.push("char", Value::string(v, self.span));
                Ok(self.rec)
    }

    fn visit_string<E>(mut self, v: String) -> Result<Self::Value, E>
        where
            E: serde::de::Error, {
                self.rec.push("String", Value::string(v, self.span));
                Ok(self.rec)
    }

    fn visit_borrowed_str<E>(mut self, v: &'de str) -> Result<Self::Value, E>
        where
            E: serde::de::Error, {
                self.rec.push("Borrowed String", Value::string(v, self.span));
                Ok(self.rec)
    }

    fn visit_byte_buf<E>(mut self, v: Vec<u8>) -> Result<Self::Value, E>
        where
            E: serde::de::Error, {
                self.rec.push("Bytes", Value::binary(v, self.span));
                Ok(self.rec)
    }

    fn visit_borrowed_bytes<E>(mut self, v: &'de [u8]) -> Result<Self::Value, E>
        where
            E: serde::de::Error, {
                self.rec.push("Bytes", Value::binary(v, self.span));
                Ok(self.rec)
    }

    //fn visit_some<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    //    where
    //        D: Deserializer<'de>, {
    //    deserializer.
    //}
}

pub fn from_ron(
    _param: Option<Spanned<String>>,
    val: &str,
    value_span: Span,
) -> Result<Value, LabeledError> {
    let result = ron::Deserializer::from_str(val);
    match result {
        Ok(mut v) => {
            let visit = RonRecord::new(value_span);
            let deserialised: Result<nu_protocol::Record, ron::Error> = v.deserialize_any(visit);
            Ok(Value::Record { val: deserialised.unwrap(), internal_span: value_span })
        }
        _ => return Err(LabeledError { 
            label: "failed to create deserializer".into(), 
            msg: "failed to deseariles RON string".into(), 
            span: Some(value_span) }),

    }
}