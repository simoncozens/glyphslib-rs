use serde::{ser, Serialize};
use smol_str::{SmolStr, SmolStrBuilder};

use crate::{
    error::{Error, Result},
    is_alnum_strict, is_numeric,
};

pub struct Serializer {
    output: Vec<SmolStr>,
    // Stack to buffer sequence elements and decide inline vs block formatting
    seq_stack: Vec<SeqState>,
    // Track nesting of maps to append trailing semicolon for top-level map
    map_depth: usize,
}

struct SeqState {
    elements: Vec<Vec<SmolStr>>, // serialized tokens per element
    all_simple: bool,
    all_numeric: bool, // true if all elements are numeric (affects comma spacing)
}

macro_rules! forward_to {
    ($method_from: ident, $t: ty, $method_to:ident, $conversion:expr) => {
        fn $method_from(self, v: $t) -> Result<()> {
            self.$method_to($conversion(v))
        }
    };
}

const FLOAT_PRECISION: i32 = 4;

pub fn to_string<T>(value: &T) -> Result<String>
where
    T: Serialize,
{
    let mut serializer = Serializer {
        output: Vec::new(),
        seq_stack: Vec::new(),
        map_depth: 0,
    };
    value.serialize(&mut serializer)?;
    Ok(serializer.output.join(""))
}

impl ser::Serializer for &mut Serializer {
    type Ok = ();
    type Error = Error;
    type SerializeSeq = Self;
    type SerializeTuple = Self;
    type SerializeTupleStruct = Self;
    type SerializeTupleVariant = Self;
    type SerializeMap = Self;
    type SerializeStruct = Self;
    type SerializeStructVariant = Self;

    fn serialize_bool(self, v: bool) -> Result<()> {
        self.output
            .push(SmolStr::new_static(if v { "1" } else { "0" }));
        Ok(())
    }
    forward_to!(serialize_i8, i8, serialize_i64, i64::from);
    forward_to!(serialize_i16, i16, serialize_i64, i64::from);
    forward_to!(serialize_i32, i32, serialize_i64, i64::from);
    forward_to!(serialize_u8, u8, serialize_u64, u64::from);
    forward_to!(serialize_u16, u16, serialize_u64, u64::from);
    forward_to!(serialize_u32, u32, serialize_u64, u64::from);

    fn serialize_i64(self, v: i64) -> Result<()> {
        self.output.push(SmolStr::new(format!("{v}")));
        Ok(())
    }

    fn serialize_u64(self, v: u64) -> Result<()> {
        self.output.push(SmolStr::new(format!("{v}")));
        Ok(())
    }

    forward_to!(serialize_f32, f32, serialize_f64, f64::from);

    fn serialize_f64(self, v: f64) -> Result<()> {
        self.output.push(SmolStr::new(format!(
            "{}",
            (v * 10_f64.powi(FLOAT_PRECISION)).round() / 10_f64.powi(FLOAT_PRECISION)
        )));
        Ok(())
    }

    fn serialize_char(self, v: char) -> Result<()> {
        self.output.push(SmolStr::new_inline(&format!("{v}")));
        Ok(())
    }

    fn serialize_str(self, v: &str) -> Result<()> {
        escape_string(&mut self.output, v);
        Ok(())
    }

    fn serialize_bytes(self, data: &[u8]) -> Result<()> {
        let mut builder = SmolStrBuilder::new();
        builder.push('<');
        for byte in data {
            let [one, two] = hex_digits_for_byte(*byte);
            builder.push(one);
            builder.push(two);
        }
        builder.push('>');
        self.output.push(builder.finish());
        Ok(())
    }

    fn serialize_none(self) -> Result<()> {
        self.serialize_unit()
    }

    fn serialize_some<T>(self, value: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        value.serialize(self)
    }

    fn serialize_unit(self) -> Result<()> {
        // ????
        self.output.push(SmolStr::new_static("null"));
        Ok(())
    }

    fn serialize_unit_struct(self, _name: &'static str) -> Result<()> {
        self.serialize_unit()
    }

