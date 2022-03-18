// This file is part of olympus-xmp. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/raphaelcohn/olympus-xmp/master/COPYRIGHT. No part of olympus-xmp, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2022 The developers of olympus-xmp. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/raphaelcohn/olympus-xmp/master/COPYRIGHT.


/// As defined by [BCP 47](https://www.rfc-editor.org/rfc/bcp/bcp47.txt).
/// And RFC 5646.
/// <https://en.wikipedia.org/wiki/IETF_language_tag> is helpful.
/// <https://www.iana.org/assignments/language-subtag-registry/language-subtag-registry> is official.
#[derive(Debug, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub enum Bcp47LanguageTag
{
	#[allow(missing_docs)]
	Normal(Normal),
	
	#[allow(missing_docs)]
	PrivateUse(PrivateUse),

	#[allow(missing_docs)]
	Grandfathered(Grandfathered),
}

impl Bcp47LanguageTag
{
	#[allow(missing_docs)]
	#[inline(always)]
	pub fn parse(language_tag: &str) -> Result<Self, Bcp47LanguageTagParseError>
	{
		parse_bcp47_language_tag(language_tag)
	}
}
