// This file is part of olympus-xmp. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/raphaelcohn/olympus-xmp/master/COPYRIGHT. No part of olympus-xmp, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2022 The developers of olympus-xmp. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/raphaelcohn/olympus-xmp/master/COPYRIGHT.


/// An attribute value.
pub trait XmpAttributeValue<'a>: Sized
{
	/// Error.
	type Error: error::Error;
	
	/// Parse.
	fn parse(raw: &'a str) -> Result<Self, Self::Error>;
	
	#[doc(hidden)]
	fn into_xmp_attribute_value_parse_error(error: Self::Error) -> XmpAttributeValueParseError;
}

impl<'a> XmpAttributeValue<'a> for &'a str
{
	type Error = Infallible;
	
	#[inline(always)]
	fn parse(raw: &'a str) -> Result<Self, Self::Error>
	{
		Ok(raw)
	}
	
	#[inline(always)]
	fn into_xmp_attribute_value_parse_error(_error: Self::Error) -> XmpAttributeValueParseError
	{
		unreachable!("Should be impossible to call as Self::Error is Infallible")
	}
}

impl<'a> XmpAttributeValue<'a> for NonZeroU32
{
	type Error = ParseIntError;
	
	#[inline(always)]
	fn parse(raw: &'a str) -> Result<Self, Self::Error>
	{
		NonZeroU32::from_str(raw)
	}
	
	#[inline(always)]
	fn into_xmp_attribute_value_parse_error(error: Self::Error) -> XmpAttributeValueParseError
	{
		XmpAttributeValueParseError::NonZeroU32(error)
	}
}

impl<'a> XmpAttributeValue<'a> for Option<NonZeroU16>
{
	type Error = ParseIntError;
	
	#[inline(always)]
	fn parse(raw: &'a str) -> Result<Self, Self::Error>
	{
		u16::from_str(raw).map(NonZeroU16::new)
	}
	
	#[inline(always)]
	fn into_xmp_attribute_value_parse_error(error: Self::Error) -> XmpAttributeValueParseError
	{
		XmpAttributeValueParseError::OptionNonZeroU16(error)
	}
}

impl<'a> XmpAttributeValue<'a> for bool
{
	type Error = UnknownStringVariantParseError;
	
	#[inline(always)]
	fn parse(raw: &'a str) -> Result<Self, Self::Error>
	{
		match raw
		{
			"True" => Ok(true),
			
			"False" => Ok(false),
			
			_ => Err(UnknownStringVariantParseError::from(raw))
		}
	}
	
	#[inline(always)]
	fn into_xmp_attribute_value_parse_error(error: Self::Error) -> XmpAttributeValueParseError
	{
		XmpAttributeValueParseError::Boolean(error)
	}
}

impl<'a> XmpAttributeValue<'a> for Iso3166Dash1Country
{
	type Error = UnknownStringVariantParseError;

	#[inline(always)]
	fn parse(raw: &'a str) -> Result<Self, Self::Error>
	{
		Self::from_str(raw)
	}

	#[inline(always)]
	fn into_xmp_attribute_value_parse_error(error: Self::Error) -> XmpAttributeValueParseError
	{
		XmpAttributeValueParseError::Iso3166Dash1Country(error)
	}
}

impl<'a> XmpAttributeValue<'a> for Iso3166Dash1AlphaCountryCode
{
	type Error = Iso3166Dash1AlphaCountryCodeParseError;

	#[inline(always)]
	fn parse(raw: &'a str) -> Result<Self, Self::Error>
	{
		use Iso3166Dash1AlphaCountryCodeParseError::*;
		use Iso3166Dash1AlphaCountryCode::*;
		
		let bytes = raw.as_bytes();
		match bytes.len()
		{
			2 =>
			{
				let token = Self::letter_to_number::<0>(bytes)? + Self::letter_to_number::<1>(bytes)?;
				Ok(Alpha2(unsafe { transmute(token) }))
			},

			3 =>
			{
				let token = Self::letter_to_number::<0>(bytes)? + Self::letter_to_number::<1>(bytes)? + Self::letter_to_number::<2>(bytes)?;
				Ok(Alpha3(unsafe { transmute(token) }))
			},

			_ => Err(LengthIsNot2Or3(raw.to_string()))
		}
	}

	#[inline(always)]
	fn into_xmp_attribute_value_parse_error(error: Self::Error) -> XmpAttributeValueParseError
	{
		XmpAttributeValueParseError::Iso3166Dash1AlphaCountryCode(error)
	}
}
