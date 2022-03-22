// This file is part of olympus-xmp. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/raphaelcohn/olympus-xmp/master/COPYRIGHT. No part of olympus-xmp, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2022 The developers of olympus-xmp. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/raphaelcohn/olympus-xmp/master/COPYRIGHT.


#[allow(missing_docs)]
#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub enum RegisteredLanguageSubtag
{
	Alpha5([Alpha; 5]),
	
	Alpha6([Alpha; 6]),
	
	Alpha7([Alpha; 7]),
	
	Alpha8([Alpha; 8]),
}

impl RegisteredLanguageSubtag
{
	#[inline(always)]
	fn parse<OkConstructor: FnOnce([Alpha; length]) -> Self, const length: usize>(subtag: &[u8], ok: OkConstructor) -> Result<Language, LanguageSubtagParseError>
	{
		const InclusiveMinimumLength: usize = 5;
		const InclusiveMaximumLength: usize = 8;
		debug_assert!(length >= InclusiveMinimumLength);
		debug_assert!(length <= InclusiveMaximumLength);
		debug_assert!(subtag.len() >= InclusiveMinimumLength);
		debug_assert!(subtag.len() <= InclusiveMaximumLength);
		
		Alpha::validate_and_convert_array::<_, _, _, _, length>(subtag, |alpha_array| Language::Registered(ok(alpha_array)), LanguageSubtagParseError::InvalidAlpha)
	}
}
