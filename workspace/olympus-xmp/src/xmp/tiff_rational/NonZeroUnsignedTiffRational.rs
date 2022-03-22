// This file is part of olympus-xmp. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/raphaelcohn/olympus-xmp/master/COPYRIGHT. No part of olympus-xmp, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2022 The developers of olympus-xmp. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/raphaelcohn/olympus-xmp/master/COPYRIGHT.


/// A non-zero TIFF `RATIONAL` field represented as two `LONG`s.
///
/// In effect, a fraction.
///
/// Format is `LONG/LONG`.
///
/// A `LONG` is an unsigned 32-bit integer in decimal.
#[derive(Debug, Copy, Clone, Hash)]
pub struct NonZeroUnsignedTiffRational
{
	numerator: NonZeroU32,

	denominator: NonZeroU32,
}

impl const From<NonZeroU8> for NonZeroUnsignedTiffRational
{
	#[inline(always)]
	fn from(value: NonZeroU8) -> Self
	{
		Self::scalar_unchecked(value.get() as u32)
	}
}

impl const From<NonZeroU16> for NonZeroUnsignedTiffRational
{
	fn from(value: NonZeroU16) -> Self
	{
		Self::scalar_unchecked(value.get() as u32)
	}
}

impl const From<NonZeroU32> for NonZeroUnsignedTiffRational
{
	fn from(value: NonZeroU32) -> Self
	{
		Self::scalar(value)
	}
}

impl const Default for NonZeroUnsignedTiffRational
{
	#[inline(always)]
	fn default() -> Self
	{
		Self::scalar(Self::One)
	}
}

impl PartialEq for NonZeroUnsignedTiffRational
{
	#[inline(always)]
	fn eq(&self, other: &Self) -> bool
	{
		let (left, right) = self.scale(other);
		left == right
	}
}

impl Eq for NonZeroUnsignedTiffRational
{
}

impl PartialOrd for NonZeroUnsignedTiffRational
{
	#[inline(always)]
	fn partial_cmp(&self, other: &Self) -> Option<Ordering>
	{
		Some(self.cmp(other))
	}
}

impl Ord for NonZeroUnsignedTiffRational
{
	#[inline(always)]
	fn cmp(&self, other: &Self) -> Ordering
	{
		let (left, right) = self.scale(other);
		left.cmp(&right)
	}
}

impl Mul for NonZeroUnsignedTiffRational
{
	type Output = Self;
	
	#[inline(always)]
	fn mul(self, rhs: Self) -> Self::Output
	{
		let numerator = (self.numerator() as u64) * (rhs.numerator() as u64);
		let denominator = (self.denominator() as u64) * (rhs.denominator() as u64);
		let greatest_common_divisor = binary_u64(numerator, denominator);
		Self
		{
			numerator: new_non_zero_u32((numerator / greatest_common_divisor) as u32),
			denominator: new_non_zero_u32((denominator / greatest_common_divisor) as u32),
		}
	}
}

impl<'a> XmpAttributeValue<'a> for NonZeroUnsignedTiffRational
{
	type Error = NonZeroUnsignedTiffRationalParseError;
	
	#[inline(always)]
	fn parse(value: &'a str) -> Result<Self, Self::Error>
	{
		let UnsignedTiffRational { numerator, denominator } = UnsignedTiffRational::parse(value).map_err(NonZeroUnsignedTiffRationalParseError::TiffRational)?;
		if numerator == 0
		{
			Err(NonZeroUnsignedTiffRationalParseError::ZeroNumerator)
		}
		else
		{
			Ok(Self { numerator: new_non_zero_u32(numerator), denominator })
		}
	}
	
	#[inline(always)]
	fn into_xmp_attribute_value_parse_error(error: Self::Error) -> XmpAttributeValueParseError
	{
		XmpAttributeValueParseError::NonZeroUnsignedTiffRational(error)
	}
}

impl NonZeroUnsignedTiffRational
{
	const One: NonZeroU32 = new_non_zero_u32(1);
	
	#[inline(always)]
	const fn scalar_unchecked(numerator: u32) -> Self
	{
		Self::scalar(new_non_zero_u32(numerator))
	}
	
	#[inline(always)]
	const fn scalar(numerator: NonZeroU32) -> Self
	{
		Self
		{
			numerator,
		
			denominator: Self::One,
		}
	}
	
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
			numerator: new_non_zero_u32(self.numerator() / greatest_common_divisor),
			denominator: new_non_zero_u32(self.denominator() / greatest_common_divisor),
		}
	}
	
	#[inline(always)]
	const fn greatest_common_divisor(&self) -> u32
	{
		binary_u32(self.numerator(), self.denominator())
	}
	
	#[inline(always)]
	const fn numerator(&self) -> u32
	{
		self.numerator.get()
	}
	
	#[inline(always)]
	const fn denominator(&self) -> u32
	{
		self.denominator.get()
	}
}
