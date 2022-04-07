// This file is part of olympus-xmp. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/raphaelcohn/olympus-xmp/master/COPYRIGHT. No part of olympus-xmp, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2022 The developers of olympus-xmp. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/raphaelcohn/olympus-xmp/master/COPYRIGHT.


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
