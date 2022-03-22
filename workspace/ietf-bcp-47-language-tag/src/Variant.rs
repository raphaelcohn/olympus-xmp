// This file is part of olympus-xmp. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/raphaelcohn/olympus-xmp/master/COPYRIGHT. No part of olympus-xmp, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2022 The developers of olympus-xmp. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/raphaelcohn/olympus-xmp/master/COPYRIGHT.


/// Must be registered `5*8alphanum / DIGIT 3alphanum`.
#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub enum Variant
{
	#[allow(missing_docs)]
	DigitAlphanumeric3(Digit, [Alphanumeric; 3]),
	
	#[allow(missing_docs)]
	Alphanumeric5([Alphanumeric; 5]),
	
	#[allow(missing_docs)]
	Alphanumeric6([Alphanumeric; 6]),
	
	#[allow(missing_docs)]
	Alphanumeric7([Alphanumeric; 7]),
	
	#[allow(missing_docs)]
	Alphanumeric8([Alphanumeric; 8]),
}

impl const From<[u8; 5]> for Variant
{
	#[inline(always)]
	fn from(value: [u8; 5]) -> Self
	{
		Self::from(Alphanumeric::new_array_unchecked(value))
	}
}

impl<'a> const From<&'a [u8; 5]> for Variant
{
	#[inline(always)]
	fn from(value: &'a [u8; 5]) -> Self
	{
		Self::from(Alphanumeric::new_array_unchecked_ref(value))
	}
}

impl const From<[Alphanumeric; 5]> for Variant
{
	#[inline(always)]
	fn from(value: [Alphanumeric; 5]) -> Self
	{
		Variant::Alphanumeric5(value)
	}
}

impl const From<[u8; 6]> for Variant
{
	#[inline(always)]
	fn from(value: [u8; 6]) -> Self
	{
		Self::from(Alphanumeric::new_array_unchecked(value))
	}
}

impl<'a> const From<&'a [u8; 6]> for Variant
{
	#[inline(always)]
	fn from(value: &'a [u8; 6]) -> Self
	{
		Self::from(Alphanumeric::new_array_unchecked_ref(value))
	}
}

impl const From<[Alphanumeric; 6]> for Variant
{
	#[inline(always)]
	fn from(value: [Alphanumeric; 6]) -> Self
	{
		Variant::Alphanumeric6(value)
	}
}

impl const From<[u8; 7]> for Variant
{
	#[inline(always)]
	fn from(value: [u8; 7]) -> Self
	{
		Self::from(Alphanumeric::new_array_unchecked(value))
	}
}

impl<'a> const From<&'a [u8; 7]> for Variant
{
	#[inline(always)]
	fn from(value: &'a [u8; 7]) -> Self
	{
		Self::from(Alphanumeric::new_array_unchecked_ref(value))
	}
}

impl const From<[Alphanumeric; 7]> for Variant
{
	#[inline(always)]
	fn from(value: [Alphanumeric; 7]) -> Self
	{
		Variant::Alphanumeric7(value)
	}
}

impl const From<[u8; 8]> for Variant
{
	#[inline(always)]
	fn from(value: [u8; 8]) -> Self
	{
		Self::from(Alphanumeric::new_array_unchecked(value))
	}
}

impl<'a> const From<&'a [u8; 8]> for Variant
{
	#[inline(always)]
	fn from(value: &'a [u8; 8]) -> Self
	{
		Self::from(Alphanumeric::new_array_unchecked_ref(value))
	}
}

impl const From<[Alphanumeric; 8]> for Variant
{
	#[inline(always)]
	fn from(value: [Alphanumeric; 8]) -> Self
	{
		Variant::Alphanumeric8(value)
	}
}
