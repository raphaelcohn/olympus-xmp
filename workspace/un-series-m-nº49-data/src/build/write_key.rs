// This file is part of olympus-xmp. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/raphaelcohn/olympus-xmp/master/COPYRIGHT. No part of olympus-xmp, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2022 The developers of olympus-xmp. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/raphaelcohn/olympus-xmp/master/COPYRIGHT.


#[inline(always)]
fn write_key(file: &mut File, key: M49CodeArray) -> io::Result<()>
{
	#[inline(always)]
	fn key_byte_as_char<const index: u8>(key: M49CodeArray) -> char
	{
		key.get_unchecked_value_safe(index) as char
	}
	
	write!(file, "b\"{}{}{}\"", key_byte_as_char::<0>(key), key_byte_as_char::<1>(key), key_byte_as_char::<2>(key))
}
