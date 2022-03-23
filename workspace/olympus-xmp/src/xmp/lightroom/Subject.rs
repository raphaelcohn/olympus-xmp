// This file is part of olympus-xmp. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/raphaelcohn/olympus-xmp/master/COPYRIGHT. No part of olympus-xmp, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2022 The developers of olympus-xmp. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/raphaelcohn/olympus-xmp/master/COPYRIGHT.


/// A string which is not empty.
#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub struct Subject<'a>(NonEmptyStr<'a>);

impl<'a> const From<&'a str> for Subject<'a>
{
	#[inline(always)]
	fn from(value: &'a str) -> Self
	{
		Self::from(NonEmptyStr::from(value))
	}
}

impl<'a> const From<NonEmptyStr<'a>> for Subject<'a>
{
	#[inline(always)]
	fn from(value: NonEmptyStr<'a>) -> Self
	{
		Self(value)
	}
}

impl<'a> const From<Subject<'a>> for NonEmptyStr<'a>
{
	#[inline(always)]
	fn from(value: Subject<'a>) -> Self
	{
		value.0
	}
}

impl<'a> const AsRef<str> for Subject<'a>
{
	#[inline(always)]
	fn as_ref(&self) -> &str
	{
		self.0.as_ref()
	}
}

impl<'a> const Borrow<str> for Subject<'a>
{
	#[inline(always)]
	fn borrow(&self) -> &str
	{
		self.0.borrow()
	}
}

impl<'a> const Deref for Subject<'a>
{
	type Target = str;
	
	#[inline(always)]
	fn deref(&self) -> &Self::Target
	{
		self.0.deref()
	}
}

impl<'a> XmpAttributeValue<'a> for Subject<'a>
{
	type Error = NonEmptyStrParseError;
	
	#[inline(always)]
	fn parse(raw: &'a str) -> Result<Self, Self::Error>
	{
		NonEmptyStr::parse(raw).map(Self)
	}
	
	#[inline(always)]
	fn into_xmp_attribute_value_parse_error(error: Self::Error) -> XmpAttributeValueParseError
	{
		XmpAttributeValueParseError::Subject(error)
	}
}
