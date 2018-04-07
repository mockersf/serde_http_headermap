use serde;
use http::header::{HeaderMap, HeaderName, HeaderValue};

use error::{Error, Result};

struct HeaderMapWriter {
    current_key: String,
    root: HeaderMap,
}
trait WriterTrait {
    fn set_key(&mut self, key: String);
    fn is_in_object(&self) -> bool;
    fn insert_value(&mut self, value: HeaderValue);
}
impl<'a> WriterTrait for &'a mut HeaderMapWriter {
    fn set_key(&mut self, key: String) {
        self.current_key = key;
    }
    fn is_in_object(&self) -> bool {
        !self.current_key.is_empty()
    }
    fn insert_value(&mut self, value: HeaderValue) {
        self.root.insert(
            HeaderName::from_bytes(self.current_key.as_bytes()).unwrap(),
            value,
        );
    }
}

struct VecWriter {
    list: Vec<HeaderValue>,
}

impl<'a> WriterTrait for &'a mut VecWriter {
    fn set_key(&mut self, _key: String) {}
    fn is_in_object(&self) -> bool {
        true
    }
    fn insert_value(&mut self, value: HeaderValue) {
        self.list.push(value);
    }
}

struct Serializer<W> {
    writer: W,
}
impl<W> Serializer<W>
where
    W: WriterTrait,
{
    pub fn new(writer: W) -> Self {
        Serializer { writer: writer }
    }

    fn reject_non_struct_root(&mut self, write: &mut FnMut(&mut W) -> Result<()>) -> Result<()> {
        if self.writer.is_in_object() {
            write(&mut self.writer)
        } else {
            Err(Error {
                message: "base object should be a struct".to_string(),
            })
        }
    }
}
impl<'a, W> serde::Serializer for &'a mut Serializer<W>
where
    W: WriterTrait,
{
    type Ok = ();
    type Error = Error;

    type SerializeSeq = SeqWriter<'a, W>;
    type SerializeTuple = Compound<'a, W>;
    type SerializeTupleStruct = Compound<'a, W>;
    type SerializeTupleVariant = Compound<'a, W>;
    type SerializeMap = Compound<'a, W>;
    type SerializeStruct = Compound<'a, W>;
    type SerializeStructVariant = Compound<'a, W>;

    fn serialize_bool(self, value: bool) -> Result<()> {
        self.reject_non_struct_root(&mut move |writer: &mut W| {
            writer.insert_value(match value {
                true => "true".parse().unwrap(),
                false => "false".parse().unwrap(),
            });
            Ok(())
        })
    }

    fn serialize_i8(self, value: i8) -> Result<()> {
        unimplemented!()
    }

    fn serialize_i16(self, value: i16) -> Result<()> {
        unimplemented!()
    }

    fn serialize_i32(self, value: i32) -> Result<()> {
        unimplemented!()
    }

    fn serialize_i64(self, value: i64) -> Result<()> {
        unimplemented!()
    }

    fn serialize_u8(self, value: u8) -> Result<()> {
        unimplemented!()
    }

    fn serialize_u16(self, value: u16) -> Result<()> {
        unimplemented!()
    }

    fn serialize_u32(self, value: u32) -> Result<()> {
        unimplemented!()
    }

    fn serialize_u64(self, value: u64) -> Result<()> {
        unimplemented!()
    }

    fn serialize_f32(self, value: f32) -> Result<()> {
        unimplemented!()
    }

    fn serialize_f64(self, value: f64) -> Result<()> {
        unimplemented!()
    }

    fn serialize_char(self, value: char) -> Result<()> {
        unimplemented!()
    }

    fn serialize_str(self, value: &str) -> Result<()> {
        if !value.is_empty() {
            self.writer.insert_value(value.parse().unwrap());
        }
        Ok(())
    }

    fn serialize_bytes(self, _value: &[u8]) -> Result<()> {
        unimplemented!()
    }

    fn serialize_unit(self) -> Result<()> {
        self.reject_non_struct_root(&mut move |_writer: &mut W| Ok(()))
    }

    fn serialize_unit_struct(self, _name: &'static str) -> Result<()> {
        unimplemented!()
    }

    fn serialize_unit_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
    ) -> Result<()> {
        unimplemented!()
    }

    fn serialize_newtype_struct<T: ?Sized>(self, _name: &'static str, value: &T) -> Result<()>
    where
        T: serde::ser::Serialize,
    {
        try!(value.serialize(self));
        Ok(())
    }

    #[inline]
    fn serialize_newtype_variant<T: ?Sized>(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
        _value: &T,
    ) -> Result<()>
    where
        T: serde::ser::Serialize,
    {
        unimplemented!()
    }

    fn serialize_none(self) -> Result<()> {
        self.reject_non_struct_root(&mut move |writer: &mut W| Ok(()))
    }

    fn serialize_some<T: ?Sized>(self, value: &T) -> Result<()>
    where
        T: serde::ser::Serialize,
    {
        value.serialize(self)
    }

    fn serialize_seq(self, _len: Option<usize>) -> Result<Self::SerializeSeq> {
        Ok(SeqWriter::new(self))
    }

    fn serialize_tuple(self, _len: usize) -> Result<Self::SerializeTuple> {
        unimplemented!()
    }

    fn serialize_tuple_struct(
        self,
        _name: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeTupleStruct> {
        unimplemented!()
    }

    fn serialize_tuple_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeTupleVariant> {
        unimplemented!()
    }

    fn serialize_map(self, _len: Option<usize>) -> Result<Self::SerializeMap> {
        Ok(Compound::new(self))
    }

    fn serialize_struct(self, _name: &'static str, len: usize) -> Result<Self::SerializeStruct> {
        self.serialize_map(Some(len))
    }

    fn serialize_struct_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeStructVariant> {
        unimplemented!()
    }
}

struct SeqWriter<'a, W: 'a> {
    ser: &'a mut Serializer<W>,
    current: VecWriter,
}

impl<'a, W> SeqWriter<'a, W> {
    fn new(ser: &'a mut Serializer<W>) -> SeqWriter<'a, W> {
        let writer = VecWriter { list: Vec::new() };
        SeqWriter {
            ser,
            current: writer,
        }
    }
}

impl<'a, W> serde::ser::SerializeSeq for SeqWriter<'a, W>
where
    W: WriterTrait,
{
    type Ok = ();
    type Error = Error;

    fn serialize_element<T: ?Sized>(&mut self, value: &T) -> Result<()>
    where
        T: serde::ser::Serialize,
    {
        let mut ser = Serializer::new(&mut self.current);
        value.serialize(&mut ser)
    }

    fn end(self) -> Result<()> {
        unimplemented!()
    }
}

struct Compound<'a, W: 'a> {
    ser: &'a mut Serializer<W>,
    is_root: bool,
    current: HeaderMapWriter,
}

impl<'a, W> Compound<'a, W>
where
    W: WriterTrait,
{
    fn new(ser: &'a mut Serializer<W>) -> Compound<'a, W> {
        let writer = HeaderMapWriter {
            root: HeaderMap::new(),
            current_key: String::new(),
        };
        let is_root = !ser.writer.is_in_object();
        Compound {
            ser,
            is_root,
            current: writer,
        }
    }
}

impl<'a, W> serde::ser::SerializeTuple for Compound<'a, W>
where
    W: WriterTrait,
{
    type Ok = ();
    type Error = Error;

    fn serialize_element<T: ?Sized>(&mut self, _value: &T) -> Result<()>
    where
        T: serde::ser::Serialize,
    {
        unimplemented!()
    }

    #[inline]
    fn end(self) -> Result<()> {
        unimplemented!()
    }
}

impl<'a, W> serde::ser::SerializeTupleStruct for Compound<'a, W>
where
    W: WriterTrait,
{
    type Ok = ();
    type Error = Error;

    fn serialize_field<T: ?Sized>(&mut self, _value: &T) -> Result<()>
    where
        T: serde::ser::Serialize,
    {
        unimplemented!()
    }

    fn end(self) -> Result<()> {
        unimplemented!()
    }
}

impl<'a, W> serde::ser::SerializeTupleVariant for Compound<'a, W>
where
    W: WriterTrait,
{
    type Ok = ();
    type Error = Error;

    fn serialize_field<T: ?Sized>(&mut self, _value: &T) -> Result<()>
    where
        T: serde::ser::Serialize,
    {
        unimplemented!()
    }

    fn end(self) -> Result<()> {
        unimplemented!()
    }
}

impl<'a, W> serde::ser::SerializeMap for Compound<'a, W>
where
    W: WriterTrait,
{
    type Ok = ();
    type Error = Error;

    fn serialize_key<T: ?Sized>(&mut self, _key: &T) -> Result<()>
    where
        T: serde::ser::Serialize,
    {
        unimplemented!()
    }

    fn serialize_value<T: ?Sized>(&mut self, _value: &T) -> Result<()>
    where
        T: serde::ser::Serialize,
    {
        unimplemented!()
    }

    fn end(self) -> Result<()> {
        unimplemented!()
    }
}

impl<'a, W> serde::ser::SerializeStruct for Compound<'a, W>
where
    W: WriterTrait,
{
    type Ok = ();
    type Error = Error;

    fn serialize_field<T: ?Sized>(&mut self, key: &'static str, value: &T) -> Result<()>
    where
        T: serde::ser::Serialize,
    {
        if self.is_root {
            self.ser.writer.set_key(key.to_string());
            try!(value.serialize(&mut *self.ser));
            Ok(())
        } else {
            (&mut self.current).set_key(key.to_string());
            to_writer(&mut self.current, value)
        }
    }

    fn end(self) -> Result<()> {
        if !self.is_root {
            unimplemented!();
        }
        Ok(())
    }
}

impl<'a, W> serde::ser::SerializeStructVariant for Compound<'a, W>
where
    W: WriterTrait,
{
    type Ok = ();
    type Error = Error;

    fn serialize_field<T: ?Sized>(&mut self, _key: &'static str, _value: &T) -> Result<()>
    where
        T: serde::ser::Serialize,
    {
        unimplemented!()
    }

    fn end(self) -> Result<()> {
        unimplemented!()
    }
}

fn to_writer<T: ?Sized>(writer: &mut HeaderMapWriter, value: &T) -> Result<()>
where
    T: serde::ser::Serialize,
{
    let mut ser = Serializer::new(writer);
    try!(value.serialize(&mut ser));
    Ok(())
}

pub fn to_headermap<T: ?Sized>(value: &T) -> Result<HeaderMap>
where
    T: serde::ser::Serialize,
{
    let mut writer = HeaderMapWriter {
        root: HeaderMap::new(),
        current_key: String::new(),
    };
    try!(to_writer(&mut writer, value));
    Ok(writer.root)
}
