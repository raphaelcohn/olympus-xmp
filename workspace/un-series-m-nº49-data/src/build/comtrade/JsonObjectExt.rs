// This file is part of olympus-xmp. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/raphaelcohn/olympus-xmp/master/COPYRIGHT. No part of olympus-xmp, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2022 The developers of olympus-xmp. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/raphaelcohn/olympus-xmp/master/COPYRIGHT.


trait JsonObjectExt
{
	fn take_array_or_panic(&mut self, key: &str) -> Array;
	
	fn take_object_or_panic(&mut self, key: &str) -> Object;
	
	fn take_string_or_panic(&mut self, key: &str) -> String;
	
	#[doc(hidden)]
	fn take_or_panic(&mut self, key: &str) -> JsonValue;
}

impl JsonObjectExt for Object
{
	#[inline(always)]
	fn take_array_or_panic(&mut self, key: &str) -> Array
	{
		self.take_or_panic(key).as_array_or_panic(key)
	}
	
	#[inline(always)]
	fn take_object_or_panic(&mut self, key: &str) -> Object
	{
		self.take_or_panic(key).as_object_or_panic(key)
	}
	
	#[inline(always)]
	fn take_string_or_panic(&mut self, key: &str) -> String
	{
		self.take_or_panic(key).as_string_or_panic(key)
	}
	
	#[inline(always)]
	fn take_or_panic(&mut self, key: &str) -> JsonValue
	{
		match self.remove(key)
		{
			None => panic!("Could not take value for key '{}'", key),
			
			Some(value) => value
		}
	}
}
