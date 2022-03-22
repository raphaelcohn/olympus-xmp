// This file is part of olympus-xmp. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/raphaelcohn/olympus-xmp/master/COPYRIGHT. No part of olympus-xmp, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2022 The developers of olympus-xmp. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/raphaelcohn/olympus-xmp/master/COPYRIGHT.


#[allow(missing_docs)]
#[derive(Default, Debug, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub struct GrandfatheredRecord
{
	extended_language_range: Option<String>,
}

impl ParseRecord for GrandfatheredRecord
{
	type Key = String;
	
	#[inline(always)]
	fn parse_key(tag: String) -> Result<Self::Key, KeyParseError>
	{
		Ok(tag)
	}
	
	#[inline(always)]
	fn parse(preferred_value: Option<String>, prefix: Vec<String>, suppress_script: Option<String>, macrolanguage: Option<String>, scope: Option<Scope>) -> Result<Self, RecordParseError>
	{
		use FieldNotPermittedError::*;
		
		if !prefix.is_empty()
		{
			Err(PrefixNotPermittedInThisRecordType)?
		}
		if suppress_script.is_some()
		{
			Err(SuppressScriptNotPermittedInThisRecordType)?
		}
		if macrolanguage.is_some()
		{
			Err(MacrolanguageNotPermittedInThisRecordType)?
		}
		if scope.is_some()
		{
			Err(ScopeNotPermittedInThisRecordType)?
		}
		
		Ok
		(
			Self
			{
				extended_language_range: preferred_value,
			}
		)
	}
}

impl GrandfatheredRecord
{
	#[allow(missing_docs)]
	#[inline(always)]
	pub fn extended_language_range(&self) -> Option<&str>
	{
		self.extended_language_range.as_ref().map(|value| value.as_str())
	}
}
