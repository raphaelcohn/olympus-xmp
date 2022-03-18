// This file is part of olympus-xmp. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/raphaelcohn/olympus-xmp/master/COPYRIGHT. No part of olympus-xmp, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2022 The developers of olympus-xmp. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/raphaelcohn/olympus-xmp/master/COPYRIGHT.


#[allow(missing_docs)]
#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub struct IanaRegisteredIso639Alpha2Code([Alpha; 2]);

impl IanaRegisteredIso639Alpha2Code
{
	#[inline(always)]
	fn parse(first_subtag: &[u8]) -> Result<Self, LanguageFirstSubtagParseError>
	{
		Alpha::validate_alpha_to_lower_case::<_, _, _, _, 2>(first_subtag, Self, FirstSubtagLengthIsTwoToEightButInvalidAlpha)
	}
}
