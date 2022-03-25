// This file is part of olympus-xmp. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/raphaelcohn/olympus-xmp/master/COPYRIGHT. No part of olympus-xmp, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2022 The developers of olympus-xmp. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/raphaelcohn/olympus-xmp/master/COPYRIGHT.


#[inline(always)]
const fn letter_to_number_unchecked<const index: u8, const length: usize>(bytes: &[u8; length]) -> u16
{
	if cfg!(debug_assertions)
	{
		if index as usize >= length
		{
			panic!("index must be less than length")
		}
	}
	
	if cfg!(debug_assertions)
	{
		if length != 2 && length != 3
		{
			panic!("length must be 2 or 3")
		}
	}
	
	let letter = bytes[index as usize];
	if cfg!(debug_assertions)
	{
		if letter < A || letter > Z
		{
			panic!("letter must be A to Z inclusive")
		}
	}
	
	letter_to_number_scaled::<index>(letter)
}
