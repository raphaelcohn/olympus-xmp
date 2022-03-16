// This file is part of olympus-xmp. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/raphaelcohn/olympus-xmp/master/COPYRIGHT. No part of olympus-xmp, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2022 The developers of olympus-xmp. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/raphaelcohn/olympus-xmp/master/COPYRIGHT.


pub(crate) fn parse_language_subtag_registry() -> Result<Records, LanguageSubtagRegistryFileParseError>
{
	use LanguageSubtagRegistryFileParseError::*;
	
	let buffer = open_our_module_file_path(relative_module_path!(), "language-subtag-registry", 1024).map_err(CouldNotOpenFile)?;
	Records::parse(PullEventParser::new(&buffer))
}
