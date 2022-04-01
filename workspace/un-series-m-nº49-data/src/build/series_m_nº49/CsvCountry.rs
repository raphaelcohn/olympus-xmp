// This file is part of olympus-xmp. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/raphaelcohn/olympus-xmp/master/COPYRIGHT. No part of olympus-xmp, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2022 The developers of olympus-xmp. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/raphaelcohn/olympus-xmp/master/COPYRIGHT.


#[derive(Debug, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub(super) struct CsvCountry
{
	names_in_united_nations_official_languages: NamesInUnitedNationsOfficialLanguages,
	
	iso_3166_1_alpha2_code: Iso3166Dash1Alpha2Code,
	
	iso_3166_1_alpha3_code: Option<Iso3166Dash1Alpha3Code>,
	
	developing: Developing,
}

impl AsMut<NamesInUnitedNationsOfficialLanguages> for CsvCountry
{
	#[inline(always)]
	fn as_mut(&mut self) -> &mut NamesInUnitedNationsOfficialLanguages
	{
		&mut self.names_in_united_nations_official_languages
	}
}
