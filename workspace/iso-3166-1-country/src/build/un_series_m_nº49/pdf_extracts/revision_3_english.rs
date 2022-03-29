// This file is part of olympus-xmp. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/raphaelcohn/olympus-xmp/master/COPYRIGHT. No part of olympus-xmp, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2022 The developers of olympus-xmp. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/raphaelcohn/olympus-xmp/master/COPYRIGHT.


#[inline(always)]
const fn revision_3_english(code: &'static [u8; 3], name: &'static str, english_twelve_character_abbreviation: Option<&'static [u8]>, iso_3166_alpha_3_code: Option<(&'static [u8; 3])>) -> (M49Code, &'static str, Option<TwelveCharacterAbbreviation>, Option<Iso3166Dash1Alpha3Code>)
{
	(M49Code::from(m49_code), name, english_twelve_character_abbreviation.map(TwelveCharacterAbbreviation::new), iso_3166_alpha_3_code.map(Iso3166Dash1Alpha3Code::from))
}
