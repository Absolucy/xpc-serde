/*
	Copyright (c) 2021 Lucy <lucy@absolucy.moe>

	This Source Code Form is subject to the terms of the Mozilla Public
	License, v. 2.0. If a copy of the MPL was not distributed with this
	file, You can obtain one at http://mozilla.org/MPL/2.0/.
*/

use serde::{de, ser};

#[derive(Debug, Clone, thiserror::Error)]
pub enum SerializeError {
	#[error("failed to serialize string: {0}")]
	NulString(#[from] std::ffi::NulError),
	#[error("key must be a string")]
	InvalidKey,
	#[error("attempted to serialize value without key")]
	MissingKey,
	#[error("expected {0}")]
	Expected(&'static str),
	#[error("{0}")]
	Custom(String),
}

impl ser::Error for SerializeError {
	fn custom<T>(msg: T) -> Self
	where
		T: std::fmt::Display,
	{
		Self::Custom(msg.to_string())
	}
}

#[derive(Debug, Clone, thiserror::Error)]
pub enum DeserializeError {
	#[error("failed to deserialize string: {0}")]
	InvalidString(#[from] std::str::Utf8Error),
	#[error("expected {0}, got {1}")]
	Unexpected(&'static str, &'static str),
	#[error("attempted to fetch element from end of array")]
	EndOfArray,
	#[error("{0}")]
	Custom(String),
}

impl de::Error for DeserializeError {
	fn custom<T>(msg: T) -> Self
	where
		T: std::fmt::Display,
	{
		Self::Custom(msg.to_string())
	}
}
