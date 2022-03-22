// This file is part of olympus-xmp. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/raphaelcohn/olympus-xmp/master/COPYRIGHT. No part of olympus-xmp, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2022 The developers of olympus-xmp. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/raphaelcohn/olympus-xmp/master/COPYRIGHT.


/// A string which is not empty.
#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub struct NonEmptyStr<'a>(pub &'a str);

impl<'a> const From<&'a str> for NonEmptyStr<'a>
{
	#[inline(always)]
	fn from(value: &'a str) -> Self
	{
		Self(value)
	}
}

impl<'a> const From<NonEmptyStr<'a>> for &'a str
{
	#[inline(always)]
	fn from(value: NonEmptyStr<'a>) -> Self
	{
		value.0
	}
}

impl<'a> const AsRef<str> for NonEmptyStr<'a>
{
	#[inline(always)]
	fn as_ref(&self) -> &str
	{
		self.0
	}
}

impl<'a> const Borrow<str> for NonEmptyStr<'a>
{
	#[inline(always)]
	fn borrow(&self) -> &str
	{
		self.0
	}
}

impl<'a> const Deref for NonEmptyStr<'a>
{
	type Target = str;
	
	#[inline(always)]
	fn deref(&self) -> &Self::Target
	{
		self.0
	}
}

impl<'a> XmpAttributeValue<'a> for NonEmptyStr<'a>
{
	type Error = NonEmptyStrParseError;
	
	#[inline(always)]
	fn parse(raw: &'a str) -> Result<Self, Self::Error>
	{
		if raw.is_empty()
		{
			Err(NonEmptyStrParseError)
		}
		else
		{
			Ok(Self(raw))
		}
	}
	
	#[inline(always)]
	fn into_xmp_attribute_value_parse_error(error: Self::Error) -> XmpAttributeValueParseError
	{
		XmpAttributeValueParseError::NonEmptyStr(error)
	}
}
