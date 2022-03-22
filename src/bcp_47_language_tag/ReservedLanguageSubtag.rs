// This file is part of olympus-xmp. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/raphaelcohn/olympus-xmp/master/COPYRIGHT. No part of olympus-xmp, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2022 The developers of olympus-xmp. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/raphaelcohn/olympus-xmp/master/COPYRIGHT.


#[allow(missing_docs)]
#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub struct ReservedLanguageSubtag([Alpha; 4]);

impl ReservedLanguageSubtag
{
	#[inline(always)]
	fn parse(subtag: &[u8]) -> Result<Language, LanguageSubtagParseError>
	{
		const length: usize = 4;
		debug_assert_eq!(subtag.len(), length);
		Alpha::validate_and_convert_array::<_, _, _, _, length>(subtag, |alpha_array| Language::Reserved(Self(alpha_array)), LanguageSubtagParseError::InvalidAlpha)
	}
}
