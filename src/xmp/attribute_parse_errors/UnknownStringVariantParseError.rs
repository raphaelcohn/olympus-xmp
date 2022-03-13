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
	fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result
	{
		Debug::fmt(self, f)
	}
}

impl error::Error for UnknownStringVariantParseError
{
}

impl UnknownStringVariantParseError
{
	#[inline(always)]
	pub(super) fn parse_prefixed_value_returning_suffix<'a, const prefix: &'static str>(raw: &'a str) -> Result<&'a str, Self>
	{
		let needle =
		{
			let length = prefix.len();
			debug_assert_ne!(length, 0);
			let prefix_ = prefix.as_bytes();
			prefix_.get_unchecked_value_safe(length - 1)
		};
		
		let bytes = raw.as_bytes();
		let index = memrchr(needle, bytes).ok_or(Self::from(raw))?;
		
		{
			let actual_prefix = bytes.get_unchecked_range_safe(..index);
			if unsafe { from_utf8_unchecked(actual_prefix) } != prefix
			{
				return Err(Self::from(raw))
			}
		}
		
		let actual_suffix = bytes.get_unchecked_range_safe((index + 1) .. );
		Ok(unsafe { from_utf8_unchecked(actual_suffix) })
	}
}
