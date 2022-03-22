// This file is part of olympus-xmp. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/raphaelcohn/olympus-xmp/master/COPYRIGHT. No part of olympus-xmp, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2022 The developers of olympus-xmp. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/raphaelcohn/olympus-xmp/master/COPYRIGHT.


#[allow(missing_docs)]
#[derive(Debug, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub struct VariantRecord
{
	prefix: Vec<String>,
	
	preferred_subtag: Option<String>,
}

impl ParseRecord for VariantRecord
{
	type Key = String;
	
	#[inline(always)]
	fn parse_key(subtag: String) -> Result<Self::Key, KeyParseError>
	{
		Ok(subtag)
	}
	
	#[inline(always)]
	fn parse(preferred_value: Option<String>, prefix: Vec<String>, suppress_script: Option<String>, macrolanguage: Option<String>, scope: Option<Scope>) -> Result<Self, RecordParseError>
	{
		use FieldNotPermittedError::*;
		
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
				prefix,
				
				preferred_subtag: preferred_value,
			}
		)
	}
}

impl VariantRecord
{
	#[allow(missing_docs)]
	#[inline(always)]
	pub fn prefix(&self) -> &[String]
	{
		&self.prefix
	}
	
	#[allow(missing_docs)]
	#[inline(always)]
	pub fn preferred_subtag(&self) -> Option<&str>
	{
		self.preferred_subtag.as_ref().map(|value| value.as_str())
	}
}
