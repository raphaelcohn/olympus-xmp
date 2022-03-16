// This file is part of olympus-xmp. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/raphaelcohn/olympus-xmp/master/COPYRIGHT. No part of olympus-xmp, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2022 The developers of olympus-xmp. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/raphaelcohn/olympus-xmp/master/COPYRIGHT.


trait ParseRecord: Clone
{
	type Key: Clone + Eq + Hash + 'static;
	
	#[inline(always)]
	fn parse_key_range(inclusive_from: &str, inclusive_to: &str) -> Result<&'static [Self::Key], TagOrSubtagRangeError>
	{
		Err(TagOrSubtagRangeError::TypeDoesNotHaveRangeSupportImplemented { inclusive_from: inclusive_from.to_string(), inclusive_to: inclusive_to.to_string() })
	}
	
	fn parse_key(tag_or_subtag: String) -> Result<Self::Key, KeyParseError>;
	
	fn parse(preferred_value: Option<String>, prefix: Vec<String>, suppress_script: Option<String>, macrolanguage: Option<String>, scope: Option<Scope>) -> Result<Self, RecordParseError>;
	
	#[doc(hidden)]
	#[inline(always)]
	fn subtag_to_byte_array<const C: usize>(subtag: &str) -> Result<[u8; C], KeyParseError>
	{
		use KeyParseError::*;
		
		{
			let length = subtag.len();
			if length != C
			{
				return Err(SubtagInvalidLength { length, minimum: C, maximum: C, subtag: subtag.to_string() })
			}
		}
		
		Self::validate_is_lower_case_alpha(subtag)?;
		
		let mut key: [MaybeUninit<u8>; C] = MaybeUninit::uninit_array();
		let slice = MaybeUninit::slice_as_mut_ptr(&mut key);
		let pointer = subtag.as_ptr();
		Ok
		(
			unsafe
			{
				slice.copy_from_nonoverlapping(pointer, C);
				MaybeUninit::array_assume_init(key)
			}
		)
	}
	
	#[doc(hidden)]
	#[inline(always)]
	fn validate_is_lower_case_alpha(value: &str) -> Result<(), KeyParseError>
	{
		let bytes = value.as_bytes();
		for index in 0 .. bytes.len()
		{
			match bytes.get_unchecked_value_safe(index)
			{
				b'a' ..= b'z' => (),
				
				byte @ _ => return Err(KeyParseError::TagOrSubtagIsNotLowercaseAlpha { index, byte })
			}
		}
		Ok(())
	}
}
