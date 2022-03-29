// This file is part of olympus-xmp. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/raphaelcohn/olympus-xmp/master/COPYRIGHT. No part of olympus-xmp, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2022 The developers of olympus-xmp. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/raphaelcohn/olympus-xmp/master/COPYRIGHT.


#[inline(always)]
const fn revision_2_regions_and_groupings_abbreviations(code: &'static [u8; 3], english_name: &'static str, legacy_english_eight_character_abbreviation: &'static [u8], english_twelve_character_abbreviation: &'static [u8]) -> (M49Code, &'static str, (LegacyEightCharacterAbbreviation, TwelveCharacterAbbreviation))
{
	(M49Code::from(m49_code), english_name, (LegacyEightCharacterAbbreviation::new(legacy_english_eight_character_abbreviation), TwelveCharacterAbbreviation::new(english_twelve_character_abbreviation)))
}
