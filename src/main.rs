/*
	Copyright (c) 2021 Lucy <lucy@absolucy.moe>

	This Source Code Form is subject to the terms of the Mozilla Public
	License, v. 2.0. If a copy of the MPL was not distributed with this
	file, You can obtain one at http://mozilla.org/MPL/2.0/.
*/

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
enum Animal {
	Dog,
	Frog(String, Vec<isize>),
	Cat { age: usize, name: String },
	AntHive(Vec<String>),
}

fn main() {
	let map = {
		let mut map = HashMap::<&'static str, u64>::new();
		map.insert("a", 1);
		map.insert("b", 2);
		map.insert("c", 3);
		map
	};
	let a = xpc_serde::serialize(&Animal::Dog).unwrap();
	let b = xpc_serde::serialize(&Animal::Frog("Frogger".to_string(), vec![1, 2, 3])).unwrap();
	let c = xpc_serde::serialize(&Animal::Cat {
		age: 3,
		name: "Cookie".to_string(),
	})
	.unwrap();
	let d = xpc_serde::serialize(&Animal::AntHive(vec![
		"foo".to_string(),
		"bar".to_string(),
		"baz".to_string(),
	]))
	.unwrap();
	let e = xpc_serde::serialize(&map).unwrap();
	assert_eq!(map, xpc_serde::deserialize(e).unwrap());
	assert_eq!(Animal::Dog, xpc_serde::deserialize(a).unwrap());
	assert_eq!(
		Animal::Frog("Frogger".to_string(), vec![1, 2, 3]),
		xpc_serde::deserialize(b).unwrap()
	);
	assert_eq!(
		Animal::Cat {
			age: 3,
			name: "Cookie".to_string(),
		},
		xpc_serde::deserialize(c).unwrap()
	);
	assert_eq!(
		Animal::AntHive(vec![
			"foo".to_string(),
			"bar".to_string(),
			"baz".to_string(),
		]),
		xpc_serde::deserialize(d).unwrap()
	);
}
