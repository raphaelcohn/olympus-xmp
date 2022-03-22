// This file is part of olympus-xmp. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/raphaelcohn/olympus-xmp/master/COPYRIGHT. No part of olympus-xmp, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2022 The developers of olympus-xmp. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/raphaelcohn/olympus-xmp/master/COPYRIGHT.


/// A color mode.
#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
#[repr(u8)]
pub enum PhotoshopColorMode
{
	#[allow(missing_docs)]
	Bitmap = 0,

	#[allow(missing_docs)]
	GrayScale = 1,

	#[allow(missing_docs)]
	IndexedColor = 2,

	#[allow(missing_docs)]
	RgbColor = 3,

	#[allow(missing_docs)]
	CmykColor = 4,

	#[allow(missing_docs)]
	Multichannel = 7,

	#[allow(missing_docs)]
	Duotone = 8,

	#[allow(missing_docs)]
	LabColor = 9,
}

impl<'a> XmpAttributeValue<'a> for PhotoshopColorMode
{
	type Error = U8ParseError;
	
	#[inline(always)]
	fn parse(value: &'a str) -> Result<Self, Self::Error>
	{
		use U8ParseError::*;
		
		let value = u8::from_str(value).map_err(InvalidU8)?;
		match value
		{
			0 ..= 4 | 7 ..= 9 => Ok(unsafe { transmute(value) }),
			
			_ => Err(InvalidValue(value)),
		}
	}
	
	#[inline(always)]
	fn into_xmp_attribute_value_parse_error(error: Self::Error) -> XmpAttributeValueParseError
	{
		XmpAttributeValueParseError::PhotoshopColorMode(error)
	}
}
