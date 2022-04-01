// This file is part of olympus-xmp. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/raphaelcohn/olympus-xmp/master/COPYRIGHT. No part of olympus-xmp, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2022 The developers of olympus-xmp. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/raphaelcohn/olympus-xmp/master/COPYRIGHT.


#[inline(always)]
const fn validate_static_abbreviation<const Length: usize>(abbreviation: StaticAbbreviation) -> StaticAbbreviation
{
	let length = abbreviation.len();
	if length == 0
	{
		panic!("Is empty")
	}
	if length > Length
	{
		panic!("Is longer than Length")
	}
	
	const Comma: u8 = b',';
	let mut index = 0;
	while index != length
	{
		match abbreviation[index]
		{
			A ..= Z => (),
			
			Space | Comma | Hyphen | Period => (),
			
			_ => panic!("Contains invalid byte")
		}
		index += 1;
	}
	
	abbreviation
}
