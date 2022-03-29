// This file is part of olympus-xmp. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/raphaelcohn/olympus-xmp/master/COPYRIGHT. No part of olympus-xmp, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2022 The developers of olympus-xmp. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/raphaelcohn/olympus-xmp/master/COPYRIGHT.


#[inline(always)]
const fn revision_4(m49_code: &'static [u8; 3], twelve_character_abbreviation: &'static [u8]) -> (M49Code, TwelveCharacterAbbreviation)
{
	(M49Code::from(m49_code), TwelveCharacterAbbreviation::new(twelve_character_abbreviation))
}
