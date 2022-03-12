// This file is part of olympus-xmp. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/raphaelcohn/olympus-xmp/master/COPYRIGHT. No part of olympus-xmp, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2022 The developers of olympus-xmp. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/raphaelcohn/olympus-xmp/master/COPYRIGHT.


/// A sensitivity type.
///
/// See ISO 12232.
#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
#[repr(u16)]
pub enum ExifSensitivityType
{
	#[allow(missing_docs)]
	Unknown = 0,

	/// Standard Output Sensitivity (SOS).
	StandardOutputSensitivity = 1,

	/// Recommended Expure Index (REI).
	RecommendedExposureIndex = 2,

	/// ISO speed.
	IsoSpeed = 3,
	
	/// Standard Output Sensitivity (SOS) and Recommended Expure Index (REI).
	StandardOutputSensitivityAndRecommendedExposureIndex = 4,
	
	/// Standard Output Sensitivity (SOS) and ISO speed.
	StandardOutputSensitivityAndIsoSpeed = 5,
	
	/// Recommended Expure Index (REI) and ISO speed.
	RecommendedExposureIndexAndIsoSpeed = 6,
	
	/// Standard Output Sensitivity (SOS), Recommended Expure Index (REI) and ISO speed.
	StandardOutputSensitivityAndRecommendedExposureIndexAndIsoSpeed = 7,
}

impl<'a> XmpAttributeValue<'a> for ExifSensitivityType
{
	type Error = U16ParseError;
	
	#[inline(always)]
	fn parse(value: &'a str) -> Result<Self, Self::Error>
	{
		use U16ParseError::*;
		
		let value = u16::from_str(value).map_err(InvalidU16)?;
		match value
		{
			0 ..= 7 => Ok(unsafe { transmute(value) }),
			
			_ => Err(InvalidValue(value)),
		}
	}
	
	#[inline(always)]
	fn into_xmp_attribute_value_parse_error(error: Self::Error) -> XmpAttributeValueParseError
	{
		XmpAttributeValueParseError::ExifSensitivityType(error)
	}
}
