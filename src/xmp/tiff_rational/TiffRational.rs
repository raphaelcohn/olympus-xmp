// This file is part of olympus-xmp. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/raphaelcohn/olympus-xmp/master/COPYRIGHT. No part of olympus-xmp, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2022 The developers of olympus-xmp. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/raphaelcohn/olympus-xmp/master/COPYRIGHT.



/// A TIFF `RATIONAL` field represented as two `LONG`s.
///
/// In effect, a fraction.
///
/// Format is `LONG/LONG`.
///
/// A `LONG` is an unsigned 32-bit integer in decimal.
#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub struct TiffRational
{
	numerator: u32,

	denominator: NonZeroU32,
}

impl const Default for TiffRational
{
	#[inline(always)]
	fn default() -> Self
	{
		Self
		{
			numerator: 0,
			denominator: new_non_zero_u32(1),
		}
	}
}

impl<'a> XmpAttributeValue<'a> for TiffRational
{
	type Error = TiffRationalParseError;
	
	#[inline(always)]
	fn parse(value: &'a str) -> Result<Self, Self::Error>
	{
		use TiffRationalParseError::*;
		let bytes = value.as_bytes();
		
		let index = memchr(b'/', bytes).ok_or(MissingDenominator)?;
		
		#[inline(always)]
		fn parse_unsigned_integer<AUR: AsUsizeRange<u8>, U: FromStr<Err=ParseIntError>>(bytes: &[u8], range: AUR,  error: impl FnOnce(ParseIntError) -> TiffRationalParseError) -> Result<U, TiffRationalParseError>
		{
			let bytes = bytes.get_unchecked_range_safe(range);
			U::from_str(unsafe { from_utf8_unchecked(bytes) }).map_err(error)
		}
		
		Ok
		(
			Self
			{
				numerator: parse_unsigned_integer(bytes, .. index, InvalidU32Numerator)?,
				denominator: parse_unsigned_integer(bytes, (index + 1) .., InvalidNonZeroU32Denominator)?,
			}
		)
	}
	
	#[inline(always)]
	fn into_xmp_attribute_value_parse_error(error: Self::Error) -> XmpAttributeValueParseError
	{
		XmpAttributeValueParseError::TiffRational(error)
	}
}
