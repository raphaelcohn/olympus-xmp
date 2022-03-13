// This file is part of olympus-xmp. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/raphaelcohn/olympus-xmp/master/COPYRIGHT. No part of olympus-xmp, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2022 The developers of olympus-xmp. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/raphaelcohn/olympus-xmp/master/COPYRIGHT.


/// A label.
#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
#[repr(i8)]
pub enum XmpLabel
{
	#[allow(missing_docs)]
	Select = -1,

	#[allow(missing_docs)]
	Second = 0,

	#[allow(missing_docs)]
	Approved = 1,

	#[allow(missing_docs)]
	Review = 2,

	#[allow(missing_docs)]
	To_Do = 3,
}

impl<'a> XmpAttributeValue<'a> for XmpLabel
{
	type Error = UnknownStringVariantParseError;
	
	#[inline(always)]
	fn parse(raw: &'a str) -> Result<Self, Self::Error>
	{
		use XmpLabel::*;
		match raw
		{
			"Select" => Ok(Select),
			
			"Second" => Ok(Second),
			
			"Approved" => Ok(Approved),
			
			"Review" => Ok(Review),
			
			"To Do" => Ok(To_Do),
			
			_ => Err(UnknownStringVariantParseError::from(raw)),
		}
	}
	
	#[inline(always)]
	fn into_xmp_attribute_value_parse_error(error: Self::Error) -> XmpAttributeValueParseError
	{
		XmpAttributeValueParseError::XmpLabel(error)
	}
}
