// This file is part of olympus-xmp. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/raphaelcohn/olympus-xmp/master/COPYRIGHT. No part of olympus-xmp, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2022 The developers of olympus-xmp. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/raphaelcohn/olympus-xmp/master/COPYRIGHT.


#[inline(always)]
pub(super) fn get_0(mut remaining_bytes: &mut &[u8]) -> Option<u8>
{
	let bytes = *remaining_bytes;
	if bytes.is_empty()
	{
		None
	}
	else
	{
		let value = bytes.get_unchecked_value_safe(0);
		*remaining_bytes = remaining_bytes.get_unchecked_range_mut_safe(1 .. );
		Some(value)
	}
}
