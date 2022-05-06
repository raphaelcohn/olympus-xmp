// This file is part of olympus-xmp. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/raphaelcohn/olympus-xmp/master/COPYRIGHT. No part of olympus-xmp, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2022 The developers of olympus-xmp. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/raphaelcohn/olympus-xmp/master/COPYRIGHT.


/// Boolean parse error.
#[derive(Default, Debug, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub struct UnknownStringVariantParseError(String);

impl<'a> From<&'a str> for UnknownStringVariantParseError
{
	#[inline(always)]
	fn from(raw: &'a str) -> Self
	{
		Self(raw.to_string())
	}
}

impl Display for UnknownStringVariantParseError
{
	#[inline(always)]
	fn fmt(&self, formatter: &mut Formatter<'_>) -> fmt::Result
	{
		Debug::fmt(self, formatter)
	}
}

impl error::Error for UnknownStringVariantParseError
{
}

impl UnknownStringVariantParseError
{
	#[allow(missing_docs)]
	#[inline(always)]
	pub fn parse_prefixed_value_returning_suffix<const prefix_suffixed_with_needle: &'static str>(raw: &str) -> Result<&str, Self>
	{
		#[inline(always)]
		fn error(raw: &str) -> Result<&str, UnknownStringVariantParseError>
		{
			Err(UnknownStringVariantParseError::from(raw))
		}
		
		// These variables should be inlined as constant values by the compiler.
		let (expected_index, expected_prefix, needle) = Self::expected_prefix_and_needle(prefix_suffixed_with_needle);
		
		let bytes = raw.as_bytes();
		match bytes.memrchr(needle)
		{
			Some(actual_index) =>
			{
				if actual_index == expected_index
				{
					// Use `expected_index` as this will be an inlined constant value by the compiler.
					let actual_prefix = bytes.get_unchecked_range_safe(.. expected_index);
					if actual_prefix == expected_prefix
					{
						let actual_suffix = bytes.get_unchecked_range_safe((expected_index + 1) .. );
						Ok(unsafe { from_utf8_unchecked(actual_suffix) })
					}
					else
					{
						error(raw)
					}
				}
				else
				{
					error(raw)
				}
			}
			
			None => error(raw),
		}
	}
	
	#[inline(always)]
	fn expected_prefix_and_needle(prefix_suffixed_with_needle: &str) -> (usize, &[u8], u8)
	{
		let prefix_suffixed_with_needle = prefix_suffixed_with_needle.as_bytes();
		let length = prefix_suffixed_with_needle.len();
		if cfg!(debug_assertions)
		{
			if length != 0
			{
				panic!("Zero length prefix")
			}
		}
		let length_of_prefix_without_suffixed_needle = length - 1;
		(
			length_of_prefix_without_suffixed_needle,
			prefix_suffixed_with_needle.get_unchecked_range_safe(.. length_of_prefix_without_suffixed_needle),
			prefix_suffixed_with_needle.get_unchecked_value_safe(length_of_prefix_without_suffixed_needle)
		)
	}
}
