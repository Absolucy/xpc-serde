/*
	Copyright (c) 2021 Lucy <lucy@absolucy.moe>

	This Source Code Form is subject to the terms of the Mozilla Public
	License, v. 2.0. If a copy of the MPL was not distributed with this
	file, You can obtain one at http://mozilla.org/MPL/2.0/.
*/

use crate::error::DeserializeError;
use serde::de::{
	self, DeserializeSeed, Deserializer, EnumAccess, IntoDeserializer, MapAccess, SeqAccess,
	VariantAccess,
};
use std::{
	collections::{HashMap, VecDeque},
	ffi::CString,
};
use xpc_connection::Message;

pub(crate) struct XpcDeserializer {
	pub(crate) message: Message,
}

impl<'de, 'a> Deserializer<'de> for XpcDeserializer {
	type Error = DeserializeError;

	fn deserialize_any<V>(self, visitor: V) -> Result<V::Value, Self::Error>
	where
		V: de::Visitor<'de>,
	{
		match self.message {
			Message::Bool(v) => visitor.visit_bool(v),
			Message::Double(v) => visitor.visit_f64(v),
			Message::Int64(v) => visitor.visit_i64(v),
			Message::String(v) => visitor.visit_string(v.into_string().unwrap()),
			Message::Dictionary(v) => visitor.visit_map(MapAccessor {
				elements: v.into_iter().collect(),
				current_value: None,
			}),
			Message::Array(v) => visitor.visit_seq(VecAccessor { elements: v.into() }),
			Message::Data(v) => visitor.visit_byte_buf(v),
			Message::Uint64(v) => visitor.visit_u64(v),
			Message::Null => visitor.visit_unit(),
			_ => Err(DeserializeError::Unexpected(
				"any valid type",
				xpc_message_to_type(&self.message),
			)),
		}
	}

	fn deserialize_bool<V>(self, visitor: V) -> Result<V::Value, Self::Error>
	where
		V: de::Visitor<'de>,
	{
		match self.message {
			Message::Bool(v) => visitor.visit_bool(v),
			_ => Err(DeserializeError::Unexpected(
				"bool",
				xpc_message_to_type(&self.message),
			)),
		}
	}

	fn deserialize_i8<V>(self, visitor: V) -> Result<V::Value, Self::Error>
	where
		V: de::Visitor<'de>,
	{
		match self.message {
			Message::Int64(v) => visitor.visit_i8(v as i8),
			Message::Uint64(v) => visitor.visit_i8(v as i8),
			_ => Err(DeserializeError::Unexpected(
				"i64",
				xpc_message_to_type(&self.message),
			)),
		}
	}

	fn deserialize_i16<V>(self, visitor: V) -> Result<V::Value, Self::Error>
	where
		V: de::Visitor<'de>,
	{
		match self.message {
			Message::Int64(v) => visitor.visit_i16(v as i16),
			Message::Uint64(v) => visitor.visit_i16(v as i16),
			_ => Err(DeserializeError::Unexpected(
				"i64",
				xpc_message_to_type(&self.message),
			)),
		}
	}

	fn deserialize_i32<V>(self, visitor: V) -> Result<V::Value, Self::Error>
	where
		V: de::Visitor<'de>,
	{
		match self.message {
			Message::Int64(v) => visitor.visit_i32(v as i32),
			Message::Uint64(v) => visitor.visit_i32(v as i32),
			_ => Err(DeserializeError::Unexpected(
				"i64",
				xpc_message_to_type(&self.message),
			)),
		}
	}

	fn deserialize_i64<V>(self, visitor: V) -> Result<V::Value, Self::Error>
	where
		V: de::Visitor<'de>,
	{
		match self.message {
			Message::Int64(v) => visitor.visit_i64(v),
			Message::Uint64(v) => visitor.visit_i64(v as i64),
			_ => Err(DeserializeError::Unexpected(
				"i64",
				xpc_message_to_type(&self.message),
			)),
		}
	}

	fn deserialize_u8<V>(self, visitor: V) -> Result<V::Value, Self::Error>
	where
		V: de::Visitor<'de>,
	{
		match self.message {
			Message::Int64(v) => visitor.visit_u8(v as u8),
			Message::Uint64(v) => visitor.visit_u8(v as u8),
			_ => Err(DeserializeError::Unexpected(
				"u64",
				xpc_message_to_type(&self.message),
			)),
		}
	}

	fn deserialize_u16<V>(self, visitor: V) -> Result<V::Value, Self::Error>
	where
		V: de::Visitor<'de>,
	{
		match self.message {
			Message::Int64(v) => visitor.visit_u16(v as u16),
			Message::Uint64(v) => visitor.visit_u16(v as u16),
			_ => Err(DeserializeError::Unexpected(
				"u64",
				xpc_message_to_type(&self.message),
			)),
		}
	}

	fn deserialize_u32<V>(self, visitor: V) -> Result<V::Value, Self::Error>
	where
		V: de::Visitor<'de>,
	{
		match self.message {
			Message::Int64(v) => visitor.visit_u32(v as u32),
			Message::Uint64(v) => visitor.visit_u32(v as u32),
			_ => Err(DeserializeError::Unexpected(
				"u64",
				xpc_message_to_type(&self.message),
			)),
		}
	}

	fn deserialize_u64<V>(self, visitor: V) -> Result<V::Value, Self::Error>
	where
		V: de::Visitor<'de>,
	{
		match self.message {
			Message::Int64(v) => visitor.visit_u64(v as u64),
			Message::Uint64(v) => visitor.visit_u64(v),
			_ => Err(DeserializeError::Unexpected(
				"u64",
				xpc_message_to_type(&self.message),
			)),
		}
	}

	fn deserialize_f32<V>(self, visitor: V) -> Result<V::Value, Self::Error>
	where
		V: de::Visitor<'de>,
	{
		match self.message {
			Message::Double(v) => visitor.visit_f32(v as f32),
			_ => Err(DeserializeError::Unexpected(
				"f64",
				xpc_message_to_type(&self.message),
			)),
		}
	}

	fn deserialize_f64<V>(self, visitor: V) -> Result<V::Value, Self::Error>
	where
		V: de::Visitor<'de>,
	{
		match self.message {
			Message::Double(v) => visitor.visit_f64(v),
			_ => Err(DeserializeError::Unexpected(
				"f64",
				xpc_message_to_type(&self.message),
			)),
		}
	}

	fn deserialize_char<V>(self, visitor: V) -> Result<V::Value, Self::Error>
	where
		V: de::Visitor<'de>,
	{
		match self.message {
			Message::Uint64(v) => match char::from_u32(v as u32) {
				Some(c) => visitor.visit_char(c),
				None => Err(DeserializeError::Unexpected(
					"utf-8 character",
					xpc_message_to_type(&self.message),
				)),
			},
			Message::Int64(v) => match char::from_u32(v as u32) {
				Some(c) => visitor.visit_char(c),
				None => Err(DeserializeError::Unexpected(
					"utf-8 character",
					xpc_message_to_type(&self.message),
				)),
			},
			Message::String(s) if s.to_str().map(str::len).unwrap_or(0) == 1 => {
				visitor.visit_char(s.to_str()?.chars().next().unwrap())
			}
			_ => Err(DeserializeError::Unexpected(
				"u64",
				xpc_message_to_type(&self.message),
			)),
		}
	}

	fn deserialize_str<V>(self, visitor: V) -> Result<V::Value, Self::Error>
	where
		V: de::Visitor<'de>,
	{
		match self.message {
			Message::String(s) => visitor.visit_str(s.to_str()?),
			_ => Err(DeserializeError::Unexpected(
				"string",
				xpc_message_to_type(&self.message),
			)),
		}
	}

	fn deserialize_string<V>(self, visitor: V) -> Result<V::Value, Self::Error>
	where
		V: de::Visitor<'de>,
	{
		match self.message {
			Message::String(s) => visitor.visit_string(s.to_str()?.to_string()),
			_ => Err(DeserializeError::Unexpected(
				"string",
				xpc_message_to_type(&self.message),
			)),
		}
	}

	fn deserialize_bytes<V>(self, visitor: V) -> Result<V::Value, Self::Error>
	where
		V: de::Visitor<'de>,
	{
		match self.message {
			Message::Data(s) => visitor.visit_bytes(s.as_ref()),
			_ => Err(DeserializeError::Unexpected(
				"bytes",
				xpc_message_to_type(&self.message),
			)),
		}
	}

	fn deserialize_byte_buf<V>(self, visitor: V) -> Result<V::Value, Self::Error>
	where
		V: de::Visitor<'de>,
	{
		match self.message {
			Message::Data(s) => visitor.visit_byte_buf(s),
			_ => Err(DeserializeError::Unexpected(
				"bytes",
				xpc_message_to_type(&self.message),
			)),
		}
	}

	fn deserialize_option<V>(self, visitor: V) -> Result<V::Value, Self::Error>
	where
		V: de::Visitor<'de>,
	{
		match self.message {
			Message::Null => visitor.visit_none(),
			_ => visitor.visit_some(self),
		}
	}

	fn deserialize_unit<V>(self, visitor: V) -> Result<V::Value, Self::Error>
	where
		V: de::Visitor<'de>,
	{
		match self.message {
			Message::Null => visitor.visit_unit(),
			_ => Err(DeserializeError::Unexpected(
				"null",
				xpc_message_to_type(&self.message),
			)),
		}
	}

	fn deserialize_unit_struct<V>(
		self,
		_name: &'static str,
		visitor: V,
	) -> Result<V::Value, Self::Error>
	where
		V: de::Visitor<'de>,
	{
		self.deserialize_unit(visitor)
	}

	fn deserialize_newtype_struct<V>(
		self,
		_name: &'static str,
		visitor: V,
	) -> Result<V::Value, Self::Error>
	where
		V: de::Visitor<'de>,
	{
		visitor.visit_newtype_struct(self)
	}

	fn deserialize_seq<V>(self, visitor: V) -> Result<V::Value, Self::Error>
	where
		V: de::Visitor<'de>,
	{
		match self.message {
			Message::Array(array) => {
				let access = VecAccessor {
					elements: VecDeque::from(array),
				};
				visitor.visit_seq(access)
			}
			_ => Err(DeserializeError::Unexpected(
				"array",
				xpc_message_to_type(&self.message),
			)),
		}
	}

	fn deserialize_tuple<V>(self, _len: usize, visitor: V) -> Result<V::Value, Self::Error>
	where
		V: de::Visitor<'de>,
	{
		self.deserialize_seq(visitor)
	}

	fn deserialize_tuple_struct<V>(
		self,
		_name: &'static str,
		_len: usize,
		visitor: V,
	) -> Result<V::Value, Self::Error>
	where
		V: de::Visitor<'de>,
	{
		self.deserialize_seq(visitor)
	}

	fn deserialize_map<V>(self, visitor: V) -> Result<V::Value, Self::Error>
	where
		V: de::Visitor<'de>,
	{
		match self.message {
			Message::Dictionary(map) => {
				let access = MapAccessor {
					elements: map.into_iter().collect(),
					current_value: None,
				};
				visitor.visit_map(access)
			}
			_ => Err(DeserializeError::Unexpected(
				"map",
				xpc_message_to_type(&self.message),
			)),
		}
	}

	fn deserialize_struct<V>(
		self,
		_name: &'static str,
		_fields: &'static [&'static str],
		visitor: V,
	) -> Result<V::Value, Self::Error>
	where
		V: de::Visitor<'de>,
	{
		self.deserialize_map(visitor)
	}

	fn deserialize_enum<V>(
		self,
		_name: &'static str,
		_variants: &'static [&'static str],
		visitor: V,
	) -> Result<V::Value, Self::Error>
	where
		V: de::Visitor<'de>,
	{
		match self.message {
			Message::String(s) => visitor.visit_enum(s.to_str()?.to_string().into_deserializer()),
			Message::Dictionary(map) => visitor.visit_enum(EnumAccessor { map, variant: None }),
			_ => Err(DeserializeError::Unexpected(
				"enum",
				xpc_message_to_type(&self.message),
			)),
		}
	}

	fn deserialize_identifier<V>(self, visitor: V) -> Result<V::Value, Self::Error>
	where
		V: de::Visitor<'de>,
	{
		self.deserialize_str(visitor)
	}

	fn deserialize_ignored_any<V>(self, visitor: V) -> Result<V::Value, Self::Error>
	where
		V: de::Visitor<'de>,
	{
		self.deserialize_any(visitor)
	}
}

