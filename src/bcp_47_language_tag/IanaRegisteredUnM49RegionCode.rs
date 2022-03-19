// This file is part of olympus-xmp. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/raphaelcohn/olympus-xmp/master/COPYRIGHT. No part of olympus-xmp, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2022 The developers of olympus-xmp. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/raphaelcohn/olympus-xmp/master/COPYRIGHT.


/// UN Standard Country or Area Codes for Statistical Use (Series M, Nº 49) region code registered in the IANA language registry.
#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub struct IanaRegisteredUnM49RegionCode([Digit; 3]);

impl<'a> const From<&'a [u8; 3]> for IanaRegisteredUnM49RegionCode
{
	#[inline(always)]
	fn from(value: &'a [u8; 3]) -> Self
	{
		Self(unsafe { transmute_copy(value) })
	}
}

impl const From<[u8; 3]> for IanaRegisteredUnM49RegionCode
{
	#[inline(always)]
	fn from(value: [u8; 3]) -> Self
	{
		Self(unsafe { transmute(value) })
	}
}
