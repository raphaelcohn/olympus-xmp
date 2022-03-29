// This file is part of olympus-xmp. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/raphaelcohn/olympus-xmp/master/COPYRIGHT. No part of olympus-xmp, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2022 The developers of olympus-xmp. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/raphaelcohn/olympus-xmp/master/COPYRIGHT.


/// Does not handle:-
///
/// * Escape sequences (eg `\"`).
/// * Doubled double quotes (sic) (eg `""` to mean an escaped quote).
/// * `CRLF` end of line sequences (which manifest as a trailing `CR`).
///
/// Panics if:-
///
/// * Encounters an unmatched double quote.
#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub(super) struct CrudeCsvLineIterator<'a>(Option<&'a [u8]>);

impl<'a> From<&'a str> for CrudeCsvLineIterator<'a>
{
	#[inline(always)]
	fn from(value: &'a str) -> Self
	{
		Self::from(value.as_bytes())
	}
}

impl<'a> From<&'a [u8]> for CrudeCsvLineIterator<'a>
{
	#[inline(always)]
	fn from(value: &'a [u8]) -> Self
	{
		Self(Some(value))
	}
}

impl<'a> Iterator for CrudeCsvLineIterator<'a>
{
	type Item = &'a [u8];
	
	#[inline(always)]
	fn next(&mut self) -> Option<Self::Item>
	{
		const DoubleQuote: u8 = b'"';
		
		let bytes = self.0?;
		let length = bytes.len();
		
		if length == 0
		{
			self.set_empty();
			Some(bytes)
		}
		else if bytes.get_unchecked_value_safe(0) == DoubleQuote
		{
			let haystack = bytes.get_unchecked_range_safe(1 .. );
			let index = memchr(DoubleQuote, haystack).expect("Field not terminated by closing double quote");
			let slice = haystack.get_unchecked_range_safe(.. index);
			let expected_comma_index = index + 1;
			
			if expected_comma_index == haystack.len()
			{
				self.set_empty();
			}
			else
			{
				assert_eq!(haystack.get_unchecked_value_safe(expected_comma_index), Self::Comma, "Closing double quote not followed by comma");
				let from = expected_comma_index + 1;
				self.set_more(haystack, from);
			}
			Some(slice)
		}
		else
		{
			match memchr(Self::Comma, bytes)
			{
				None =>
				{
					self.set_empty();
					Some(bytes)
				}
				
				Some(index) =>
				{
					let slice = bytes.get_unchecked_range_safe(.. index);
					self.set_more(bytes, index + 1);
					Some(slice)
				}
			}
		}
	}
}

impl<'a> CrudeCsvLineIterator<'a>
{
	const Comma: u8 = b',';
	
	#[inline(always)]
	fn snapshot(&self) -> Self
	{
		self.clone()
	}
	
	#[inline(always)]
	fn reset(&mut self, snapshot: Self)
	{
		*self = snapshot
	}
	
	#[inline(always)]
	fn reset_and_skip_comma_next(&mut self, snapshot: Self) -> &'a [u8]
	{
		self.reset(snapshot);
		
		let bytes =
		{
			let bytes = self.0;
			unsafe { bytes.unwrap_unchecked() }
		};
		
		let first_comma_index =
		{
			let skip = memchr(Self::Comma, bytes);
			unsafe { skip.unwrap_unchecked() }
		};
		
		let from_index = first_comma_index + 1;
		let haystack = bytes.get_unchecked_range_safe(from_index .. );
		let length = match memchr(Self::Comma, haystack)
		{
			None =>
			{
				self.set_empty();
				haystack.len()
			},
			
			Some(second_comma_index) =>
			{
				self.set_more(haystack, second_comma_index + 1);
				second_comma_index
			}
		};
		
		let to_index = from_index + length;
		bytes.get_unchecked_range_safe(.. to_index)
	}
	
	#[inline(always)]
	fn set_empty(&mut self)
	{
		self.0 = None;
	}
	
	#[inline(always)]
	fn set_more(&mut self, bytes: &'a [u8], from: usize)
	{
		self.0 = Some(bytes.get_unchecked_range_safe(from .. ));
	}
}
