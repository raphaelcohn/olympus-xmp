// This file is part of olympus-xmp. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/raphaelcohn/olympus-xmp/master/COPYRIGHT. No part of olympus-xmp, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2022 The developers of olympus-xmp. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/raphaelcohn/olympus-xmp/master/COPYRIGHT.


#[derive(Debug, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
struct RegionRecord
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
			TwoUpperCaseAlpha([b'Q', b'M']),
			TwoUpperCaseAlpha([b'Q', b'N']),
			TwoUpperCaseAlpha([b'Q', b'O']),
			TwoUpperCaseAlpha([b'Q', b'P']),
			TwoUpperCaseAlpha([b'Q', b'Q']),
			TwoUpperCaseAlpha([b'Q', b'R']),
			TwoUpperCaseAlpha([b'Q', b'S']),
			TwoUpperCaseAlpha([b'Q', b'T']),
			TwoUpperCaseAlpha([b'Q', b'U']),
			TwoUpperCaseAlpha([b'Q', b'V']),
			TwoUpperCaseAlpha([b'Q', b'W']),
			TwoUpperCaseAlpha([b'Q', b'X']),
			TwoUpperCaseAlpha([b'Q', b'Y']),
			TwoUpperCaseAlpha([b'Q', b'Z']),
		];
		
		static RegionUseRange2: [RegionSubtag; 26] =
		[
			TwoUpperCaseAlpha([b'X', b'A']),
			TwoUpperCaseAlpha([b'X', b'B']),
			TwoUpperCaseAlpha([b'X', b'C']),
			TwoUpperCaseAlpha([b'X', b'D']),
			TwoUpperCaseAlpha([b'X', b'E']),
			TwoUpperCaseAlpha([b'X', b'F']),
			TwoUpperCaseAlpha([b'X', b'G']),
			TwoUpperCaseAlpha([b'X', b'H']),
			TwoUpperCaseAlpha([b'X', b'I']),
			TwoUpperCaseAlpha([b'X', b'J']),
			TwoUpperCaseAlpha([b'X', b'K']),
			TwoUpperCaseAlpha([b'X', b'L']),
			TwoUpperCaseAlpha([b'X', b'M']),
			TwoUpperCaseAlpha([b'X', b'N']),
			TwoUpperCaseAlpha([b'X', b'O']),
			TwoUpperCaseAlpha([b'X', b'P']),
			TwoUpperCaseAlpha([b'X', b'Q']),
			TwoUpperCaseAlpha([b'X', b'R']),
			TwoUpperCaseAlpha([b'X', b'S']),
			TwoUpperCaseAlpha([b'X', b'T']),
			TwoUpperCaseAlpha([b'X', b'U']),
			TwoUpperCaseAlpha([b'X', b'V']),
			TwoUpperCaseAlpha([b'X', b'W']),
			TwoUpperCaseAlpha([b'X', b'X']),
			TwoUpperCaseAlpha([b'X', b'Y']),
			TwoUpperCaseAlpha([b'X', b'Z']),
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
