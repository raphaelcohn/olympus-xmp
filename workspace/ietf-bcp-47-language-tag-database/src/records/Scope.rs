// This file is part of olympus-xmp. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/raphaelcohn/olympus-xmp/master/COPYRIGHT. No part of olympus-xmp, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2022 The developers of olympus-xmp. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/raphaelcohn/olympus-xmp/master/COPYRIGHT.


#[allow(missing_docs)]
#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub enum Scope
{
	collection,

	macrolanguage,

	private_use,

	special,

	individual_language,
}

impl Default for Scope
{
	#[inline(always)]
	fn default() -> Self
	{
		Scope::individual_language
	}
}

impl FromStr for Scope
{
	type Err = UnknownStringVariantError;
	
	#[inline(always)]
	fn from_str(field_body: &str) -> Result<Self, Self::Err>
	{
		use Scope::*;
		
		match field_body
		{
			"collection" => Ok(collection),
			
			"macrolanguage" => Ok(macrolanguage),
			
			"private-use" => Ok(private_use),
			
			"special" => Ok(special),
			
			_ => Err(UnknownStringVariantError(field_body.to_string()))
		}
	}
}
