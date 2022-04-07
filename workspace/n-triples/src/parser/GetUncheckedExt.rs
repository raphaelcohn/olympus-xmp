// This file is part of olympus-xmp. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/raphaelcohn/olympus-xmp/master/COPYRIGHT. No part of olympus-xmp, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2022 The developers of olympus-xmp. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/raphaelcohn/olympus-xmp/master/COPYRIGHT.


pub(super) trait GetUncheckedExt<T>: GetUnchecked<T>
{
	#[inline(always)]
	fn before_index(&self, index: usize) -> &[T]
	{
		self.get_unchecked_range_safe(.. index)
	}
	
	#[inline(always)]
	fn after_index(&self, index: usize) -> &[T]
	{
		self.get_unchecked_range_safe((index + 1) ..)
	}
}

impl<T, GU: GetUnchecked<T>> GetUncheckedExt<T> for GU
{
}
