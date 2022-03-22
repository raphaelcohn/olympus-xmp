// This file is part of olympus-xmp. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/raphaelcohn/olympus-xmp/master/COPYRIGHT. No part of olympus-xmp, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2022 The developers of olympus-xmp. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/raphaelcohn/olympus-xmp/master/COPYRIGHT.


/// `(2*8alphanum)`.
#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub enum ExtensionPortion
{
	#[allow(missing_docs)]
	Alphanumeric2([Alphanumeric; 2]),
	
	#[allow(missing_docs)]
	Alphanumeric3([Alphanumeric; 3]),
	
	#[allow(missing_docs)]
	Alphanumeric4([Alphanumeric; 4]),
	
	#[allow(missing_docs)]
	Alphanumeric5([Alphanumeric; 5]),
	
	#[allow(missing_docs)]
	Alphanumeric6([Alphanumeric; 6]),
	
	#[allow(missing_docs)]
	Alphanumeric7([Alphanumeric; 7]),
	
	#[allow(missing_docs)]
	Alphanumeric8([Alphanumeric; 8]),
}

impl const From<[u8; 2]> for ExtensionPortion
{
	#[inline(always)]
	fn from(value: [u8; 2]) -> Self
	{
		Self::from(Alphanumeric::new_array_unchecked(value))
	}
}

impl<'a> const From<&'a [u8; 2]> for ExtensionPortion
{
	#[inline(always)]
	fn from(value: &'a [u8; 2]) -> Self
	{
		Self::from(Alphanumeric::new_array_unchecked_ref(value))
	}
}

impl const From<[Alphanumeric; 2]> for ExtensionPortion
{
	#[inline(always)]
	fn from(value: [Alphanumeric; 2]) -> Self
	{
		ExtensionPortion::Alphanumeric2(value)
	}
}

impl const From<[u8; 3]> for ExtensionPortion
{
	#[inline(always)]
	fn from(value: [u8; 3]) -> Self
	{
		Self::from(Alphanumeric::new_array_unchecked(value))
	}
}

impl<'a> const From<&'a [u8; 3]> for ExtensionPortion
{
	#[inline(always)]
	fn from(value: &'a [u8; 3]) -> Self
	{
		Self::from(Alphanumeric::new_array_unchecked_ref(value))
	}
}

impl const From<[Alphanumeric; 3]> for ExtensionPortion
{
	#[inline(always)]
	fn from(value: [Alphanumeric; 3]) -> Self
	{
		ExtensionPortion::Alphanumeric3(value)
	}
}

impl const From<[u8; 4]> for ExtensionPortion
{
	#[inline(always)]
	fn from(value: [u8; 4]) -> Self
	{
		Self::from(Alphanumeric::new_array_unchecked(value))
	}
}

impl<'a> const From<&'a [u8; 4]> for ExtensionPortion
{
	#[inline(always)]
	fn from(value: &'a [u8; 4]) -> Self
	{
		Self::from(Alphanumeric::new_array_unchecked_ref(value))
	}
}

impl const From<[Alphanumeric; 4]> for ExtensionPortion
{
	#[inline(always)]
	fn from(value: [Alphanumeric; 4]) -> Self
	{
		ExtensionPortion::Alphanumeric4(value)
	}
}

impl const From<[u8; 5]> for ExtensionPortion
{
	#[inline(always)]
	fn from(value: [u8; 5]) -> Self
	{
		Self::from(Alphanumeric::new_array_unchecked(value))
	}
}

impl<'a> const From<&'a [u8; 5]> for ExtensionPortion
{
	#[inline(always)]
	fn from(value: &'a [u8; 5]) -> Self
	{
		Self::from(Alphanumeric::new_array_unchecked_ref(value))
	}
}

impl const From<[Alphanumeric; 5]> for ExtensionPortion
{
	#[inline(always)]
	fn from(value: [Alphanumeric; 5]) -> Self
	{
		ExtensionPortion::Alphanumeric5(value)
	}
}

impl const From<[u8; 6]> for ExtensionPortion
{
	#[inline(always)]
	fn from(value: [u8; 6]) -> Self
	{
		Self::from(Alphanumeric::new_array_unchecked(value))
	}
}

impl<'a> const From<&'a [u8; 6]> for ExtensionPortion
{
	#[inline(always)]
	fn from(value: &'a [u8; 6]) -> Self
	{
		Self::from(Alphanumeric::new_array_unchecked_ref(value))
	}
}

impl const From<[Alphanumeric; 6]> for ExtensionPortion
{
	#[inline(always)]
	fn from(value: [Alphanumeric; 6]) -> Self
	{
		ExtensionPortion::Alphanumeric6(value)
	}
}

impl const From<[u8; 7]> for ExtensionPortion
{
	#[inline(always)]
	fn from(value: [u8; 7]) -> Self
	{
		Self::from(Alphanumeric::new_array_unchecked(value))
	}
}

impl<'a> const From<&'a [u8; 7]> for ExtensionPortion
{
	#[inline(always)]
	fn from(value: &'a [u8; 7]) -> Self
	{
		Self::from(Alphanumeric::new_array_unchecked_ref(value))
	}
}

impl const From<[Alphanumeric; 7]> for ExtensionPortion
{
	#[inline(always)]
	fn from(value: [Alphanumeric; 7]) -> Self
	{
		ExtensionPortion::Alphanumeric7(value)
	}
}

impl const From<[u8; 8]> for ExtensionPortion
{
	#[inline(always)]
	fn from(value: [u8; 8]) -> Self
	{
		Self::from(Alphanumeric::new_array_unchecked(value))
	}
}

impl<'a> const From<&'a [u8; 8]> for ExtensionPortion
{
	#[inline(always)]
	fn from(value: &'a [u8; 8]) -> Self
	{
		Self::from(Alphanumeric::new_array_unchecked_ref(value))
	}
}

impl const From<[Alphanumeric; 8]> for ExtensionPortion
{
	#[inline(always)]
	fn from(value: [Alphanumeric; 8]) -> Self
	{
		ExtensionPortion::Alphanumeric8(value)
	}
}
