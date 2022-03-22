// This file is part of olympus-xmp. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/raphaelcohn/olympus-xmp/master/COPYRIGHT. No part of olympus-xmp, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2022 The developers of olympus-xmp. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/raphaelcohn/olympus-xmp/master/COPYRIGHT.


#[allow(missing_docs)]
#[derive(Debug, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub struct ScriptRecord
{
	preferred_subtag: Option<String>,
}

impl ParseRecord for ScriptRecord
{
	type Key = [u8; 4];
	
	#[inline(always)]
	fn parse_key_range(inclusive_from: &str, inclusive_to: &str) -> Result<&'static [Self::Key], TagOrSubtagRangeError>
	{
		static PrivateUseRange: [[u8; 4]; 50] =
		[
			[Q, a, a, a],
			[Q, a, a, b],
			[Q, a, a, c],
			[Q, a, a, d],
			[Q, a, a, e],
			[Q, a, a, f],
			[Q, a, a, g],
			[Q, a, a, h],
			[Q, a, a, i],
			[Q, a, a, j],
			[Q, a, a, k],
			[Q, a, a, l],
			[Q, a, a, m],
			[Q, a, a, n],
			[Q, a, a, o],
			[Q, a, a, p],
			[Q, a, a, q],
			[Q, a, a, r],
			[Q, a, a, s],
			[Q, a, a, t],
			[Q, a, a, u],
			[Q, a, a, v],
			[Q, a, a, w],
			[Q, a, a, x],
			[Q, a, a, y],
			[Q, a, a, z],
			
			[Q, a, b, a],
			[Q, a, b, b],
			[Q, a, b, c],
			[Q, a, b, d],
			[Q, a, b, e],
			[Q, a, b, f],
			[Q, a, b, g],
			[Q, a, b, h],
			[Q, a, b, i],
			[Q, a, b, j],
			[Q, a, b, k],
			[Q, a, b, l],
			[Q, a, b, m],
			[Q, a, b, n],
			[Q, a, b, o],
			[Q, a, b, p],
			[Q, a, b, q],
			[Q, a, b, r],
			[Q, a, b, s],
			[Q, a, b, t],
			[Q, a, b, u],
			[Q, a, b, v],
			[Q, a, b, w],
			[Q, a, b, x],
		];
		
		match (inclusive_from, inclusive_to)
		{
			("Qaaa", "Qabx") => Ok(&PrivateUseRange),
			
			_ => Err(TagOrSubtagRangeError::TypeDoesNotSupportSpecificRange { inclusive_from: inclusive_from.to_string(), inclusive_to: inclusive_from.to_string() }),
		}
	}
	
	#[inline(always)]
	fn parse_key(subtag: String) -> Result<Self::Key, KeyParseError>
	{
		const Length: usize = 4;
		Self::validate_length::<Length>(&subtag)?;
		Self::validate_is_upper_case_alpha(&subtag.as_str()[ .. 1])?;
		Self::validate_is_lower_case_alpha(&subtag.as_str()[1 .. ])?;
		Ok(Self::copy_to_array::<Length>(&subtag))
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
				preferred_subtag: preferred_value,
			}
		)
	}
}

impl ScriptRecord
{
	#[allow(missing_docs)]
	#[inline(always)]
	pub fn preferred_subtag(&self) -> Option<&str>
	{
		self.preferred_subtag.as_ref().map(|value| value.as_str())
	}
}