pub(crate) struct VecAccessor {
	elements: VecDeque<Message>,
}

impl<'de> SeqAccess<'de> for VecAccessor {
	type Error = DeserializeError;

	fn next_element_seed<T>(&mut self, seed: T) -> Result<Option<T::Value>, Self::Error>
	where
		T: DeserializeSeed<'de>,
	{
		if self.elements.is_empty() {
			Ok(None)
		} else {
			let message = self
				.elements
				.pop_front()
				.ok_or(DeserializeError::EndOfArray)?;
			seed.deserialize(XpcDeserializer { message })
				.map(Option::Some)
		}
	}

	fn size_hint(&self) -> Option<usize> {
		Some(self.elements.len())
	}
}

pub(crate) struct MapAccessor {
	elements: VecDeque<(CString, Message)>,
	current_value: Option<Message>,
}

impl<'de> MapAccess<'de> for MapAccessor {
	type Error = DeserializeError;

	fn next_key_seed<K>(&mut self, seed: K) -> Result<Option<K::Value>, Self::Error>
	where
		K: DeserializeSeed<'de>,
	{
		if self.elements.is_empty() {
			Ok(None)
		} else {
			let (key, value) = self
				.elements
				.pop_front()
				.ok_or(DeserializeError::EndOfArray)?;
			self.current_value = Some(value);
			seed.deserialize(XpcDeserializer {
				message: Message::String(key),
			})
			.map(Option::Some)
		}
	}

