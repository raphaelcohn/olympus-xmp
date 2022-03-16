// This file is part of olympus-xmp. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/raphaelcohn/olympus-xmp/master/COPYRIGHT. No part of olympus-xmp, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2022 The developers of olympus-xmp. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/raphaelcohn/olympus-xmp/master/COPYRIGHT.


#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
struct RegionRecord
{
}

impl ParseRecord for RegionRecord
{
	type Key = [u8; 2];
	
	#[inline(always)]
	fn parse_key_range(inclusive_from: &str, inclusive_to: &str) -> Result<&'static [Self::Key], TagOrSubtagRangeError>
	{
		static RegionUseRange1: [[u8; 2]; 14] =
		[
			[b'Q', b'M'],
			[b'Q', b'N'],
			[b'Q', b'O'],
			[b'Q', b'P'],
			[b'Q', b'Q'],
			[b'Q', b'R'],
			[b'Q', b'S'],
			[b'Q', b'T'],
			[b'Q', b'U'],
			[b'Q', b'V'],
			[b'Q', b'W'],
			[b'Q', b'X'],
			[b'Q', b'Y'],
			[b'Q', b'Z'],
		];
		
		static RegionUseRange2: [[u8; 2]; 26] =
		[
			[b'X', b'A'],
			[b'X', b'B'],
			[b'X', b'C'],
			[b'X', b'D'],
			[b'X', b'E'],
			[b'X', b'F'],
			[b'X', b'G'],
			[b'X', b'H'],
			[b'X', b'I'],
			[b'X', b'J'],
			[b'X', b'K'],
			[b'X', b'L'],
			[b'X', b'M'],
			[b'X', b'N'],
			[b'X', b'O'],
			[b'X', b'P'],
			[b'X', b'Q'],
			[b'X', b'R'],
			[b'X', b'S'],
			[b'X', b'T'],
			[b'X', b'U'],
			[b'X', b'V'],
			[b'X', b'W'],
			[b'X', b'X'],
			[b'X', b'Y'],
			[b'X', b'Z'],
		];
		
		match (inclusive_from, inclusive_to)
		{
			("QM", "QZ") => Ok(&RegionUseRange1),
			
			("XA", "XZ") => Ok(&RegionUseRange2),
			
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
