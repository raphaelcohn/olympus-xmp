// This file is part of olympus-xmp. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/raphaelcohn/olympus-xmp/master/COPYRIGHT. No part of olympus-xmp, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2022 The developers of olympus-xmp. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/raphaelcohn/olympus-xmp/master/COPYRIGHT.


#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub(crate) enum Type
{
	extlang,
	
	grandfathered,
	
	language,
	
	redundant,
	
	region,
	
	script,
	
	variant,
}

impl FromStr for Type
{
	type Err = UnknownStringVariantError;
	
	#[inline(always)]
	fn from_str(field_body: &str) -> Result<Self, Self::Err>
	{
		use Type::*;
		
		match field_body
		{
			"extlang" => Ok(extlang),
			
			"grandfathered" => Ok(grandfathered),
			
			"language" => Ok(language),
			
			"redundant" => Ok(redundant),
			
			"region" => Ok(region),
			
			"script" => Ok(script),
			
			"variant" => Ok(variant),
			
			_ => Err(UnknownStringVariantError(field_body.to_string()))
		}
	}
}