	fn next_value_seed<V>(&mut self, seed: V) -> Result<V::Value, Self::Error>
	where
		V: DeserializeSeed<'de>,
	{
		match self.current_value.take() {
			Some(message) => seed.deserialize(XpcDeserializer { message }),
			None => panic!("value called without key"),
		}
	}

	fn size_hint(&self) -> Option<usize> {
		Some(self.elements.len())
	}
}

pub(crate) struct EnumAccessor {
	map: HashMap<CString, Message>,
	variant: Option<CString>,
}

impl<'de> EnumAccess<'de> for EnumAccessor {
	type Error = DeserializeError;
	type Variant = Self;

	fn variant_seed<V>(mut self, seed: V) -> Result<(V::Value, Self::Variant), Self::Error>
	where
		V: DeserializeSeed<'de>,
	{
		let key = self
			.map
			.keys()
			.cloned()
			.next()
			.ok_or(DeserializeError::EndOfArray)?;
		self.variant = Some(key.clone());
		let key = seed.deserialize(XpcDeserializer {
			message: Message::String(key),
		})?;
		Ok((key, self))
	}
}

impl<'de> VariantAccess<'de> for EnumAccessor {
	type Error = DeserializeError;

	fn unit_variant(self) -> Result<(), Self::Error> {
		Err(DeserializeError::Unexpected("string", "map"))
	}

