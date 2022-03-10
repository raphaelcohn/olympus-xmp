// This file is part of olympus-xmp. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/raphaelcohn/olympus-xmp/master/COPYRIGHT. No part of olympus-xmp, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2022 The developers of olympus-xmp. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/raphaelcohn/olympus-xmp/master/COPYRIGHT.


/// A rating.
#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
#[repr(i8)]
pub enum XmpRating
{
	#[allow(missing_docs)]
	Rejected = -1,

	#[allow(missing_docs)]
	Unrated = 0,

	#[allow(missing_docs)]
	_1 = 1,

	#[allow(missing_docs)]
	_2 = 2,

	#[allow(missing_docs)]
	_3 = 3,

	#[allow(missing_docs)]
	_4 = 4,
	
	#[allow(missing_docs)]
	_5 = 5,
}

impl<'a> XmpAttributeValue<'a> for XmpRating
{
	type Error = I8ParseError;
	
	#[inline(always)]
	fn parse(value: &'a str) -> Result<Self, Self::Error>
	{
		use I8ParseError::*;
		
		let value = i8::from_str(value).map_err(InvalidI8)?;
		match value
		{
			-1 ..= 5 => Ok(unsafe { transmute(value) }),
			
			_ => Err(InvalidValue(value)),
		}
	}
	
	#[inline(always)]
	fn into_xmp_attribute_value_parse_error(error: Self::Error) -> XmpAttributeValueParseError
	{
		XmpAttributeValueParseError::XmpRating(error)
	}
}
