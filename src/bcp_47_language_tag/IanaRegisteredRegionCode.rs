// This file is part of olympus-xmp. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/raphaelcohn/olympus-xmp/master/COPYRIGHT. No part of olympus-xmp, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2022 The developers of olympus-xmp. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/raphaelcohn/olympus-xmp/master/COPYRIGHT.


#[allow(missing_docs)]
#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub enum IanaRegisteredRegionCode
{
	#[allow(missing_docs)]
	IanaRegisteredIso3166Dash1Alpha2CountryCode(IanaRegisteredIso3166Dash1Alpha2CountryCode),
	
	/// UN Standard Country or Area Codes for Statistical Use (Series M, Nº 49) region code registered in the IANA language registry.
	#[allow(missing_docs)]
	IanaRegisteredUnM49RegionCode(IanaRegisteredUnM49RegionCode),
}

impl<'a> const From<&'a [u8; 2]> for IanaRegisteredRegionCode
{
	#[inline(always)]
	fn from(value: &'a [u8; 2]) -> Self
	{
		IanaRegisteredRegionCode::IanaRegisteredIso3166Dash1Alpha2CountryCode(IanaRegisteredIso3166Dash1Alpha2CountryCode::from(value))
	}
}

impl const From<[u8; 2]> for IanaRegisteredRegionCode
{
	#[inline(always)]
	fn from(value: [u8; 2]) -> Self
	{
		IanaRegisteredRegionCode::IanaRegisteredIso3166Dash1Alpha2CountryCode(IanaRegisteredIso3166Dash1Alpha2CountryCode::from(value))
	}
}

impl<'a> const From<&'a [u8; 3]> for IanaRegisteredRegionCode
{
	#[inline(always)]
	fn from(value: &'a [u8; 3]) -> Self
	{
		IanaRegisteredRegionCode::IanaRegisteredUnM49RegionCode(IanaRegisteredUnM49RegionCode::from(value))
	}
}

impl const From<[u8; 3]> for IanaRegisteredRegionCode
{
	#[inline(always)]
	fn from(value: [u8; 3]) -> Self
	{
		IanaRegisteredRegionCode::IanaRegisteredUnM49RegionCode(IanaRegisteredUnM49RegionCode::from(value))
	}
}
