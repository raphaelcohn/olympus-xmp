// This file is part of olympus-xmp. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/raphaelcohn/olympus-xmp/master/COPYRIGHT. No part of olympus-xmp, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2022 The developers of olympus-xmp. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/raphaelcohn/olympus-xmp/master/COPYRIGHT.


#[inline(always)]
fn subsequent<const index: u8>(id: &[u8]) -> u8
{
	let subsequent = id.get_unchecked_value_safe(index);
	match subsequent
	{
		_0 ..= _9 => subsequent,
		
		_ => panic!("Subsequent value is not 0 to 9 but {} at index {}", subsequent, index)
	}
}
