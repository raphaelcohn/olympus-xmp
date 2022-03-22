// This file is part of olympus-xmp. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/raphaelcohn/olympus-xmp/master/COPYRIGHT. No part of olympus-xmp, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2022 The developers of olympus-xmp. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/raphaelcohn/olympus-xmp/master/COPYRIGHT.


pub(super) trait Subtag<'a>
{
	fn byte_0(self) -> u8;
	
	fn slice_less_first_byte(self) -> &'a [u8];
}

impl<'a> Subtag<'a> for &'a [u8]
{
	#[inline(always)]
	fn byte_0(self) -> u8
	{
		self.get_unchecked_value_safe(0usize)
	}
	
	#[inline(always)]
	fn slice_less_first_byte(self) -> &'a[u8]
	{
		self.get_unchecked_range_safe(1 .. )
	}
}
