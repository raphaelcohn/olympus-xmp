// This file is part of olympus-xmp. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/raphaelcohn/olympus-xmp/master/COPYRIGHT. No part of olympus-xmp, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2022 The developers of olympus-xmp. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/raphaelcohn/olympus-xmp/master/COPYRIGHT.


/// IPTC world region.
///
/// One of the values listed in <https://cv.iptc.org/newscodes/worldregion/>.
#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub enum IptcWorldRegion
{
	#[allow(missing_docs)]
	World,
	
	#[allow(missing_docs)]
	Africa,
	
	#[allow(missing_docs)]
	SouthAmerica,
	
	#[allow(missing_docs)]
	Oceania,
	
	#[allow(missing_docs)]
	NorthAmerica,
	
	#[allow(missing_docs)]
	Asia,
	
	#[allow(missing_docs)]
	Europe,
	
	#[allow(missing_docs)]
	Antarctica,
}

impl<'a> XmpAttributeValue<'a> for IptcWorldRegion
{
	type Error = UnknownStringVariantParseError;
	
	#[inline(always)]
	fn parse(raw: &'a str) -> Result<Self, Self::Error>
	{
		use IptcWorldRegion::*;
		
		match raw
		{
			"World" => Ok(World),
			
			"Africa" => Ok(Africa),
			
			"South America" => Ok(SouthAmerica),
			
			"Oceania" => Ok(Oceania),
			
			"North America" => Ok(NorthAmerica),
			
			"Asia" => Ok(Asia),
			
			"Europe" => Ok(Europe),
			
			"Antarctica" => Ok(Antarctica),
			
			_ => Err(UnknownStringVariantParseError::from(raw)),
		}
	}
	
	#[inline(always)]
	fn into_xmp_attribute_value_parse_error(error: Self::Error) -> XmpAttributeValueParseError
	{
		XmpAttributeValueParseError::IptcWorldRegion(error)
	}
}