    fn serialize_unit_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        variant: &'static str,
    ) -> Result<()> {
        self.serialize_str(variant)
    }

    fn serialize_newtype_struct<T>(self, _name: &'static str, value: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        value.serialize(self)
    }

    fn serialize_newtype_variant<T>(
        self,
        _name: &'static str,
        _variant_index: u32,
        variant: &'static str,
        value: &T,
    ) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        self.output.push(SmolStr::new_static("{newtype"));
        variant.serialize(&mut *self)?;
        self.output.push(SmolStr::new_static(" = "));
        value.serialize(&mut *self)?;
        self.output.push(SmolStr::new_static(";}"));
        Ok(())
    }

    fn serialize_seq(self, _len: Option<usize>) -> Result<Self::SerializeSeq> {
        self.seq_stack.push(SeqState {
            elements: Vec::new(),
            all_simple: true,
            all_numeric: true,
        });
        Ok(self)
    }

    fn serialize_tuple(self, _len: usize) -> Result<Self::SerializeTuple> {
        self.output.push(SmolStr::new_static("("));
        Ok(self)
    }

    // Tuple structs look just like sequences in JSON.
    fn serialize_tuple_struct(
        self,
        _name: &'static str,
        len: usize,
    ) -> Result<Self::SerializeTupleStruct> {
        self.serialize_seq(Some(len))
    }

    fn serialize_tuple_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        variant: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeTupleVariant> {
        self.output.push(SmolStr::new_static("{tuplevariant"));
        variant.serialize(&mut *self)?;
        self.output.push(SmolStr::new_static(" = ("));
        Ok(self)
    }

    fn serialize_map(self, _len: Option<usize>) -> Result<Self::SerializeMap> {
        self.map_depth += 1;
        self.output.push(SmolStr::new_static("{"));
        Ok(self)
    }

    fn serialize_struct(self, _name: &'static str, len: usize) -> Result<Self::SerializeStruct> {
        self.serialize_map(Some(len))
    }

    fn serialize_struct_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        variant: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeStructVariant> {
        self.output.push(SmolStr::new_static("{structvariant"));
        variant.serialize(&mut *self)?;
        self.output.push(SmolStr::new_static(" = }"));
        Ok(self)
    }
}

impl ser::SerializeSeq for &mut Serializer {
    // Must match the `Ok` type of the serializer.
    type Ok = ();
    // Must match the `Error` type of the serializer.
    type Error = Error;

    // Serialize a single element of the sequence.
    fn serialize_element<T>(&mut self, value: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        // Serialize element into a temporary serializer to inspect complexity
        let mut tmp = Serializer {
            output: Vec::new(),
            seq_stack: Vec::new(),
            map_depth: 0,
        };
        value.serialize(&mut tmp)?;
        // Determine if element is complex (starts with map or sequence)
        let complex = match tmp.output.first() {
            Some(first) => {
                let s = first.as_str();
                s.starts_with("(\n") || s == "(" || s == "{"
            }
            None => false,
        };
        // Check if all output tokens are numeric (for comma spacing decision)
        let all_numeric = tmp.output.iter().all(|tok| {
            let s = tok.as_str();
            s.parse::<f64>().is_ok() || s == "-" || s == "."
        });
        if let Some(state) = self.seq_stack.last_mut() {
            if complex {
                state.all_simple = false;
            }
            if !all_numeric {
                state.all_numeric = false;
            }
            state.elements.push(tmp.output);
        }
        Ok(())
    }

    // Close the sequence.
    fn end(self) -> Result<()> {
        if let Some(state) = self.seq_stack.pop() {
            if state.all_simple {
                // Inline formatting: (a,b,c) or (a, b, c) depending on type
                self.output.push(SmolStr::new_static("("));
                for (i, elem) in state.elements.iter().enumerate() {
                    if i > 0 {
                        // Numbers: no space. Strings/other: space after comma
                        if state.all_numeric {
                            self.output.push(SmolStr::new_static(","));
                        } else {
                            self.output.push(SmolStr::new_static(", "));
                        }
                    }
                    // Append element tokens verbatim
                    for tok in elem {
                        self.output.push(tok.clone());
                    }
                }
                self.output.push(SmolStr::new_static(")"));
            } else {
                // Block formatting with newlines
                self.output.push(SmolStr::new_static("(\n"));
                for (i, elem) in state.elements.iter().enumerate() {
                    if i > 0 {
                        self.output.push(SmolStr::new_static(",\n"));
                    }
                    for tok in elem {
                        self.output.push(tok.clone());
                    }
                }
                self.output.push(SmolStr::new_static("\n)"));
            }
            // Semicolons are never added after arrays - they're only added by
            // serialize_value for dictionary values
            Ok(())
        } else {
            // Should not happen
            Ok(())
        }
    }
}

