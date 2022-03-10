// This file is part of olympus-xmp. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/raphaelcohn/olympus-xmp/master/COPYRIGHT. No part of olympus-xmp, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2022 The developers of olympus-xmp. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/raphaelcohn/olympus-xmp/master/COPYRIGHT.


/// PLUS model release status.
#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub enum PlusModelReleaseStatus
{
	/// None.
	///
	/// `http://ns.useplus.org/ldf/vocab/MR-NON`.
	NoReleases,
	
	/// Not Applicable.
	///
	/// `http://ns.useplus.org/ldf/vocab/MR-NAP`.
	NotApplicable,
	
	/// Unlimited Property Releases.
	///
	/// `http://ns.useplus.org/ldf/vocab/MR-UMR`.
	Unlimited,
	
	/// Limited or Incomplete Property Releases.
	///
	/// `http://ns.useplus.org/ldf/vocab/MR-LMR`.
	LimitedOrIncomplete,
}

impl<'a> XmpAttributeValue<'a> for PlusModelReleaseStatus
{
	type Error = UnknownStringVariantParseError;
	
	#[inline(always)]
	fn parse(raw: &'a str) -> Result<Self, Self::Error>
	{
		let suffix = UnknownStringVariantParseError::parse_prefixed_value_returning_suffix::<"http://ns.useplus.org/ldf/vocab/MR-">(raw)?;
		use PlusModelReleaseStatus::*;
		match suffix
		{
			"NON" => Ok(NoReleases),
			
			"NAP" => Ok(NotApplicable),
			
			"UMR" => Ok(Unlimited),
			
			"LMR" => Ok(LimitedOrIncomplete),
			
			_ => Err(UnknownStringVariantParseError::from(raw)),
		}
	}
	
	#[inline(always)]
	fn into_xmp_attribute_value_parse_error(error: Self::Error) -> XmpAttributeValueParseError
	{
		XmpAttributeValueParseError::PlusModelReleaseStatus(error)
	}
}
