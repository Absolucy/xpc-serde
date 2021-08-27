/*
	Copyright (c) 2021 Lucy <lucy@absolucy.moe>

	This Source Code Form is subject to the terms of the Mozilla Public
	License, v. 2.0. If a copy of the MPL was not distributed with this
	file, You can obtain one at http://mozilla.org/MPL/2.0/.
*/

use crate::error::SerializeError;
use serde::ser::{
	Serialize, SerializeMap, SerializeSeq, SerializeStruct, SerializeStructVariant, SerializeTuple,
	SerializeTupleStruct, SerializeTupleVariant, Serializer,
};
use std::{collections::HashMap, ffi::CString};
use xpc_connection::Message;

pub(crate) struct XpcSerializer;

impl<'a> Serializer for &'a mut XpcSerializer {
	type Ok = Message;
	type Error = SerializeError;

	type SerializeSeq = XpcSeqSerializer<'a>;
	type SerializeTuple = XpcSeqSerializer<'a>;
	type SerializeTupleStruct = XpcSeqSerializer<'a>;
	type SerializeTupleVariant = XpcVariantSerializer<'a>;
	type SerializeMap = XpcMapSerializer<'a>;
	type SerializeStruct = XpcMapSerializer<'a>;
	type SerializeStructVariant = XpcMapSerializer<'a>;

	fn serialize_bool(self, v: bool) -> Result<Self::Ok, Self::Error> {
		Ok(Message::Bool(v))
	}

	fn serialize_i8(self, v: i8) -> Result<Self::Ok, Self::Error> {
		Ok(Message::Int64(i64::from(v)))
	}

	fn serialize_i16(self, v: i16) -> Result<Self::Ok, Self::Error> {
		Ok(Message::Int64(i64::from(v)))
	}

	fn serialize_i32(self, v: i32) -> Result<Self::Ok, Self::Error> {
		Ok(Message::Int64(i64::from(v)))
	}

	fn serialize_i64(self, v: i64) -> Result<Self::Ok, Self::Error> {
		Ok(Message::Int64(v))
	}

	fn serialize_u8(self, v: u8) -> Result<Self::Ok, Self::Error> {
		Ok(Message::Uint64(u64::from(v)))
	}

	fn serialize_u16(self, v: u16) -> Result<Self::Ok, Self::Error> {
		Ok(Message::Uint64(u64::from(v)))
	}

	fn serialize_u32(self, v: u32) -> Result<Self::Ok, Self::Error> {
		Ok(Message::Uint64(u64::from(v)))
	}

	fn serialize_u64(self, v: u64) -> Result<Self::Ok, Self::Error> {
		Ok(Message::Uint64(v))
	}

	fn serialize_f32(self, v: f32) -> Result<Self::Ok, Self::Error> {
		Ok(Message::Double(f64::from(v)))
	}

	fn serialize_f64(self, v: f64) -> Result<Self::Ok, Self::Error> {
		Ok(Message::Double(v))
	}

	fn serialize_char(self, v: char) -> Result<Self::Ok, Self::Error> {
		Ok(Message::Uint64(u64::from(v)))
	}

	fn serialize_str(self, v: &str) -> Result<Self::Ok, Self::Error> {
		CString::new(v)
			.map(Message::String)
			.map_err(SerializeError::from)
	}

	fn serialize_bytes(self, v: &[u8]) -> Result<Self::Ok, Self::Error> {
		Ok(Message::Data(v.to_vec()))
	}

	fn serialize_none(self) -> Result<Self::Ok, Self::Error> {
		self.serialize_unit()
	}

	fn serialize_some<T: ?Sized>(self, value: &T) -> Result<Self::Ok, Self::Error>
	where
		T: serde::Serialize,
	{
		value.serialize(self)
	}

	fn serialize_unit(self) -> Result<Self::Ok, Self::Error> {
		Ok(Message::Null)
	}

