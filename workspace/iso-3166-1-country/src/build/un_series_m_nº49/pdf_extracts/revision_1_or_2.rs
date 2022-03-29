// This file is part of olympus-xmp. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/raphaelcohn/olympus-xmp/master/COPYRIGHT. No part of olympus-xmp, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2022 The developers of olympus-xmp. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/raphaelcohn/olympus-xmp/master/COPYRIGHT.


#[inline(always)]
const fn revision_1_or_2(code: &'static [u8; 3], name: &'static str, legacy_english_eight_character_abbreviation: &'static [u8], english_twelve_character_abbreviation: &'static [u8], iso_3166_alpha_codes: Option<(&'static [u8; 2], &'static [u8; 3])>) -> (M49Code, &'static str, (LegacyEightCharacterAbbreviation, TwelveCharacterAbbreviation), Option<(Iso3166Dash1Alpha2Code, Iso3166Dash1Alpha3Code)>)
{
	(M49Code::from(m49_code), name, (LegacyEightCharacterAbbreviation::new(legacy_english_eight_character_abbreviation), TwelveCharacterAbbreviation::new(english_twelve_character_abbreviation)), iso_3166_alpha_codes.map(|(iso_3166_alpha_2_code, iso_3166_alpha_3_code)| (Iso3166Dash1Alpha2Code::from(iso_3166_alpha_2_code), Iso3166Dash1Alpha3Code::from(iso_3166_alpha_3_code))))
}