	fn newtype_variant_seed<T>(mut self, seed: T) -> Result<T::Value, Self::Error>
	where
		T: DeserializeSeed<'de>,
	{
		match self
			.variant
			.take()
			.and_then(|variant| self.map.remove(&variant))
		{
			Some(message) => seed.deserialize(XpcDeserializer { message }),
			None => Err(DeserializeError::EndOfArray),
		}
	}

	fn tuple_variant<V>(mut self, _len: usize, visitor: V) -> Result<V::Value, Self::Error>
	where
		V: de::Visitor<'de>,
	{
		match self
			.variant
			.take()
			.and_then(|variant| self.map.remove(&variant))
		{
			Some(message) => XpcDeserializer { message }.deserialize_seq(visitor),
			None => Err(DeserializeError::EndOfArray),
		}
	}

	fn struct_variant<V>(
		mut self,
		_fields: &'static [&'static str],
		visitor: V,
	) -> Result<V::Value, Self::Error>
	where
		V: de::Visitor<'de>,
	{
		match self
			.variant
			.take()
			.and_then(|variant| self.map.remove(&variant))
		{
			Some(message) => XpcDeserializer { message }.deserialize_map(visitor),
			None => Err(DeserializeError::EndOfArray),
		}
	}
}

fn xpc_message_to_type(message: &Message) -> &'static str {
	match message {
		Message::Bool(_) => "bool",
		Message::Double(_) => "f64",
		Message::Int64(_) => "i64",
		Message::String(_) => "string",
		Message::Dictionary(_) => "map",
		Message::Array(_) => "array",
		Message::Data(_) => "bytes",
		Message::Uint64(_) => "u64",
		Message::Null => "null",
		_ => "invalid",
	}
}