	fn serialize_unit_struct(self, _name: &'static str) -> Result<Self::Ok, Self::Error> {
		self.serialize_unit()
	}

	fn serialize_unit_variant(
		self,
		_name: &'static str,
		_variant_index: u32,
		variant: &'static str,
	) -> Result<Self::Ok, Self::Error> {
		self.serialize_str(variant)
	}

	fn serialize_newtype_struct<T: ?Sized>(
		self,
		_name: &'static str,
		value: &T,
	) -> Result<Self::Ok, Self::Error>
	where
		T: serde::Serialize,
	{
		value.serialize(self)
	}

	fn serialize_newtype_variant<T: ?Sized>(
		self,
		_name: &'static str,
		_variant_index: u32,
		variant: &'static str,
		value: &T,
	) -> Result<Self::Ok, Self::Error>
	where
		T: serde::Serialize,
	{
		let mut dict = HashMap::<CString, Message>::with_capacity(2);
		dict.insert(CString::new(variant)?, value.serialize(self)?);
		Ok(Message::Dictionary(dict))
	}

	fn serialize_seq(self, len: Option<usize>) -> Result<Self::SerializeSeq, Self::Error> {
		Ok(XpcSeqSerializer {
			serializer: self,
			sequence: Vec::with_capacity(len.unwrap_or(0)),
		})
	}

	fn serialize_tuple(self, len: usize) -> Result<Self::SerializeTuple, Self::Error> {
		Ok(XpcSeqSerializer {
			serializer: self,
			sequence: Vec::with_capacity(len),
		})
	}

	fn serialize_tuple_struct(
		self,
		_name: &'static str,
		len: usize,
	) -> Result<Self::SerializeTupleStruct, Self::Error> {
		Ok(XpcSeqSerializer {
			serializer: self,
			sequence: Vec::with_capacity(len),
		})
	}

	fn serialize_tuple_variant(
		self,
		_name: &'static str,
		_variant_index: u32,
		variant: &'static str,
		len: usize,
	) -> Result<Self::SerializeTupleVariant, Self::Error> {
		Ok(XpcVariantSerializer {
			serializer: self,
			variant,
			sequence: Vec::with_capacity(len),
		})
	}

	fn serialize_map(self, len: Option<usize>) -> Result<Self::SerializeMap, Self::Error> {
		Ok(XpcMapSerializer {
			serializer: self,
			map: HashMap::with_capacity(len.unwrap_or(0)),
			key: None,
		})
	}

	fn serialize_struct(
		self,
		_name: &'static str,
		len: usize,
	) -> Result<Self::SerializeStruct, Self::Error> {
		Ok(XpcMapSerializer {
			serializer: self,
			map: HashMap::with_capacity(len),
			key: None,
		})
	}

	fn serialize_struct_variant(
		self,
		_name: &'static str,
		_variant_index: u32,
		_variant: &'static str,
		len: usize,
	) -> Result<Self::SerializeStructVariant, Self::Error> {
		Ok(XpcMapSerializer {
			serializer: self,
			map: HashMap::with_capacity(len),
			key: None,
		})
	}
}

pub(crate) struct XpcSeqSerializer<'a> {
	serializer: &'a mut XpcSerializer,
	sequence: Vec<Message>,
}

impl<'a> SerializeSeq for XpcSeqSerializer<'a> {
	type Ok = Message;
	type Error = SerializeError;

	fn serialize_element<T: ?Sized>(&mut self, value: &T) -> Result<(), Self::Error>
	where
		T: Serialize,
	{
		self.sequence.push(value.serialize(&mut *self.serializer)?);
		Ok(())
	}

	fn end(self) -> Result<Self::Ok, Self::Error> {
		Ok(Message::Array(self.sequence))
	}
}

impl<'a> SerializeTuple for XpcSeqSerializer<'a> {
	type Ok = Message;
	type Error = SerializeError;

