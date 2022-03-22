// This file is part of olympus-xmp. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/raphaelcohn/olympus-xmp/master/COPYRIGHT. No part of olympus-xmp, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2022 The developers of olympus-xmp. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/raphaelcohn/olympus-xmp/master/COPYRIGHT.


/// Parses the contents of the shipped language-subtag-registry file and then caches the result.
#[inline(always)]
pub fn language_subtag_registry() -> Result<&'static Records, LanguageSubtagRegistryFileContentsParseError>
{
	static Database: SyncOnceCell<Records> = SyncOnceCell::new();
	
	Database.get_or_try_init(|| parse_language_subtag_registry(include_str!("language-subtag-registry")))
}
