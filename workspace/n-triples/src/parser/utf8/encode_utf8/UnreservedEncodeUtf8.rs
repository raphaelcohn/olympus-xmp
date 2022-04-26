// This file is part of olympus-xmp. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/raphaelcohn/olympus-xmp/master/COPYRIGHT. No part of olympus-xmp, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2022 The developers of olympus-xmp. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/raphaelcohn/olympus-xmp/master/COPYRIGHT.


pub(super) struct UnreservedEncodeUtf8<'a>
{
	buffer: &'a mut Vec<u8>,
	
	offset: usize
}

impl<'a> EncodeUtf8 for UnreservedEncodeUtf8<'a>
{
	type R = ();
	
	#[inline(always)]
	fn push_unchecked<const length: usize>(mut self, encoded_utf8_bytes: [u8; length]) -> Self::R
	{
		let from_pointer = encoded_utf8_bytes.as_ptr();
		let to_pointer = self.to_pointer().as_ptr();
		let new_length = self.offset() + length;
		unsafe { to_pointer.copy_from_nonoverlapping(from_pointer, length) };
		self.set_length(new_length)
	}
}

impl<'a> UnreservedEncodeUtf8<'a>
{
	#[inline(always)]
	pub(super) const fn new(buffer: &'a mut Vec<u8>, offset: usize) -> Self
	{
		Self
		{
			buffer,
			offset,
		}
	}
	
	#[inline(always)]
	fn try_reserve(&mut self, length: usize) -> Result<(), TryReserveError>
	{
		self.buffer.try_reserve(length)
	}
	
	#[inline(always)]
	fn to_pointer(&mut self) -> NonNull<u8>
	{
		let pointer = self.buffer.as_mut_ptr();
		let to_pointer = unsafe { pointer.add(self.offset) };
		new_non_null(to_pointer)
	}
	
	#[inline(always)]
	fn offset(&self) -> usize
	{
		self.offset
	}
	
	#[inline(always)]
	fn set_length(&mut self, new_length: usize)
	{
		unsafe { self.buffer.set_len(new_length) }
	}
}
