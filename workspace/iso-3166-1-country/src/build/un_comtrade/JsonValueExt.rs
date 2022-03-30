// This file is part of olympus-xmp. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/raphaelcohn/olympus-xmp/master/COPYRIGHT. No part of olympus-xmp, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2022 The developers of olympus-xmp. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/raphaelcohn/olympus-xmp/master/COPYRIGHT.


trait JsonValueExt
{
	fn as_array_or_panic(self, description: &str) -> Array;
	
	fn as_object_or_panic(self, description: &str) -> Object;
	
	fn as_string_or_panic(self, description: &str) -> String;
}

impl JsonValueExt for JsonValue
{
	#[inline(always)]
	fn as_array_or_panic(self, description: &str) -> Array
	{
		match self
		{
			JsonValue::Array(array) => array,
			
			_ => panic!("Expected an array for description '{}' but got something else", description)
		}
	}
	
	#[inline(always)]
	fn as_object_or_panic(self, description: &str) -> Object
	{
		match self
		{
			JsonValue::Object(object) => object,
			
			_ => panic!("Expected an object for description '{}' but got something else", description)
		}
	}
	
	#[inline(always)]
	fn as_string_or_panic(self, description: &str) -> String
	{
		match self
		{
			JsonValue::String(string) => string,
			
			_ => panic!("Expected a string for description '{}' but got something else", description)
		}
	}
}
