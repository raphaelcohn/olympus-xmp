// This file is part of olympus-xmp. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/raphaelcohn/olympus-xmp/master/COPYRIGHT. No part of olympus-xmp, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2022 The developers of olympus-xmp. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/raphaelcohn/olympus-xmp/master/COPYRIGHT.


#[allow(missing_docs)]
#[derive(Debug, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub struct RegionRecord
{
	preferred_subtag: Option<String>,
}

impl ParseRecord for RegionRecord
{
	type Key = RegionSubtag;
	
	#[inline(always)]
	fn parse_key_range(inclusive_from: &str, inclusive_to: &str) -> Result<&'static [Self::Key], TagOrSubtagRangeError>
	{
		use RegionSubtag::TwoUpperCaseAlpha;
		
		static RegionUseRange1: [RegionSubtag; 14] =
		[
			TwoUpperCaseAlpha([Q, M]),
			TwoUpperCaseAlpha([Q, N]),
			TwoUpperCaseAlpha([Q, O]),
			TwoUpperCaseAlpha([Q, P]),
			TwoUpperCaseAlpha([Q, Q]),
			TwoUpperCaseAlpha([Q, R]),
			TwoUpperCaseAlpha([Q, S]),
			TwoUpperCaseAlpha([Q, T]),
			TwoUpperCaseAlpha([Q, U]),
			TwoUpperCaseAlpha([Q, V]),
			TwoUpperCaseAlpha([Q, W]),
			TwoUpperCaseAlpha([Q, X]),
			TwoUpperCaseAlpha([Q, Y]),
			TwoUpperCaseAlpha([Q, Z]),
		];
		
		static RegionUseRange2: [RegionSubtag; 26] =
		[
			TwoUpperCaseAlpha([X, A]),
			TwoUpperCaseAlpha([X, B]),
			TwoUpperCaseAlpha([X, C]),
			TwoUpperCaseAlpha([X, D]),
			TwoUpperCaseAlpha([X, E]),
			TwoUpperCaseAlpha([X, F]),
			TwoUpperCaseAlpha([X, G]),
			TwoUpperCaseAlpha([X, H]),
			TwoUpperCaseAlpha([X, I]),
			TwoUpperCaseAlpha([X, J]),
			TwoUpperCaseAlpha([X, K]),
			TwoUpperCaseAlpha([X, L]),
			TwoUpperCaseAlpha([X, M]),
			TwoUpperCaseAlpha([X, N]),
			TwoUpperCaseAlpha([X, O]),
			TwoUpperCaseAlpha([X, P]),
			TwoUpperCaseAlpha([X, Q]),
			TwoUpperCaseAlpha([X, R]),
			TwoUpperCaseAlpha([X, S]),
			TwoUpperCaseAlpha([X, T]),
			TwoUpperCaseAlpha([X, U]),
			TwoUpperCaseAlpha([X, V]),
			TwoUpperCaseAlpha([X, W]),
			TwoUpperCaseAlpha([X, X]),
			TwoUpperCaseAlpha([X, Y]),
			TwoUpperCaseAlpha([X, Z]),
		];
		
		match (inclusive_from, inclusive_to)
		{
			("QM", "QZ") => Ok(&RegionUseRange1),
			
			("XA", "XZ") => Ok(&RegionUseRange2),
			
			_ => Err(TagOrSubtagRangeError::TypeDoesNotSupportSpecificRange { inclusive_from: inclusive_from.to_string(), inclusive_to: inclusive_from.to_string() }),
		}
	}
	
	#[inline(always)]
	fn parse_key(subtag: String) -> Result<Self::Key, KeyParseError>
	{
		use RegionSubtag::*;
		
		match subtag.len()
		{
			2 => Ok(TwoUpperCaseAlpha(Self::subtag_to_byte_array::<_, 2>(&subtag, Self::validate_is_upper_case_alpha)?)),
			
			3 => Ok(ThreeDigit(Self::subtag_to_byte_array::<_, 3>(&subtag, Self::validate_is_digit)?)),
			
			length @ _ => Err(KeyParseError::SubtagInvalidLength { length, minimum: 2, maximum: 3, subtag: subtag.to_string() }),
		}
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

impl RegionRecord
{
	#[allow(missing_docs)]
	#[inline(always)]
	pub fn preferred_subtag(&self) -> Option<&str>
	{
		self.preferred_subtag.as_ref().map(|value| value.as_str())
	}
}
