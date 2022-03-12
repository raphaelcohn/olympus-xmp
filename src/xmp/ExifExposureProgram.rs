// This file is part of olympus-xmp. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/raphaelcohn/olympus-xmp/master/COPYRIGHT. No part of olympus-xmp, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2022 The developers of olympus-xmp. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/raphaelcohn/olympus-xmp/master/COPYRIGHT.


/// Exposure program.
#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
#[repr(u16)]
pub enum ExifExposureProgram
{
	#[allow(missing_docs)]
	Undefined = 0,

	#[allow(missing_docs)]
	Manual = 1,

	/// Also known as 'Program mode'.
	Normal = 2,

	#[allow(missing_docs)]
	Aperture = 3,

	#[allow(missing_docs)]
	Shutter = 4,

	/// Biased towards depth-of-field.
	Creative = 5,

	/// Biased towards faster shutter speed.
	Action = 6,

	/// For close-up photos with the background out-of-focus.
	Portrait = 7,

	/// For landscape photos with the background in focus.
	Landscape = 8,
}

impl const Default for ExifExposureProgram
{
	#[inline(always)]
	fn default() -> Self
	{
		ExifExposureProgram::Undefined
	}
}

impl<'a> XmpAttributeValue<'a> for ExifExposureProgram
{
	type Error = U16ParseError;
	
	#[inline(always)]
	fn parse(value: &'a str) -> Result<Self, Self::Error>
	{
		use U16ParseError::*;
		
		let value = u16::from_str(value).map_err(InvalidU16)?;
		match value
		{
			0 ..= 8 => Ok(unsafe { transmute(value) }),
			
			_ => Err(InvalidValue(value)),
		}
	}
	
	#[inline(always)]
	fn into_xmp_attribute_value_parse_error(error: Self::Error) -> XmpAttributeValueParseError
	{
		XmpAttributeValueParseError::ExifExposureProgram(error)
	}
}
