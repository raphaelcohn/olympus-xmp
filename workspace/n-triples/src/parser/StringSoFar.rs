// This file is part of olympus-xmp. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/raphaelcohn/olympus-xmp/master/COPYRIGHT. No part of olympus-xmp, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2022 The developers of olympus-xmp. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/raphaelcohn/olympus-xmp/master/COPYRIGHT.


#[derive(Debug, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub(super) enum StringSoFar<'a>
{
	Stack
	{
		from: &'a [u8],
		
		slice_length: usize,
	},

	Heap(String),
}

impl<'a> StringSoFar<'a>
{
	#[inline(always)]
	pub(super) fn initial(remaining_bytes: &mut &'a [u8]) -> Self
	{
		StringSoFar::Stack { from: *remaining_bytes, slice_length: 0 }
	}
	
	#[inline(always)]
	pub(super) fn to_cow(self) -> Cow<'a, str>
	{
		use StringSoFar::*;
		
		match self
		{
			Heap(string) => Cow::Owned(string),
			
			Stack { from, slice_length } => Cow::Borrowed(Self::to_str(from, slice_length))
		}
	}
	
	#[inline(always)]
	pub(super) fn push_forcing_heap_UCHAR4(&mut self, remaining_bytes: &mut &[u8]) -> Result<(), UCHARParseError>
	{
		const length: usize = 4;
		self.push_forcing_heap_UCHAR::<_, length>(remaining_bytes, |bytes| Ok(Self::hex_digit::<0, _>(bytes)? | Self::hex_digit::<1, _>(bytes)? | Self::hex_digit::<2, _>(bytes)? | Self::hex_digit::<3, _>(bytes)?))
	}
	
	#[inline(always)]
	pub(super) fn push_forcing_heap_UCHAR8(&mut self, remaining_bytes: &mut &[u8]) -> Result<(), UCHARParseError>
	{
		const length: usize = 8;
		self.push_forcing_heap_UCHAR::<_, length>(remaining_bytes, |bytes| Ok(Self::hex_digit::<0, _>(bytes)? | Self::hex_digit::<1, _>(bytes)? | Self::hex_digit::<2, _>(bytes)? | Self::hex_digit::<3, _>(bytes)? | Self::hex_digit::<4, _>(bytes)? | Self::hex_digit::<5, _>(bytes)? | Self::hex_digit::<6, _>(bytes)? | Self::hex_digit::<7, _>(bytes)?))
	}
	
	#[doc(hidden)]
	#[inline(always)]
	fn push_forcing_heap_UCHAR<PHD: FnOnce(&[u8; length]) -> Result<u32, UCHARParseError>, const length: usize>(&mut self, remaining_bytes: &mut &[u8], parse_hex_digits: PHD) -> Result<(), UCHARParseError>
	{
		{
			let expected_length = length as usize;
			let actual_length = remaining_bytes.len();
			if actual_length < expected_length
			{
				return Err(UCHARParseError::TooFewBytesRemain { expected_length, actual_length })
			}
		}
		
		let bytes = *remaining_bytes;
		
		let bytes_pointer = bytes.as_ptr().cast::<[u8; length]>();
		let value = parse_hex_digits(unsafe { &*bytes_pointer })?;
		
		let character = char::try_from(value)?;
		*remaining_bytes = bytes.get_unchecked_range_safe(length .. );
		self.push_forcing_heap(character)?;
		Ok(())
	}
	
	#[inline(always)]
	pub(super) fn push_forcing_heap(&mut self, character: char) -> Result<(), TryReserveError>
	{
		use StringSoFar::*;
		
		match self
		{
			Heap(string) => Self::string_push_character(string, character),
			
			Stack { from, slice_length } =>
			{
				let string = Self::from_stack_to_heap(*from, *slice_length, character)?;
				*self = Heap(string);
				Ok(())
			}
		}
	}
	
	#[inline(always)]
	pub(super) fn push_ascii(&mut self, character: char) -> Result<(), TryReserveError>
	{
		use StringSoFar::*;
		
		debug_assert!(Self::is_ascii(character));
		
		match self
		{
			Heap(string) => Self::string_push_ascii(string, character as u8),
			
			Stack { slice_length, .. } =>
			{
				*slice_length = *slice_length + 1;
				Ok(())
			},
		}
	}
	
	#[inline(always)]
	pub(super) fn push(&mut self, character: char) -> Result<(), TryReserveError>
	{
		use StringSoFar::*;
		
		match self
		{
			Heap(string) => Self::string_push_character(string, character),
			
			Stack { from, slice_length } =>
			{
				let old_slice_length = *slice_length;
				if Self::is_ascii(character)
				{
					*slice_length = old_slice_length + 1;
				}
				else
				{
					let string = Self::from_stack_to_heap(*from, old_slice_length, character)?;
					*self = Heap(string);
				}
				Ok(())
			}
		}
	}
	
	#[doc(hidden)]
	#[inline(always)]
	fn from_stack_to_heap(from: &[u8], slice_length: usize, character: char) -> Result<String, TryReserveError>
	{
		let minimum_capacity = slice_length + size_of::<char>();
		
		let mut buffer = Vec::new();
		buffer.try_reserve(minimum_capacity)?;
		
		{
			let to_pointer: *mut u8 = buffer.as_mut_ptr();
			let from_pointer = from.as_ptr();
			// NOTE: Explicitly does NOT call buffer.set_len(slice_length), as this is done in `encode_utf8_not_reserving_space()` below.
			unsafe { to_pointer.copy_from_nonoverlapping(from_pointer, slice_length); }
		}
		
		unsafe { encode_utf8_not_reserving_space(&mut buffer, character, slice_length) };
		Ok(unsafe { String::from_utf8_unchecked(buffer) })
	}
	
	#[doc(hidden)]
	#[inline(always)]
	fn string_push_ascii(string: &mut String, character: u8) -> Result<(), TryReserveError>
	{
		let buffer = unsafe { string.as_mut_vec() };
		buffer.try_reserve(1)?;
		buffer.push_unchecked(character);
		Ok(())
	}
	
	#[doc(hidden)]
	#[inline(always)]
	fn string_push_character(string: &mut String, character: char) -> Result<(), TryReserveError>
	{
		let buffer = unsafe { string.as_mut_vec() };
		encode_utf8_reserving_space(buffer, character)
	}
	
	#[doc(hidden)]
	#[inline(always)]
	fn to_str(from: &'a [u8], slice_length: usize) -> &'a str
	{
		let range = Self::to_bytes(from, slice_length);
		unsafe { from_utf8_unchecked(range) }
	}
	
	#[doc(hidden)]
	#[inline(always)]
	fn to_bytes(from: &'a [u8], slice_length: usize) -> &'a [u8]
	{
		from.get_unchecked_range_safe(.. slice_length)
	}
	
	#[doc(hidden)]
	#[inline(always)]
	fn hex_digit<const index: usize, const length: usize>(bytes: &[u8; length]) -> Result<u32, UCHARParseError>
	{
		debug_assert!(index < length);
		
		let potential_hex_digit = bytes.get_unchecked_value_safe(index);
		let correction = match potential_hex_digit
		{
			_0 ..= _9 => _0,
			
			A ..= F => A,
			
			a ..= f => a,
			
			_ => return Err(UCHARParseError::InvalidHexDigit(potential_hex_digit)),
		};
		let value = potential_hex_digit - correction;
		
		let shift = 4 * (length - index - 1);
		Ok((value as u32) << (shift as u32))
	}
	
	#[doc(hidden)]
	#[inline(always)]
	const fn is_ascii(character: char) -> bool
	{
		character <= (0x7F as char)
	}
}
