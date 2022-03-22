// This file is part of olympus-xmp. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/raphaelcohn/olympus-xmp/master/COPYRIGHT. No part of olympus-xmp, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2022 The developers of olympus-xmp. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/raphaelcohn/olympus-xmp/master/COPYRIGHT.


/// Parses the contents of a language-subtag-registry file downloaded from <https://www.iana.org/assignments/language-subtag-registry/language-subtag-registry>.
///
/// Normally, you'd want to use `language_subtag_registry()` instead of this method.
#[inline(always)]
pub fn parse_language_subtag_registry(language_subtag_registry_file_contents: &str) -> Result<Records, LanguageSubtagRegistryFileContentsParseError>
{
	Records::parse(PullEventParser::new(language_subtag_registry_file_contents))
}
