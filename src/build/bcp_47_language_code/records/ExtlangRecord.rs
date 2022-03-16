// This file is part of olympus-xmp. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/raphaelcohn/olympus-xmp/master/COPYRIGHT. No part of olympus-xmp, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2022 The developers of olympus-xmp. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/raphaelcohn/olympus-xmp/master/COPYRIGHT.


#[derive(Default, Debug, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
struct ExtlangRecord
{
	extended_language_range: Option<String>,
	
	prefix: Vec<String>,
	
	suppress_script: Option<String>,
	
	macrolanguage: Option<String>,
	
	scope: Scope,
}

impl ParseRecord for ExtlangRecord
{
	type Key = [u8; 3];
	
	/// According to BCP 47, page 5:-
	/// ```
	/// extlang       = 3ALPHA              ; selected ISO 639 codes
	///                 *2("-" 3ALPHA)      ; permanently reserved
	/// ```
	/// Here `extlang` is `subtag`.
	#[inline(always)]
	fn parse_key(subtag: String) -> Result<Self::Key, KeyParseError>
	{
		Self::subtag_to_byte_array::<3>(&subtag)
	}
	
	#[inline(always)]
	fn parse(preferred_value: Option<String>, prefix: Vec<String>, suppress_script: Option<String>, macrolanguage: Option<String>, scope: Option<Scope>) -> Result<Self, RecordParseError>
	{
		Ok
		(
			Self
			{
				extended_language_range: preferred_value,
				
				prefix,
				
				suppress_script,
				
				macrolanguage,
				
				scope: scope.unwrap_or_default(),
			}
		)
	}
}
