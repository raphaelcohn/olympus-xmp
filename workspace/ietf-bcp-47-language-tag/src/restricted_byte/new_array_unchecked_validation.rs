// This file is part of olympus-xmp. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/raphaelcohn/olympus-xmp/master/COPYRIGHT. No part of olympus-xmp, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2022 The developers of olympus-xmp. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/raphaelcohn/olympus-xmp/master/COPYRIGHT.


#[inline(always)]
const fn new_array_unchecked_validation<RBC: ~const RestrictedByteConst, const length: usize>(value: &[u8; length])
{
	if cfg!(debug_assertions)
	{
		// TODO: Uses a while loop because for loops are not yet implemented for const functions.
		let mut index = 0;
		while index < length
		{
			let byte = value[index];
			if !RBC::validate_byte(byte)
			{
				panic!("Invalid byte")
			}
			index += 1;
		}
	}
}