// Same thing but for tuples.
// Tuple (pos, node, etc.) have no space between elements and no newlines.
impl ser::SerializeTuple for &mut Serializer {
    type Ok = ();
    type Error = Error;

    fn serialize_element<T>(&mut self, value: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        if !self.output.ends_with(&[SmolStr::new_static("(")]) {
            self.output.push(SmolStr::new_static(","));
        }
        value.serialize(&mut **self)
    }

    fn end(self) -> Result<()> {
        self.output.push(SmolStr::new_static(")"));
        Ok(())
    }
}

// Same thing but for tuple structs.
impl ser::SerializeTupleStruct for &mut Serializer {
    type Ok = ();
    type Error = Error;

    fn serialize_field<T>(&mut self, value: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        if !self.output.ends_with(&[SmolStr::new_static("(")]) {
            self.output.push(SmolStr::new_static(", "));
        }
        value.serialize(&mut **self)
    }

    fn end(self) -> Result<()> {
        self.output.push(SmolStr::new_static(")"));
        Ok(())
    }
}

impl ser::SerializeTupleVariant for &mut Serializer {
    type Ok = ();
    type Error = Error;

    fn serialize_field<T>(&mut self, value: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        if !self.output.ends_with(&[SmolStr::new_static("(")]) {
            self.output.push(SmolStr::new_static(", "));
        }
        value.serialize(&mut **self)
    }

    fn end(self) -> Result<()> {
        self.output.push(SmolStr::new_static(");}"));
        Ok(())
    }
}

impl ser::SerializeMap for &mut Serializer {
    type Ok = ();
    type Error = Error;

    fn serialize_key<T>(&mut self, key: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        // if !self.output.ends_with('{') {
        //     self.output.push(SmolStr::new_static(";"));
        // }
        self.output.push(SmolStr::new_static("\n"));
        key.serialize(&mut **self)
    }

    fn serialize_value<T>(&mut self, value: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        self.output.push(SmolStr::new_static(" = "));
        value.serialize(&mut **self)?;
        self.output.push(SmolStr::new_static(";"));
        Ok(())
    }

    fn end(self) -> Result<()> {
        self.output.push(SmolStr::new_static("\n}"));
        // Never add semicolon after closing brace - semicolons are only added
        // by serialize_value for dictionary values
        self.map_depth = self.map_depth.saturating_sub(1);
        Ok(())
    }
}

impl ser::SerializeStruct for &mut Serializer {
    type Ok = ();
    type Error = Error;

    fn serialize_field<T>(&mut self, key: &'static str, value: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        self.output.push(SmolStr::new_static("\n"));
        key.serialize(&mut **self)?;
        self.output.push(SmolStr::new_static(" = "));
        value.serialize(&mut **self)?;
        self.output.push(SmolStr::new_static(";"));
        Ok(())
    }

    fn end(self) -> Result<()> {
        if self.output.last() == Some(&SmolStr::new_static(";")) {
            self.output.pop();
            self.output.push(SmolStr::new_static(";"));
        }
        self.output.push(SmolStr::new_static("\n}"));
        Ok(())
    }
}

impl ser::SerializeStructVariant for &mut Serializer {
    type Ok = ();
    type Error = Error;

    fn serialize_field<T>(&mut self, key: &'static str, value: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        if !self.output.ends_with(&[SmolStr::new_static("{")]) {
            self.output.push(SmolStr::new_static("; "));
        }
        key.serialize(&mut **self)?;
        self.output.push(SmolStr::new_static(" = "));
        value.serialize(&mut **self)
    }

    fn end(self) -> Result<()> {
        self.output.push(SmolStr::new_static("};}"));
        Ok(())
    }
}

