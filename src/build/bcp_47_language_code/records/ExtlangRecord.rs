// This file is part of olympus-xmp. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/raphaelcohn/olympus-xmp/master/COPYRIGHT. No part of olympus-xmp, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2022 The developers of olympus-xmp. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/raphaelcohn/olympus-xmp/master/COPYRIGHT.


#[derive(Default, Debug, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
struct ExtlangRecord
{
	extended_language_range: Option<String>,
	
	prefix: String,
	
	suppress_script: Option<[u8; 4]>,
	
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
		const Length: usize = 3;
		Self::validate_length::<Length>(&subtag)?;
		Self::subtag_to_byte_array::<_, Length>(&subtag, Self::validate_is_lower_case_alpha)
	}
	
	#[inline(always)]
	fn parse(preferred_value: Option<String>, mut prefix: Vec<String>, suppress_script: Option<String>, macrolanguage: Option<String>, scope: Option<Scope>) -> Result<Self, RecordParseError>
	{
		Ok
		(
			Self
			{
				extended_language_range: preferred_value,
				
				prefix:
				{
					if prefix.len() != 1
					{
						Err(FieldNotPermittedError::PrefixMustHaveExactlyOneValueForExtlangRecord)?
					}
					let pop = prefix.pop();
					unsafe { pop.unwrap_unchecked() }
				},
				
				suppress_script: match suppress_script
				{
					None => None,
					
					Some(suppress_script) => Some(ScriptRecord::parse_key(suppress_script)?),
				},
				
				macrolanguage,
				
				scope: scope.unwrap_or_default(),
			}
		)
	}
}
