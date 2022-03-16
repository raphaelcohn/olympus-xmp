// This file is part of olympus-xmp. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/raphaelcohn/olympus-xmp/master/COPYRIGHT. No part of olympus-xmp, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2022 The developers of olympus-xmp. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/raphaelcohn/olympus-xmp/master/COPYRIGHT.


#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
struct ScriptRecord
{
}

impl ParseRecord for ScriptRecord
{
	type Key = [u8; 4];
	
	#[inline(always)]
	fn parse_key_range(inclusive_from: &str, inclusive_to: &str) -> Result<&'static [Self::Key], TagOrSubtagRangeError>
	{
		static PrivateUseRange: [[u8; 4]; 50] =
		[
			[b'Q', b'a', b'a', b'a'],
			[b'Q', b'a', b'a', b'b'],
			[b'Q', b'a', b'a', b'c'],
			[b'Q', b'a', b'a', b'd'],
			[b'Q', b'a', b'a', b'e'],
			[b'Q', b'a', b'a', b'f'],
			[b'Q', b'a', b'a', b'g'],
			[b'Q', b'a', b'a', b'h'],
			[b'Q', b'a', b'a', b'i'],
			[b'Q', b'a', b'a', b'j'],
			[b'Q', b'a', b'a', b'k'],
			[b'Q', b'a', b'a', b'l'],
			[b'Q', b'a', b'a', b'm'],
			[b'Q', b'a', b'a', b'n'],
			[b'Q', b'a', b'a', b'o'],
			[b'Q', b'a', b'a', b'p'],
			[b'Q', b'a', b'a', b'q'],
			[b'Q', b'a', b'a', b'r'],
			[b'Q', b'a', b'a', b's'],
			[b'Q', b'a', b'a', b't'],
			[b'Q', b'a', b'a', b'u'],
			[b'Q', b'a', b'a', b'v'],
			[b'Q', b'a', b'a', b'w'],
			[b'Q', b'a', b'a', b'x'],
			[b'Q', b'a', b'a', b'y'],
			[b'Q', b'a', b'a', b'z'],
			
			[b'Q', b'a', b'b', b'a'],
			[b'Q', b'a', b'b', b'b'],
			[b'Q', b'a', b'b', b'c'],
			[b'Q', b'a', b'b', b'd'],
			[b'Q', b'a', b'b', b'e'],
			[b'Q', b'a', b'b', b'f'],
			[b'Q', b'a', b'b', b'g'],
			[b'Q', b'a', b'b', b'h'],
			[b'Q', b'a', b'b', b'i'],
			[b'Q', b'a', b'b', b'j'],
			[b'Q', b'a', b'b', b'k'],
			[b'Q', b'a', b'b', b'l'],
			[b'Q', b'a', b'b', b'm'],
			[b'Q', b'a', b'b', b'n'],
			[b'Q', b'a', b'b', b'o'],
			[b'Q', b'a', b'b', b'p'],
			[b'Q', b'a', b'b', b'q'],
			[b'Q', b'a', b'b', b'r'],
			[b'Q', b'a', b'b', b's'],
			[b'Q', b'a', b'b', b't'],
			[b'Q', b'a', b'b', b'u'],
			[b'Q', b'a', b'b', b'v'],
			[b'Q', b'a', b'b', b'w'],
			[b'Q', b'a', b'b', b'x'],
		];
		
		match (inclusive_from, inclusive_to)
		{
			("Qaaa", "Qabx") => Ok(&PrivateUseRange),
			
			_ => Err(TagOrSubtagRangeError::TypeDoesNotSupportSpecificRange { inclusive_from: inclusive_from.to_string(), inclusive_to: inclusive_from.to_string() }),
		}
	}
	
	#[inline(always)]
	fn parse_key(tag_or_subtag: String) -> Result<Self::Key, KeyParseError>
	{
		unimplemented!()
	}
	
	#[inline(always)]
	fn parse(preferred_value: Option<String>, prefix: Vec<String>, suppress_script: Option<String>, macrolanguage: Option<String>, scope: Option<Scope>) -> Result<Self, RecordParseError>
	{
		unimplemented!()
	}
}