fn escape_string(buf: &mut Vec<SmolStr>, s: &str) {
    if !s.is_empty()
        && (s.as_bytes().iter().all(|&b| is_alnum_strict(b))
            && !s.as_bytes().iter().all(|&b| is_numeric(b)))
    {
        buf.push(SmolStr::new(s));
    } else {
        buf.push(SmolStr::new_static("\""));
        let mut start = 0;
        let mut ix = start;
        while ix < s.len() {
            let b = s.as_bytes()[ix];
            match b {
                b'"' | b'\\' => {
                    buf.push(SmolStr::new(&s[start..ix]));
                    buf.push(SmolStr::new_static("\\"));
                    start = ix;
                }
                _ => (),
            }
            ix += 1;
        }
        buf.push(SmolStr::new(&s[start..]));
        buf.push(SmolStr::new_static("\""));
    }
}

#[inline]
fn hex_digits_for_byte(byte: u8) -> [char; 2] {
    fn to_hex_digit(val: u8) -> char {
        match val {
            0..=9 => ('0' as u32 as u8 + val).into(),
            10..=15 => (('a' as u32 as u8) + val - 10).into(),
            _ => unreachable!("only called with values in range 0..=15"),
        }
    }

    [to_hex_digit(byte >> 4), to_hex_digit(byte & 0x0f)]
}

#[cfg(test)]
mod tests {
    use crate::Plist;

    use super::*;

    #[test]
    fn hex_to_ascii() {
        assert_eq!(hex_digits_for_byte(0x01), ['0', '1']);
        assert_eq!(hex_digits_for_byte(0x00), ['0', '0']);
        assert_eq!(hex_digits_for_byte(0xff), ['f', 'f']);
        assert_eq!(hex_digits_for_byte(0xf0), ['f', '0']);
        assert_eq!(hex_digits_for_byte(0x0f), ['0', 'f']);
    }

    #[test]
    fn test_serialize() {
        let plist: Plist = vec![
            Plist::String("hello".to_string()),
            Plist::String("world".to_string()),
        ]
        .into();
        let s = to_string(&plist).unwrap();
        assert_eq!(s, r#"(hello, world)"#);
    }

    #[test]
    fn test_serialize_map() {
        let plist_str = "{\nfoo = bar;\nhello = world;\ntuple = (1,2);\n}";
        let plist: Plist = Plist::parse(plist_str).unwrap();
        let s = to_string(&plist).unwrap();
        assert_eq!(s, plist_str);
    }

    #[test]
    fn test_serialize_struct() {
        // Based on real Glyphs file format: dictionaries in arrays have no semicolon,
        // top-level dict has no semicolon or trailing newline
        let plist_str = r#"
{
axes = (
{
hidden = 1;
name = Weight;
tag = wght;
}
);
}"#
        .trim(); // Remove leading newline
        let plist: Plist = Plist::parse(plist_str).unwrap();
        let s = to_string(&plist).unwrap();
        assert_eq!(s, plist_str);
    }

    #[test]
    fn test_vec_axis() {
        #[derive(Serialize, Debug, Default, Clone)]
        struct Axis {
            /// If the axis should be visible in the UI.
            #[serde(default)]
            pub hidden: bool,
            /// The name of the axis (e.g. `Weight``)
            pub name: String,
            /// The axis tag (e.g. `wght`)
            pub tag: String,
        }
        let foo = vec![Axis {
            hidden: true,
            name: "Weight".to_string(),
            tag: "wght".to_string(),
        }];
        let s = to_string(&foo).unwrap();
        // Block format with dict inside array: (\n{...}\n)
        // When formatted on one line for comparison, this becomes: ({ ... })
        let expected = "(\n{\nhidden = 1;\nname = Weight;\ntag = wght;\n}\n)";
        assert_eq!(s, expected);
    }

    #[test]
    fn test_string_escaping() {
        let str = "files/LinkedFontv3.glyphs";
        let mut buf = Vec::new();
        escape_string(&mut buf, str);
        let output = buf.join("");
        assert_eq!(output, "\"files/LinkedFontv3.glyphs\"");
    }
}
