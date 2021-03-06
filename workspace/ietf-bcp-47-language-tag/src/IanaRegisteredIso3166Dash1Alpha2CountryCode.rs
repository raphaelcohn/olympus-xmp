// This file is part of olympus-xmp. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/raphaelcohn/olympus-xmp/master/COPYRIGHT. No part of olympus-xmp, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2022 The developers of olympus-xmp. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/raphaelcohn/olympus-xmp/master/COPYRIGHT.


#[allow(missing_docs)]
#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub struct IanaRegisteredIso3166Dash1Alpha2CountryCode([UpperCaseAlpha; 2]);

impl Into<[UpperCaseAlpha; 2]> for IanaRegisteredIso3166Dash1Alpha2CountryCode
{
	#[inline(always)]
	fn into(self) -> [UpperCaseAlpha; 2]
	{
		self.0
	}
}

impl<'a> const From<&'a [u8; 2]> for IanaRegisteredIso3166Dash1Alpha2CountryCode
{
	#[inline(always)]
	fn from(value: &'a [u8; 2]) -> Self
	{
		Self(unsafe { transmute_copy(value) })
	}
}

impl const From<[u8; 2]> for IanaRegisteredIso3166Dash1Alpha2CountryCode
{
	#[inline(always)]
	fn from(value: [u8; 2]) -> Self
	{
		Self(unsafe { transmute(value) })
	}
}

impl const From<[UpperCaseAlpha; 2]> for IanaRegisteredIso3166Dash1Alpha2CountryCode
{
	#[inline(always)]
	fn from(value: [UpperCaseAlpha; 2]) -> Self
	{
		IanaRegisteredIso3166Dash1Alpha2CountryCode(value)
	}
}
