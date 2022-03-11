// This file is part of olympus-xmp. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/raphaelcohn/olympus-xmp/master/COPYRIGHT. No part of olympus-xmp, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2022 The developers of olympus-xmp. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/raphaelcohn/olympus-xmp/master/COPYRIGHT.


/// A TIFF `RATIONAL` field represented as two `LONG`s.
///
/// In effect, a fraction.
///
/// Format is `LONG/LONG`.
///
/// A `LONG` is an unsigned 32-bit integer in decimal.
#[derive(Debug, Copy, Clone, Hash)]
pub struct UnsignedTiffRational
{
	numerator: u32,

	denominator: NonZeroU32,
}

impl const From<NonZeroUnsignedTiffRational> for UnsignedTiffRational
{
	#[inline(always)]
	fn from(value: NonZeroUnsignedTiffRational) -> Self
	{
		Self
		{
			numerator: value.numerator(),
			denominator: value.denominator,
		}
	}
}

impl const Default for UnsignedTiffRational
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

impl PartialEq for UnsignedTiffRational
{
	#[inline(always)]
	fn eq(&self, other: &Self) -> bool
	{
		let (left, right) = self.scale(other);
		left == right
	}
}

impl Eq for UnsignedTiffRational
{
}

impl PartialOrd for UnsignedTiffRational
{
	#[inline(always)]
	fn partial_cmp(&self, other: &Self) -> Option<Ordering>
	{
		Some(self.cmp(other))
	}
}

impl Ord for UnsignedTiffRational
{
	#[inline(always)]
	fn cmp(&self, other: &Self) -> Ordering
	{
		let (left, right) = self.scale(other);
		left.cmp(&right)
	}
}

impl Mul for UnsignedTiffRational
{
	type Output = Self;
	
	#[inline(always)]
	fn mul(self, rhs: Self) -> Self::Output
	{
		self.multiply(rhs)
	}
}

impl Mul<NonZeroUnsignedTiffRational> for UnsignedTiffRational
{
	type Output = Self;
	
	#[inline(always)]
	fn mul(self, rhs: NonZeroUnsignedTiffRational) -> Self::Output
	{
		self.multiply(rhs.into())
	}
}

impl<'a> XmpAttributeValue<'a> for UnsignedTiffRational
{
	type Error = UnsignedTiffRationalParseError;
	
	#[inline(always)]
	fn parse(value: &'a str) -> Result<Self, Self::Error>
	{
		use UnsignedTiffRationalParseError::*;
		let bytes = value.as_bytes();
		
		let index = memchr(b'/', bytes).ok_or(MissingDenominator)?;
		
		#[inline(always)]
		fn parse_unsigned_integer<AUR: AsUsizeRange<u8>, U: FromStr<Err=ParseIntError>>(bytes: &[u8], range: AUR,  error: impl FnOnce(ParseIntError) -> UnsignedTiffRationalParseError) -> Result<U, UnsignedTiffRationalParseError>
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
		XmpAttributeValueParseError::UnsignedTiffRational(error)
	}
}

impl UnsignedTiffRational
{
	#[inline(always)]
	const fn scale(&self, other: &Self) -> (u64, u64)
	{
		let left_numerator = self.numerator() as u64;
		let left_denominator = self.denominator() as u64;
		
		let right_numerator = other.numerator() as u64;
		let right_denominator = other.denominator() as u64;
		
		let left = left_numerator * right_denominator;
		let right = right_numerator * left_denominator;
		
		(left, right)
	}
	
	/// Reduce to smallest possible fraction.
	#[inline(always)]
	pub const fn reduce(&self) -> Self
	{
		let greatest_common_divisor = self.greatest_common_divisor();
		Self
		{
			numerator: self.numerator() / greatest_common_divisor,
			denominator: new_non_zero_u32(self.denominator() / greatest_common_divisor),
		}
	}
	
	#[inline(always)]
	const fn greatest_common_divisor(&self) -> u32
	{
		binary_u32(self.numerator, self.denominator())
	}
	
	#[inline(always)]
	const fn numerator(&self) -> u32
	{
		self.numerator
	}
	
	#[inline(always)]
	const fn denominator(&self) -> u32
	{
		self.denominator.get()
	}
	
	#[inline(always)]
	fn multiply(self, scalar: Self) -> UnsignedTiffRational
	{
		let numerator = (self.numerator() as u64) * (scalar.numerator() as u64);
		let denominator = (self.denominator() as u64) * (scalar.denominator() as u64);
		let greatest_common_divisor = binary_u64(numerator, denominator);
		Self
		{
			numerator: (numerator / greatest_common_divisor) as u32,
			denominator: new_non_zero_u32((denominator / greatest_common_divisor) as u32),
		}
	}
}
