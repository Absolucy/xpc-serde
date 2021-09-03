/*
	Copyright (c) 2021 Lucy <lucy@absolucy.moe>

	This Source Code Form is subject to the terms of the Mozilla Public
	License, v. 2.0. If a copy of the MPL was not distributed with this
	file, You can obtain one at http://mozilla.org/MPL/2.0/.
*/

use xpc_connection::Message;

mod de;
pub mod error;
mod ser;

pub fn serialize<T>(v: &T) -> Result<Message, error::SerializeError>
where
	T: serde::Serialize,
{
	v.serialize(&mut ser::XpcSerializer)
}

pub fn deserialize<'de, T>(message: Message) -> Result<T, error::DeserializeError>
where
	T: serde::Deserialize<'de>,
{
	T::deserialize(de::XpcDeserializer { message })
}

pub(crate) fn xpc_message_to_type(message: &Message) -> &'static str {
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
