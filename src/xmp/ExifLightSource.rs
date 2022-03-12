// This file is part of olympus-xmp. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/raphaelcohn/olympus-xmp/master/COPYRIGHT. No part of olympus-xmp, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2022 The developers of olympus-xmp. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/raphaelcohn/olympus-xmp/master/COPYRIGHT.


/// A light source (white balance).
///
/// Defaults to `Unknown`.
#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
#[repr(u16)]
pub enum ExifLightSource
{
	#[allow(missing_docs)]
	Unknown = 0,
	
	#[allow(missing_docs)]
	Daylight = 1,
	
	#[allow(missing_docs)]
	Fluorescent = 2,
	
	/// Incandescent light.
	Tungsten = 3,
	
	#[allow(missing_docs)]
	Flash = 4,
	
	#[allow(missing_docs)]
	FineWeather = 9,
	
	#[allow(missing_docs)]
	CloudyWeather = 10,
	
	#[allow(missing_docs)]
	Shade = 11,
	
	/// D: 5700°K - 7100°K.
	DaylightFluorescent = 12,
	
	/// N: 4600°K - 5500°K.
	DayWhiteFluorescent = 13,
	
	/// W: 3800°K - 4500°K.
	CoolWhiteFluorescent = 14,
	
	/// WW: 3250°K - 3800°K.
	WhiteFluorescent = 15,
	
	/// L: 2600°K - 3250°K.
	WarmWhiteFluorescent = 16,
	
	#[allow(missing_docs)]
	StandardLightA = 17,
	
	#[allow(missing_docs)]
	StandardLightB = 18,
	
	#[allow(missing_docs)]
	StandardLightC = 19,
	
	#[allow(missing_docs)]
	D55 = 20,
	
	#[allow(missing_docs)]
	D65 = 21,
	
	#[allow(missing_docs)]
	D75 = 22,
	
	#[allow(missing_docs)]
	D50 = 23,
	
	/// ISO Studio Tungsten (Incandescent Light).
	IsoStudioTungsten = 24,
	
	#[allow(missing_docs)]
	Other = 255,
}

impl const Default for ExifLightSource
{
	#[inline(always)]
	fn default() -> Self
	{
		ExifLightSource::Unknown
	}
}

impl<'a> XmpAttributeValue<'a> for ExifLightSource
{
	type Error = U16ParseError;
	
	#[inline(always)]
	fn parse(value: &'a str) -> Result<Self, Self::Error>
	{
		use U16ParseError::*;
		
		let value = u16::from_str(value).map_err(InvalidU16)?;
		match value
		{
			0 ..= 4 | 9 ..= 24 | 255 => Ok(unsafe { transmute(value) }),
			
			_ => Err(InvalidValue(value)),
		}
	}
	
	#[inline(always)]
	fn into_xmp_attribute_value_parse_error(error: Self::Error) -> XmpAttributeValueParseError
	{
		XmpAttributeValueParseError::ExifLightSource(error)
	}
}
