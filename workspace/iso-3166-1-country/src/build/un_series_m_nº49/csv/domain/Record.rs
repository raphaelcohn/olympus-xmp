// This file is part of olympus-xmp. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/raphaelcohn/olympus-xmp/master/COPYRIGHT. No part of olympus-xmp, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2022 The developers of olympus-xmp. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/raphaelcohn/olympus-xmp/master/COPYRIGHT.


#[derive(Debug, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub(super) struct Record
{
	pub(super) global: NameAndM49Code,
	
	pub(super) region: Option<Region>,
	
	pub(super) country: NameAndM49Code,
	
	pub(super) iso_3166_1_alpha2_code: Iso3166Dash1Alpha2Code,
	
	pub(super) iso_3166_1_alpha3_code: Option<Iso3166Dash1Alpha3Code>,
	
	pub(super) developing: Developing,
}

impl Record
{
	#[inline(always)]
	pub(super) fn extant_country(&self, m49_code: M49Code, names: Names) -> Country
	{
		Country
		{
			names,
		
			iso_3166_1_alpha2_code: self.iso_3166_1_alpha2_code,
			
			iso_3166_1_alpha3_code: self.iso_3166_1_alpha3_code,
		
			developing: self.developing,
		
			replacements: Country::NoReplacements,
		}
	}
}