	fn serialize_element<T: ?Sized>(&mut self, value: &T) -> Result<(), Self::Error>
	where
		T: Serialize,
	{
		self.sequence.push(value.serialize(&mut *self.serializer)?);
		Ok(())
	}

	fn end(self) -> Result<Self::Ok, Self::Error> {
		Ok(Message::Array(self.sequence))
	}
}

impl<'a> SerializeTupleStruct for XpcSeqSerializer<'a> {
	type Ok = Message;
	type Error = SerializeError;

	fn serialize_field<T: ?Sized>(&mut self, value: &T) -> Result<(), Self::Error>
	where
		T: Serialize,
	{
		self.sequence.push(value.serialize(&mut *self.serializer)?);
		Ok(())
	}

	fn end(self) -> Result<Self::Ok, Self::Error> {
		Ok(Message::Array(self.sequence))
	}
}

pub(crate) struct XpcVariantSerializer<'a> {
	serializer: &'a mut XpcSerializer,
	variant: &'static str,
	sequence: Vec<Message>,
}

impl<'a> SerializeTupleVariant for XpcVariantSerializer<'a> {
	type Ok = Message;
	type Error = SerializeError;

	fn serialize_field<T: ?Sized>(&mut self, value: &T) -> Result<(), Self::Error>
	where
		T: Serialize,
	{
		self.sequence.push(value.serialize(&mut *self.serializer)?);
		Ok(())
	}

	fn end(self) -> Result<Self::Ok, Self::Error> {
		let mut dict = HashMap::<CString, Message>::with_capacity(2);
		dict.insert(CString::new(self.variant)?, Message::Array(self.sequence));
		Ok(Message::Dictionary(dict))
	}
}

pub(crate) struct XpcMapSerializer<'a> {
	serializer: &'a mut XpcSerializer,
	map: HashMap<CString, Message>,
	key: Option<CString>,
}

impl<'a> SerializeMap for XpcMapSerializer<'a> {
	type Ok = Message;
	type Error = SerializeError;

	fn serialize_key<T: ?Sized>(&mut self, key: &T) -> Result<(), Self::Error>
	where
		T: Serialize,
	{
		match key.serialize(&mut *self.serializer)? {
			Message::String(key) => {
				self.key = Some(key);
				Ok(())
			}
			_ => Err(SerializeError::InvalidKey),
		}
	}

	fn serialize_value<T: ?Sized>(&mut self, value: &T) -> Result<(), Self::Error>
	where
		T: Serialize,
	{
		match std::mem::take(&mut self.key) {
			Some(key) => {
				self.map
					.insert(key, value.serialize(&mut *self.serializer)?);
				Ok(())
			}
			None => Err(SerializeError::MissingKey),
		}
	}

	fn end(self) -> Result<Self::Ok, Self::Error> {
		Ok(Message::Dictionary(self.map))
	}
}

impl<'a> SerializeStruct for XpcMapSerializer<'a> {
	type Ok = Message;
	type Error = SerializeError;

	fn serialize_field<T: ?Sized>(
		&mut self,
		key: &'static str,
		value: &T,
	) -> Result<(), Self::Error>
	where
		T: Serialize,
	{
		let key = CString::new(key)?;
		let value = value.serialize(&mut *self.serializer)?;
		self.map.insert(key, value);
		Ok(())
	}

	fn end(self) -> Result<Self::Ok, Self::Error> {
		Ok(Message::Dictionary(self.map))
	}
}

impl<'a> SerializeStructVariant for XpcMapSerializer<'a> {
	type Ok = Message;
	type Error = SerializeError;

	fn serialize_field<T: ?Sized>(
		&mut self,
		key: &'static str,
		value: &T,
	) -> Result<(), Self::Error>
	where
		T: Serialize,
	{
		let key = CString::new(key)?;
		let value = value.serialize(&mut *self.serializer)?;
		self.map.insert(key, value);
		Ok(())
	}

	fn end(self) -> Result<Self::Ok, Self::Error> {
		Ok(Message::Dictionary(self.map))
	}
}
