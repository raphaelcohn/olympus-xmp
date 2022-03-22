// This file is part of olympus-xmp. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/raphaelcohn/olympus-xmp/master/COPYRIGHT. No part of olympus-xmp, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2022 The developers of olympus-xmp. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/raphaelcohn/olympus-xmp/master/COPYRIGHT.


/// PLUS property release status.
#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub enum PlusPropertyReleaseStatus
{
	/// None.
	///
	/// `http://ns.useplus.org/ldf/vocab/PR-NON`.
	NoReleases,
	
	/// Not Applicable.
	///
	/// `http://ns.useplus.org/ldf/vocab/PR-NAP`.
	NotApplicable,
	
	/// Unlimited Property Releases.
	///
	/// `http://ns.useplus.org/ldf/vocab/PR-UPR`.
	Unlimited,
	
	/// Limited or Incomplete Property Releases.
	///
	/// `http://ns.useplus.org/ldf/vocab/PR-LPR`.
	LimitedOrIncomplete,
}

impl<'a> XmpAttributeValue<'a> for PlusPropertyReleaseStatus
{
	type Error = UnknownStringVariantParseError;
	
	#[inline(always)]
	fn parse(raw: &'a str) -> Result<Self, Self::Error>
	{
		let suffix = UnknownStringVariantParseError::parse_prefixed_value_returning_suffix::<"http://ns.useplus.org/ldf/vocab/PR-">(raw)?;
		use PlusPropertyReleaseStatus::*;
		match suffix
		{
			"NON" => Ok(NoReleases),
			
			"NAP" => Ok(NotApplicable),
			
			"UPR" => Ok(Unlimited),
			
			"LPR" => Ok(LimitedOrIncomplete),
			
			_ => Err(UnknownStringVariantParseError::from(raw)),
		}
	}
	
	#[inline(always)]
	fn into_xmp_attribute_value_parse_error(error: Self::Error) -> XmpAttributeValueParseError
	{
		XmpAttributeValueParseError::PlusPropertyReleaseStatus(error)
	}
}
