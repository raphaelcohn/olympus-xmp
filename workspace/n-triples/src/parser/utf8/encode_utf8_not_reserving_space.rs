// This file is part of olympus-xmp. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/raphaelcohn/olympus-xmp/master/COPYRIGHT. No part of olympus-xmp, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2022 The developers of olympus-xmp. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/raphaelcohn/olympus-xmp/master/COPYRIGHT.


/// Optimized implementation that avoids making an allocation check and forcing callers to test for being out of memory when they know there is enough.
#[inline(always)]
pub(super) unsafe fn encode_utf8_not_reserving_space(buffer_with_capacity_but_length_zero: &mut Vec<u8>, offset: usize, character: char)
{
	UnreservedEncodeUtf8::new(buffer_with_capacity_but_length_zero, offset).encode_utf8(character)
}
