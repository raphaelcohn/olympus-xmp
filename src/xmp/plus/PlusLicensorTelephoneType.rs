// This file is part of olympus-xmp. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/raphaelcohn/olympus-xmp/master/COPYRIGHT. No part of olympus-xmp, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2022 The developers of olympus-xmp. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/raphaelcohn/olympus-xmp/master/COPYRIGHT.


/// PLUS licensor telephone type.
#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub enum PlusLicensorTelephoneType
{
	/// Work.
	///
	/// `http://ns.useplus.org/ldf/vocab/work`.
	work,
	
	/// Not Applicable.
	///
	/// `http://ns.useplus.org/ldf/vocab/cell`.
	cell,
	
	/// Unlimited Property Releases.
	///
	/// `http://ns.useplus.org/ldf/vocab/fax`.
	fax,
	
	/// Unlimited Property Releases.
	///
	/// `http://ns.useplus.org/ldf/vocab/home`.
	home,
	
	/// Limited or Incomplete Property Releases.
	///
	/// `http://ns.useplus.org/ldf/vocab/pager`.
	pager,
}

impl<'a> XmpAttributeValue<'a> for PlusLicensorTelephoneType
{
	type Error = UnknownStringVariantParseError;
	
	#[inline(always)]
	fn parse(raw: &'a str) -> Result<Self, Self::Error>
	{
		let suffix = UnknownStringVariantParseError::parse_prefixed_value_returning_suffix::<"http://ns.useplus.org/ldf/vocab/">(raw)?;
		use PlusLicensorTelephoneType::*;
		match suffix
		{
			"work" => Ok(work),
			
			"cell" => Ok(cell),
			
			"fax" => Ok(fax),
			
			"home" => Ok(home),
			
			"pager" => Ok(pager),
			
			_ => Err(UnknownStringVariantParseError::from(raw)),
		}
	}
	
	#[inline(always)]
	fn into_xmp_attribute_value_parse_error(error: Self::Error) -> XmpAttributeValueParseError
	{
		XmpAttributeValueParseError::PlusLicensorTelephoneType(error)
	}
}
