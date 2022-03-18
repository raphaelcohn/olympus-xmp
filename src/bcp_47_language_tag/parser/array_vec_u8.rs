// This file is part of olympus-xmp. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/raphaelcohn/olympus-xmp/master/COPYRIGHT. No part of olympus-xmp, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2022 The developers of olympus-xmp. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/raphaelcohn/olympus-xmp/master/COPYRIGHT.


#[inline(always)]
pub(super) fn array_vec_u8<const length: usize>(bytes: &[u8]) -> ArrayVec<u8, length>
{
	let bytes_length = bytes.len();
	debug_assert!(bytes_length <= length);
	
	let mut array_vec_u8 = ArrayVec::new_const();
	let to_pointer: *mut u8 = array_vec_u8.as_mut_ptr();
	let from_pointer = bytes.as_ptr();
	unsafe
	{
		to_pointer.copy_from_nonoverlapping(from_pointer, bytes_length);
		array_vec_u8.set_len(bytes_length)
	};
	array_vec_u8
}
