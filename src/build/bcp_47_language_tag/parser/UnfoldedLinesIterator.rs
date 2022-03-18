// This file is part of olympus-xmp. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/raphaelcohn/olympus-xmp/master/COPYRIGHT. No part of olympus-xmp, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2022 The developers of olympus-xmp. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/raphaelcohn/olympus-xmp/master/COPYRIGHT.


struct UnfoldedLinesIterator<'a>
{
	buffer: &'a [u8],
}

impl<'a> Iterator for UnfoldedLinesIterator<'a>
{
	type Item = Cow<'a, str>;
	
	fn next(&mut self) -> Option<Self::Item>
	{
		let length = self.buffer.len();
		if length == 0
		{
			return None
		}
		
		use Cow::*;
		let mut slices = match self.next_line_feed_index()
		{
			None => return Some(Borrowed(self.empty_buffer())),
			
			Some(index) => Borrowed(self.slice(index)),
		};
		
		loop
		{
			if self.buffer.is_empty()
			{
				break
			}
			
			const Space: u8 = b' ';
			if self.buffer.get_unchecked_value_safe(0u8) != Space
			{
				break
			}
			self.buffer = self.get_unchecked_range_safe(1u8 .. );
			
			match self.next_line_feed_index()
			{
				None =>
				{
					slices = Self::append_unfolded_line(slices, self.empty_buffer());
					break
				}
				
				Some(index) =>
				{
					slices = Self::append_unfolded_line(slices, self.slice(index));
					continue
				},
			}
		}
		
		Some(slices)
	}
}

impl<'a> UnfoldedLinesIterator<'a>
{
	const fn new(buffer: &'a str) -> Self
	{
		Self
		{
			buffer: buffer.as_bytes(),
		}
	}
	
	fn append_unfolded_line(slices: Cow<'a, str>, unfolded_line: &str) -> Cow<'a, str>
	{
		use Cow::*;
		let mut unfolded = match slices
		{
			Borrowed(one) => one.to_owned(),
			
			Owned(unfolded) => unfolded,
		};
		unfolded.push_str(unfolded_line);
		Owned(unfolded)
	}
	
	fn next_line_feed_index(&mut self) -> Option<usize>
	{
		const LineFeed: u8 = b'\n';
		memchr(LineFeed, self.buffer)
	}
	
	fn empty_buffer(&mut self) -> &'a str
	{
		static EmptyBuffer: &'static [u8] = &[];
		
		let slice = Self::to_str(self.buffer);
		self.buffer = EmptyBuffer;
		slice
	}
	
	fn slice(&mut self, index: usize) -> &'a str
	{
		let slice = Self::to_str(self.get_unchecked_range_safe(.. index));
		self.buffer = self.get_unchecked_range_safe((index + 1) ..);
		slice
	}
	
	fn get_unchecked_range_safe<AUR: AsUsizeRange<u8>>(&self, range: AUR) -> &'a [u8]
	{
		self.buffer.get_unchecked_range_safe(range)
	}
	
	fn to_str(slice: &'a [u8]) -> &'a str
	{
		unsafe { from_utf8_unchecked(slice) }
	}
}
