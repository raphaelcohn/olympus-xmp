// This file is part of olympus-xmp. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/raphaelcohn/olympus-xmp/master/COPYRIGHT. No part of olympus-xmp, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2022 The developers of olympus-xmp. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/raphaelcohn/olympus-xmp/master/COPYRIGHT.


#[allow(missing_docs)]
#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub struct IanaRegisteredIso639Alpha2Code([Alpha; 2]);

impl Into<[Alpha; 2]> for IanaRegisteredIso639Alpha2Code
{
	#[inline(always)]
	fn into(self) -> [Alpha; 2]
	{
		self.0
	}
}

impl<'a> const From<&'a [u8; 2]> for IanaRegisteredIso639Alpha2Code
{
	#[inline(always)]
	fn from(value: &'a [u8; 2]) -> Self
	{
		Self::from(Alpha::new_array_unchecked_ref(value))
	}
}

impl const From<[u8; 2]> for IanaRegisteredIso639Alpha2Code
{
	#[inline(always)]
	fn from(value: [u8; 2]) -> Self
	{
		Self::from(Alpha::new_array_unchecked(value))
	}
}

impl const From<[Alpha; 2]> for IanaRegisteredIso639Alpha2Code
{
	#[inline(always)]
	fn from(value: [Alpha; 2]) -> Self
	{
		Self(value)
	}
}
