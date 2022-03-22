// This file is part of olympus-xmp. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/raphaelcohn/olympus-xmp/master/COPYRIGHT. No part of olympus-xmp, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2022 The developers of olympus-xmp. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/raphaelcohn/olympus-xmp/master/COPYRIGHT.


#[allow(missing_docs)]
#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub struct IanaRegisteredIso639Alpha3Code([Alpha; 3]);

impl Into<[Alpha; 3]> for IanaRegisteredIso639Alpha3Code
{
	#[inline(always)]
	fn into(self) -> [Alpha; 3]
	{
		self.0
	}
}

impl<'a> const From<&'a [u8; 3]> for IanaRegisteredIso639Alpha3Code
{
	#[inline(always)]
	fn from(value: &'a [u8; 3]) -> Self
	{
		Self::from(Alpha::new_array_unchecked_ref(value))
	}
}

impl const From<[u8; 3]> for IanaRegisteredIso639Alpha3Code
{
	#[inline(always)]
	fn from(value: [u8; 3]) -> Self
	{
		Self::from(Alpha::new_array_unchecked(value))
	}
}

impl const From<[Alpha; 3]> for IanaRegisteredIso639Alpha3Code
{
	#[inline(always)]
	fn from(value: [Alpha; 3]) -> Self
	{
		Self(value)
	}
}
