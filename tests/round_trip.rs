use bytes::Bytes;
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, ffi::CString};
use xpc_connection::{message_to_xpc_object, xpc_object_to_message, Message};

macro_rules! round_trip {
	($name:ident, $value:expr, $type:ty, $expected:expr) => {
		#[test]
		fn $name() {
			let initial: $type = $value;
			let encoded = xpc_serde::serialize(&initial).expect("failed to serialize");
			assert_eq!(encoded, $expected);
			let reencoded = xpc_object_to_message(message_to_xpc_object(encoded));
			let decoded =
				xpc_serde::deserialize::<$type>(reencoded).expect("failed to deserialize");
			assert_eq!(decoded, initial);
		}
	};
	[$name:ident, $value:expr, $type:ty] => {
		#[test]
		fn $name() {
			let initial: $type = $value;
			let encoded = xpc_serde::serialize(&initial).expect("failed to serialize");
			let reencoded = xpc_object_to_message(message_to_xpc_object(encoded));
			let decoded =
				xpc_serde::deserialize::<$type>(reencoded).expect("failed to deserialize");
			assert!((decoded - initial).abs() < <$type>::EPSILON);
		}
	};
}

macro_rules! cstr {
	($value:expr) => {
		CString::new($value).expect(concat!("failed to create CString from '", $value, "'"))
	};
}

macro_rules! dict {
	[$($key:expr => $value:expr),*] => {
		{
			let mut map = HashMap::new();
			$(
				map.insert($key, $value);
			)*
			map
		}
	};
}

round_trip!(round_trip_bool, true, bool, Message::Bool(true));
round_trip!(round_trip_char, 'a', char, Message::Uint64('a' as u64));
round_trip!(round_trip_u8, 42, u8, Message::Uint64(42));
round_trip!(round_trip_i8, 42, i8, Message::Int64(42));
round_trip!(round_trip_u16, 42, u16, Message::Uint64(42));
round_trip!(round_trip_i16, 42, i16, Message::Int64(42));
round_trip!(round_trip_u32, 42, u32, Message::Uint64(42));
round_trip!(round_trip_i32, 42, i32, Message::Int64(42));
round_trip!(round_trip_u64, 42, u64, Message::Uint64(42));
round_trip!(round_trip_i64, 42, i64, Message::Int64(42));
round_trip![round_trip_f32, 42.123, f32];
round_trip![round_trip_f64, 42.123456789, f64];
round_trip!(
	round_trip_bytes,
	Bytes::from_static(&[0, 1, 2, 3, 4, 5, 6, 7, 8, 9]),
	Bytes,
	Message::Data(vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9])
);
round_trip!(
	round_trip_string,
	"Hello World!".to_string(),
	String,
	Message::String(cstr!("Hello World!"))
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
	dict!["foo".to_string() => "bar".to_string()],
	HashMap<String, String>,
	Message::Dictionary(dict![cstr!("foo") => Message::String(cstr!("bar"))])
);
round_trip!(
	round_trip_tuple,
	("Don't Panic!".to_string(), 42),
	(String, u64),
	Message::Array(vec![
		Message::String(cstr!("Don't Panic!")),
		Message::Uint64(42)
	])
);
round_trip!(
	round_trip_some,
	Some("Don't Panic!".to_string()),
	Option<String>,
	Message::String(cstr!("Don't Panic!"))
);
round_trip!(round_trip_none, None, Option<String>, Message::Null);
round_trip!(round_trip_unit, (), (), Message::Null);

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
	Message::Dictionary(dict![cstr!("Single") => Message::Uint64(42)])
);
round_trip!(
	round_trip_tuple_enum,
	TestEnum::Tuple(1, 2),
	TestEnum,
	Message::Dictionary(dict![
		cstr!("Tuple") => Message::Array(vec![Message::Uint64(1), Message::Uint64(2)])
	])
);
round_trip!(
	round_trip_struct_enum,
	TestEnum::Struct {
		a: 1,
		b: "foo".to_string()
	},
	TestEnum,
	Message::Dictionary(dict![
		cstr!("Struct") => Message::Dictionary(dict![
			cstr!("a") => Message::Uint64(1),
			cstr!("b") => Message::String(cstr!("foo"))
		])
	])
);

#[derive(Debug, PartialEq, Serialize, Deserialize)]
struct UnitStruct;
round_trip!(
	round_trip_unit_struct,
	UnitStruct,
	UnitStruct,
	Message::Null
);

#[derive(Debug, PartialEq, Serialize, Deserialize)]
struct NewtypeStruct(u32);
round_trip!(
	round_trip_newtype_struct,
	NewtypeStruct(42),
	NewtypeStruct,
	Message::Uint64(42)
);

#[derive(Debug, PartialEq, Serialize, Deserialize)]
struct TupleStruct(u32, u32);
round_trip!(
	round_trip_tuple_struct,
	TupleStruct(1, 2),
	TupleStruct,
	Message::Array(vec![Message::Uint64(1), Message::Uint64(2)])
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
	Message::Dictionary(dict![
		cstr!("a") => Message::Uint64(1),
		cstr!("b") => Message::String(cstr!("foo")),
		cstr!("c") => Message::Bool(true)
	])
);

#[derive(Debug, PartialEq, Serialize, Deserialize)]
struct ComplexStruct {
	a: TestStruct,
	b: (TestEnum, TestEnum, TestEnum, TestEnum),
	c: Vec<u64>,
	d: HashMap<String, u64>,
}
round_trip!(
	round_trip_complex_struct,
	ComplexStruct {
		a: TestStruct {
			a: 42,
			b: "foo".to_string(),
			c: true
		},
		b: (
			TestEnum::Simple,
			TestEnum::Single(42),
			TestEnum::Tuple(1, 2),
			TestEnum::Struct {
				a: 1,
				b: "bar".to_string()
			}
		),
		c: vec![1, 2, 3, 4, 5],
		d: dict!["foo".to_string() => 42, "bar".to_string() => 1337]
	},
	ComplexStruct,
	Message::Dictionary(dict![
		cstr!("a") => Message::Dictionary(dict![
			cstr!("a") => Message::Uint64(42),
			cstr!("b") => Message::String(cstr!("foo")),
			cstr!("c") => Message::Bool(true)
		]),
		cstr!("b") => Message::Array(vec![
			Message::String(cstr!("Simple")),
			Message::Dictionary(dict![
				cstr!("Single") => Message::Uint64(42)
			]),
			Message::Dictionary(dict![
				cstr!("Tuple") => Message::Array(vec![Message::Uint64(1), Message::Uint64(2)])
			]),
			Message::Dictionary(dict![
				cstr!("Struct") => Message::Dictionary(dict![
					cstr!("a") => Message::Uint64(1),
					cstr!("b") => Message::String(cstr!("bar"))
				])
			])
		]),
		cstr!("c") => Message::Array(vec![
			Message::Uint64(1),
			Message::Uint64(2),
			Message::Uint64(3),
			Message::Uint64(4),
			Message::Uint64(5)
		]),
		cstr!("d") => Message::Dictionary(dict![
			cstr!("foo") => Message::Uint64(42),
			cstr!("bar") => Message::Uint64(1337)
		])
	])
);
