// This file is part of olympus-xmp. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/raphaelcohn/olympus-xmp/master/COPYRIGHT. No part of olympus-xmp, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2022 The developers of olympus-xmp. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/raphaelcohn/olympus-xmp/master/COPYRIGHT.


/// `(1*8alphanum)`.
#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub enum PrivateUsePortion
{
	#[allow(missing_docs)]
	Alphanumeric1([Alphanumeric; 1]),
	
	#[allow(missing_docs)]
	Alphanumeric2([Alphanumeric; 2]),
	
	#[allow(missing_docs)]
	Alphanumeric3([Alphanumeric; 3]),
	
	#[allow(missing_docs)]
	Alphanumeric4([Alphanumeric; 4]),
	
	#[allow(missing_docs)]
	Alphanumeric5([Alphanumeric; 5]),
	
	#[allow(missing_docs)]
	Alphanumeric6([Alphanumeric; 6]),
	
	#[allow(missing_docs)]
	Alphanumeric7([Alphanumeric; 7]),
	
	#[allow(missing_docs)]
	Alphanumeric8([Alphanumeric; 8]),
}

impl PrivateUsePortion
{
	#[inline(always)]
	fn parse(subtag: &[u8]) -> Result<Self, PrivateUseSubtagsParseError>
	{
		use PrivateUseSubtagsParseError::*;
		use PrivateUsePortion::*;
		
		macro_rules! parse_n
		{
			($subtag: ident, $n: expr, $n_enum: ident) =>
			{
				Self::parse_n::<_, $n>(subtag, $n_enum)
			}
		}
		
		match_subtag_length!
		{
			subtag,
			
			parse_n!(subtag, 1, Alphanumeric1),
			
			parse_n!(subtag, 2, Alphanumeric2),
			
			parse_n!(subtag, 3, Alphanumeric3),
			
			parse_n!(subtag, 4, Alphanumeric4),
			
			parse_n!(subtag, 5, Alphanumeric5),
			
			parse_n!(subtag, 6, Alphanumeric6),
			
			parse_n!(subtag, 7, Alphanumeric7),
			
			parse_n!(subtag, 8, Alphanumeric8)
		}
	}
	
	#[inline(always)]
	fn parse_n<OkConstructor: FnOnce([Alphanumeric; length]) -> PrivateUsePortion, const length: usize>(subtag: &[u8], ok: OkConstructor) -> Result<Self, PrivateUseSubtagsParseError>
	{
		debug_assert_eq!(length, subtag.len());
		
		Alphanumeric::validate_and_convert_array::<_, _, _, _,length>(subtag, ok, PrivateUseSubtagsParseError::InvalidAlphanumeric)
	}
}
