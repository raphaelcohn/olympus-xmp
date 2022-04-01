// This file is part of olympus-xmp. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/raphaelcohn/olympus-xmp/master/COPYRIGHT. No part of olympus-xmp, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2022 The developers of olympus-xmp. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/raphaelcohn/olympus-xmp/master/COPYRIGHT.


/// Optimized implementation that avoids making an allocation check and forcing callers to test for being out of memory when they know there is enough.
#[inline(always)]
pub(super) unsafe fn encode_utf8_not_reserving_space(buffer_with_capacity_but_length_zero: &mut Vec<u8>, character: char, offset: usize)
{
	debug_assert!(buffer_with_capacity_but_length_zero.is_empty());
	debug_assert!(buffer_with_capacity_but_length_zero.capacity() >= (offset + size_of::<char>()));
	UnreservedEncodeUtf8::encode_utf8(buffer_with_capacity_but_length_zero, character, offset)
}

#[derive(Default, Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
struct UnreservedEncodeUtf8;

impl EncodeUtf8 for UnreservedEncodeUtf8
{
	type R = ();
	
	#[inline(always)]
	fn push_unchecked<const length: usize>(buffer: &mut Vec<u8>, offset: usize, encoded_utf8_bytes: [u8; length]) -> Self::R
	{
		let from_pointer = encoded_utf8_bytes.as_ptr();
		let to_pointer = buffer.as_mut_ptr();
		unsafe
		{
			to_pointer.add(offset).copy_from_nonoverlapping(from_pointer, length);
			buffer.set_len(offset + length)
		}
	}
}
