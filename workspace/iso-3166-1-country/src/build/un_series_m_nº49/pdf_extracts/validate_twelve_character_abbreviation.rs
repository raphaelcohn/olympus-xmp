// This file is part of olympus-xmp. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/raphaelcohn/olympus-xmp/master/COPYRIGHT. No part of olympus-xmp, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2022 The developers of olympus-xmp. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/raphaelcohn/olympus-xmp/master/COPYRIGHT.


#[inline(always)]
pub(super) const fn validate_twelve_character_abbreviation(twelve_character_abbreviation: &'static [u8]) -> &'static [u8]
{
	let length = twelve_character_abbreviation.len();
	if length == 0
	{
		panic!("Is empty")
	}
	if length > 12
	{
		panic!("Is longer than 12")
	}
	
	const Comma: u8 = b',';
	let mut index = 0;
	while index != length
	{
		match twelve_character_abbreviation[index]
		{
			A ..= Z => (),
			
			Space | Comma | Hyphen | Period => (),
			
			_ => panic!("Contains invalid byte")
		}
		index += 1;
	}
	
	twelve_character_abbreviation
}
