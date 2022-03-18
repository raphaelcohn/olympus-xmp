// This file is part of olympus-xmp. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/raphaelcohn/olympus-xmp/master/COPYRIGHT. No part of olympus-xmp, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2022 The developers of olympus-xmp. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/raphaelcohn/olympus-xmp/master/COPYRIGHT.


#[derive(Debug, Copy, Clone)]
pub(in crate::bcp_47_language_tag) struct UninitialisedArray<RB: RestrictedByte, const length: usize>([MaybeUninit<RB>; length]);

impl<RB: RestrictedByte, const length: usize> const Default for UninitialisedArray<RB, length>
{
	#[inline(always)]
	fn default() -> Self
	{
		Self(MaybeUninit::uninit_array())
	}
}

impl<RB: RestrictedByte, const length: usize> UninitialisedArray<RB, length>
{
	#[inline(always)]
	pub(in crate::bcp_47_language_tag) fn convert(&mut self, byte: u8, index: usize)
	{
		debug_assert!(index < length);
		
		*self.0.get_unchecked_mut_safe(index) = MaybeUninit::new(RB::construct(byte))
	}
	
	#[inline(always)]
	pub(in crate::bcp_47_language_tag) fn initialise(self) -> [RB; length]
	{
		unsafe { MaybeUninit::array_assume_init(self.0) }
	}
}
