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
