// This file is part of olympus-xmp. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/raphaelcohn/olympus-xmp/master/COPYRIGHT. No part of olympus-xmp, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2022 The developers of olympus-xmp. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/raphaelcohn/olympus-xmp/master/COPYRIGHT.


#[derive(Debug, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub(super) struct Country
{
	names: Names,
	
	abbreviations: Abbreviations,
	
	iso_3166_1_alpha_code: Iso3166Dash1AlphaCode,
	
	developing: Developing,
	
	replacements: &'static [M49Code],
}

impl Country
{
	const NoReplacements: &'static [M49Code] = &[];
	
	#[inline(always)]
	fn extant(names: Names, abbreviations: Option<TwelveCharacterAbbreviation>, iso_3166_1_alpha2_code: Iso3166Dash1Alpha2Code, iso_3166_1_alpha3_code: Option<Iso3166Dash1Alpha3Code>, developing: Developing) -> Self
	{
		use Abbreviations::*;
		use Iso3166Dash1AlphaCode::*;
		
		Self
		{
			names,
			
			abbreviations: match abbreviations
			{
				None => AbsentRevision3Onwards,
				
				Some(english_twelve_character_abbreviation) => Revision3Onwards
				{
					english_twelve_character_abbreviation
				},
			},
			
			iso_3166_1_alpha_code: match iso_3166_1_alpha3_code
			{
				None => Alpha2Only(iso_3166_1_alpha2_code),
				
				Some(iso_3166_1_alpha3_code) => Both
				{
					alpha2: iso_3166_1_alpha2_code,
					
					alpha3: iso_3166_1_alpha3_code,
				}
			},
			
			developing,
			
			replacements: Self::NoReplacements
		}
	}
}
