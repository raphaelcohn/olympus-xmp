// This file is part of olympus-xmp. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/raphaelcohn/olympus-xmp/master/COPYRIGHT. No part of olympus-xmp, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2022 The developers of olympus-xmp. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/raphaelcohn/olympus-xmp/master/COPYRIGHT.


#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub(super) struct MemchrIterator<'a, const needle: u8>
{
	bytes: Option<&'a [u8]>,
}

impl<'a, const needle: u8> Iterator for MemchrIterator<'a, needle>
{
	type Item = &'a [u8];
	
	#[inline(always)]
	fn next(&mut self) -> Option<Self::Item>
	{
		let bytes = match self.bytes
		{
			None => return None,
			
			Some(bytes) => bytes,
		};
		
		match memchr(needle, bytes)
		{
			None =>
			{
				self.bytes = None;
				Some(bytes)
			},
			
			Some(index) =>
			{
				let slice = bytes.get_unchecked_range_safe(.. index);
				self.bytes = Some(bytes.get_unchecked_range_safe((index + 1) .. ));
				Some(slice)
			}
		}
	}
}

impl<'a, const needle: u8> MemchrIterator<'a, needle>
{
	#[inline(always)]
	pub(super) const fn from_str(str: &'a str) -> Self
	{
		Self::new(str.as_bytes())
	}
	
	#[inline(always)]
	pub(super) fn next_first(&mut self) -> &'a [u8]
	{
		let first = self.next();
		unsafe { first.unwrap_unchecked() }
	}
	
	#[inline(always)]
	pub(super) fn is_empty(&self) -> bool
	{
		self.bytes.is_none()
	}
	
	#[inline(always)]
	const fn new(bytes: &'a [u8]) -> Self
	{
		Self
		{
			bytes: Some(bytes),
		}
	}
}
