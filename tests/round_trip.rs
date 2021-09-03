use serde::{Deserialize, Serialize};
use std::{collections::HashMap, ffi::CString};
use xpc_connection::Message;

macro_rules! round_trip {
	($name:ident, $value:expr, $type:ty, $expected:expr) => {
		#[test]
		fn $name() {
			let initial: $type = $value;
			let encoded = xpc_serde::serialize(&initial).expect("failed to serialize");
			assert_eq!(encoded, $expected);
			let decoded = xpc_serde::deserialize::<$type>(encoded).expect("failed to deserialize");
			assert_eq!(decoded, initial);
		}
	};
}

round_trip!(round_trip_bool, true, bool, Message::Bool(true));
round_trip!(round_trip_u8, 42, u8, Message::Uint64(42));
round_trip!(round_trip_i8, 42, i8, Message::Int64(42));
round_trip!(round_trip_u16, 42, u16, Message::Uint64(42));
round_trip!(round_trip_i16, 42, i16, Message::Int64(42));
round_trip!(round_trip_u32, 42, u32, Message::Uint64(42));
round_trip!(round_trip_i32, 42, i32, Message::Int64(42));
round_trip!(round_trip_u64, 42, u64, Message::Uint64(42));
round_trip!(round_trip_i64, 42, i64, Message::Int64(42));
round_trip!(
	round_trip_string,
	"Hello World!".to_string(),
	String,
	Message::String(CString::new("Hello World!").unwrap())
);
round_trip!(
	round_trip_array,
	vec![1, 2, 3, 4, 5],
	Vec<u64>,
	Message::Array(vec![
		Message::Uint64(1),
		Message::Uint64(2),
		Message::Uint64(3),
		Message::Uint64(4),
		Message::Uint64(5)
	])
);
round_trip!(
	round_trip_dict,
	{
		let mut map = HashMap::new();
		map.insert("foo".to_string(), "bar".to_string());
		map
	},
	HashMap<String, String>,
	Message::Dictionary({
		let mut map = HashMap::new();
		map.insert(
			CString::new("foo").unwrap(),
			Message::String(CString::new("bar").unwrap()),
		);
		map
	})
);

#[derive(Debug, PartialEq, Serialize, Deserialize)]
enum TestEnum {
	Simple,
	Single(u32),
	Tuple(u32, u32),
	Struct { a: u32, b: String },
}
round_trip!(
	round_trip_simple_enum,
	TestEnum::Simple,
	TestEnum,
	Message::String(CString::new("Simple").unwrap())
);
round_trip!(
	round_trip_single_enum,
	TestEnum::Single(42),
	TestEnum,
	Message::Dictionary({
		let mut map = HashMap::new();
		map.insert(CString::new("Single").unwrap(), Message::Uint64(42));
		map
	})
);
round_trip!(round_trip_tuple_enum, TestEnum::Tuple(1, 2), TestEnum, {
	let mut map = HashMap::new();
	map.insert(
		CString::new("Tuple").unwrap(),
		Message::Array(vec![Message::Uint64(1), Message::Uint64(2)]),
	);
	Message::Dictionary(map)
});
round_trip!(
	round_trip_struct_enum,
	TestEnum::Struct {
		a: 1,
		b: "foo".to_string()
	},
	TestEnum,
	{
		let mut map = HashMap::new();
		map.insert(
			CString::new("Struct").unwrap(),
			Message::Dictionary({
				let mut map = HashMap::new();
				map.insert(CString::new("a").unwrap(), Message::Uint64(1));
				map.insert(
					CString::new("b").unwrap(),
					Message::String(CString::new("foo").unwrap()),
				);
				map
			}),
		);
		Message::Dictionary(map)
	}
);

#[derive(Debug, PartialEq, Serialize, Deserialize)]
struct TestStruct {
	a: u32,
	b: String,
	c: bool,
}
round_trip!(
	round_trip_struct,
	TestStruct {
		a: 1,
		b: "foo".to_string(),
		c: true
	},
	TestStruct,
	{
		let mut map = HashMap::new();
		map.insert(CString::new("a").unwrap(), Message::Uint64(1));
		map.insert(
			CString::new("b").unwrap(),
			Message::String(CString::new("foo").unwrap()),
		);
		map.insert(CString::new("c").unwrap(), Message::Bool(true));
		Message::Dictionary(map)
	}
);
