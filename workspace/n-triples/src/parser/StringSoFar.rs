// This file is part of olympus-xmp. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/raphaelcohn/olympus-xmp/master/COPYRIGHT. No part of olympus-xmp, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2022 The developers of olympus-xmp. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/raphaelcohn/olympus-xmp/master/COPYRIGHT.


#[derive(Debug, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub(super) enum StringSoFar<'a>
{
	Heap(String),
	
	Stack
	{
		from: NonNull<u8>,
		
		slice_length: usize,
	
		marker: PhantomData<&'a [u8]>,
	},
}

impl<'a> StringSoFar<'a>
{
	#[inline(always)]
	pub(super) fn decode_next_utf8_validity_already_checked_mandatory<E: error::Error>(remaining_utf8_bytes: &mut &'a [u8], error: E) -> Result<char, E>
	{
		StringSoFar::decode_next_utf8_validity_already_checked(remaining_utf8_bytes).ok_or(error)
	}
	
	#[inline(always)]
	pub(super) fn decode_next_utf8_validity_already_checked(remaining_utf8_bytes: &mut &[u8]) -> Option<char>
	{
		decode_next_utf8_validity_already_checked(remaining_utf8_bytes).map(|(character, _utf8_character_length)| character)
	}
	
	#[inline(always)]
	pub(super) fn new_heap(character: char, utf8_character_length: Utf8CharacterLength) -> Result<Self, TryReserveError>
	{
		let mut string = String::new();
		Self::string_push_character_of_known_length(&mut string, character, utf8_character_length)?;
		Ok(StringSoFar::Heap(string))
	}
	
	#[inline(always)]
	pub(super) fn new_stack(remaining_bytes: &mut &'a [u8]) -> Self
	{
		Self::new_stack_internal((*remaining_bytes).as_ptr(), 0)
	}
	
	#[inline(always)]
	pub(super) fn new_stack_rewind_buffer(remaining_utf8_bytes: &[u8], utf8_character_length: Utf8CharacterLength) -> Self
	{
		let rewound_buffer = remaining_utf8_bytes.rewind_buffer(utf8_character_length);
		
		let slice_length = utf8_character_length.add_from_bytes(remaining_utf8_bytes);
		Self::new_stack_internal(rewound_buffer, slice_length)
	}
	
	#[inline(always)]
	const fn new_stack_internal(from: *const u8, slice_length: usize) -> Self
	{
		StringSoFar::Stack
		{
			from: new_non_null(from as *mut _),
			slice_length,
			marker: PhantomData,
		}
	}
	
	#[inline(always)]
	pub(super) fn to_cow(self) -> Cow<'a, str>
	{
		use StringSoFar::*;
		
		match self
		{
			Heap(string) => Cow::Owned(string),
			
			Stack { from, slice_length, .. } => Cow::Borrowed(Self::to_str(from, slice_length))
		}
	}
	
	#[inline(always)]
	pub(super) fn push_forcing_heap_UCHAR4(&mut self, remaining_bytes: &mut &[u8]) -> Result<(), OutOfMemoryOrUCHARParseError>
	{
		self.push_forcing_heap_UCHAR(remaining_bytes, UCHARParser::parse_UCHAR4)
	}
	
	#[inline(always)]
	pub(super) fn push_forcing_heap_UCHAR8(&mut self, remaining_bytes: &mut &[u8]) -> Result<(), OutOfMemoryOrUCHARParseError>
	{
		self.push_forcing_heap_UCHAR(remaining_bytes, UCHARParser::parse_UCHAR8)
	}
	
	#[inline(always)]
	pub(super) fn push_forcing_heap_ascii_to_lower_case(&mut self, character: char) -> Result<(), TryReserveError>
	{
		debug_assert!(Self::is_ascii(character));
		
		self.push_forcing_heap_ascii(character.to_ascii_lowercase())
	}
	
	#[inline(always)]
	pub(super) fn push_forcing_heap_ascii(&mut self, character: char) -> Result<(), TryReserveError>
	{
		debug_assert!(Self::is_ascii(character));
		
		use StringSoFar::*;
		match self
		{
			Heap(string) => Self::string_push_character_of_known_length(string, character, Utf8CharacterLength::One),
			
			Stack { from, slice_length, .. } =>
			{
				let string = Self::from_stack_to_heap_ascii(*from, *slice_length, character)?;
				*self = Heap(string);
				Ok(())
			}
		}
	}
	
	#[inline(always)]
	pub(super) fn push_forcing_heap_percent_encoded<const to_ascii_lower_case: bool>(&mut self, remaining_utf8_bytes: &mut &'a [u8]) -> Result<(), OutOfMemoryOrInvalidUtf8PercentDecodeParseError>
	{
		#[inline(always)]
		fn from_stack_to_heap_n<const length: usize>(from: NonNull<u8>, slice_length: usize, encoded_utf8_bytes: [u8; length]) -> Result<String, TryReserveError>
		{
			StringSoFar::use_new_buffer::<_, length>(from, slice_length, |buffer| encode_utf8_push_unchecked(buffer, slice_length, encoded_utf8_bytes))
		}
		
		let (character, utf8_character_length) = decode_next_percent_encoded_utf8(remaining_utf8_bytes)?;
		
		let character = if to_ascii_lower_case
		{
			if utf8_character_length == Utf8CharacterLength::One
			{
				character.to_ascii_lowercase()
			}
			else
			{
				character
			}
		}
		else
		{
			character
		};
		
		use StringSoFar::*;
		match self
		{
			Heap(string) => Self::string_push_character_of_known_length(string, character, utf8_character_length)?,
			
			Stack { from, slice_length, .. } =>
			{
				use Utf8CharacterLength::*;
				
				let from = *from;
				let slice_length = *slice_length;
				let string = match utf8_character_length
				{
					One => from_stack_to_heap_n::<1>(from, slice_length, encode_utf8_bytes_1(character as u32)),
					
					Two => from_stack_to_heap_n::<2>(from, slice_length, encode_utf8_bytes_2(character as u32)),
					
					Three => from_stack_to_heap_n::<3>(from, slice_length, encode_utf8_bytes_3(character as u32)),
					
					Four => from_stack_to_heap_n::<4>(from, slice_length, encode_utf8_bytes_4(character as u32)),
				}?;
				*self = Heap(string);
			}
		}
		Ok(())
	}
	
	#[inline(always)]
	fn push_forcing_heap_UCHAR(&mut self, remaining_bytes: &mut &[u8], parse_UCHAR: impl FnOnce(&mut &[u8]) -> Result<char, UCHARParseError>) -> Result<(), OutOfMemoryOrUCHARParseError>
	{
		self.push_forcing_heap(parse_UCHAR(remaining_bytes)?)?;
		Ok(())
	}
	
	#[inline(always)]
	fn push_forcing_heap(&mut self, character: char) -> Result<(), TryReserveError>
	{
		#[inline(always)]
		fn from_stack_to_heap(from: NonNull<u8>, slice_length: usize, character: char) -> Result<String, TryReserveError>
		{
			const CharacterSize: usize = size_of::<char>();
			StringSoFar::use_new_buffer::<_, CharacterSize>(from, slice_length, |buffer| unsafe { encode_utf8_not_reserving_space(buffer, character, slice_length) })
		}
		
		use StringSoFar::*;
		match self
		{
			Heap(string) => Self::string_push_character(string, character),
			
			Stack { from, slice_length, .. } =>
			{
				let string = from_stack_to_heap(*from, *slice_length, character)?;
				*self = Heap(string);
				Ok(())
			}
		}
	}
	
	#[inline(always)]
	pub(super) fn push_ascii(&mut self, character: char) -> Result<(), TryReserveError>
	{
		debug_assert!(Self::is_ascii(character));
		
		use StringSoFar::*;
		match self
		{
			Heap(string) => Self::string_push_ascii(string, character),
			
			Stack { slice_length, .. } =>
			{
				*slice_length = *slice_length + 1;
				Ok(())
			},
		}
	}
	
	#[inline(always)]
	pub(super) fn push(&mut self, character: char, utf8_character_length: Utf8CharacterLength) -> Result<(), TryReserveError>
	{
		use StringSoFar::*;
		match self
		{
			Heap(string) => Self::string_push_character_of_known_length(string, character, utf8_character_length),
			
			Stack { slice_length, .. } =>
			{
				let old_slice_length = *slice_length;
				*slice_length = utf8_character_length.add(old_slice_length);
				Ok(())
			}
		}
	}
	
	#[inline(always)]
	fn string_push_ascii(string: &mut String, character: char) -> Result<(), TryReserveError>
	{
		debug_assert!(Self::is_ascii(character));
		Self::encode_utf8_push_unchecked::<1>(string, encode_utf8_bytes_1(character as u32))
	}
	
	#[inline(always)]
	fn string_push_character_of_known_length(string: &mut String, character: char, utf8_character_length: Utf8CharacterLength) -> Result<(), TryReserveError>
	{
		use Utf8CharacterLength::*;
		
		match utf8_character_length
		{
			One => Self::string_push_ascii(string, character),
			
			Two => Self::encode_utf8_push_unchecked::<2>(string, encode_utf8_bytes_2(character as u32)),
			
			Three => Self::encode_utf8_push_unchecked::<3>(string, encode_utf8_bytes_3(character as u32)),
			
			Four => Self::encode_utf8_push_unchecked::<4>(string, encode_utf8_bytes_4(character as u32)),
		}
	}
	
	#[inline(always)]
	fn encode_utf8_push_unchecked<const length: usize>(string: &mut String, encoded_utf8_bytes: [u8; length]) -> Result<(), TryReserveError>
	{
		let offset = string.len();
		let buffer = unsafe { string.as_mut_vec() };
		buffer.try_reserve(length)?;
		encode_utf8_push_unchecked::<length>(buffer, offset, encoded_utf8_bytes);
		Ok(())
	}
	
	#[inline(always)]
	fn string_push_character(string: &mut String, character: char) -> Result<(), TryReserveError>
	{
		let buffer = unsafe { string.as_mut_vec() };
		encode_utf8_reserving_space(buffer, character)
	}
	
	#[inline(always)]
	fn from_stack_to_heap_ascii(from: NonNull<u8>, slice_length: usize, character: char) -> Result<String, TryReserveError>
	{
		debug_assert!(Self::is_ascii(character));
		
		Self::use_new_buffer::<_, 1>(from, slice_length, |buffer| encode_utf8_push_unchecked::<1>(buffer, slice_length, encode_utf8_bytes_1(character as u32)))
	}
	
	/// NOTE: the buffer passed to buffer_user with have a `.len()` of `0`.
	#[inline(always)]
	fn use_new_buffer<BU: FnOnce(&mut Vec<u8>) -> (), const CharacterSize: usize>(from: NonNull<u8>, slice_length: usize, buffer_user: BU) -> Result<String, TryReserveError>
	{
		let minimum_capacity = slice_length + CharacterSize;
		let mut buffer = Self::new_buffer(minimum_capacity)?;
		
		{
			let to_pointer: *mut u8 = buffer.as_mut_ptr();
			let from_pointer = from.as_ptr() as *const u8;
			// NOTE: Explicitly does NOT call buffer.set_len(slice_length), as this is done in `encode_utf8_not_reserving_space()` below.
			unsafe { to_pointer.copy_from_nonoverlapping(from_pointer, slice_length); }
		}
		
		buffer_user(&mut buffer);
		
		Ok(unsafe { String::from_utf8_unchecked(buffer) })
	}
	
	#[inline(always)]
	fn new_buffer(minimum_capacity: usize) -> Result<Vec<u8>, TryReserveError>
	{
		let mut buffer = Vec::new();
		buffer.try_reserve(minimum_capacity)?;
		Ok(buffer)
	}
	
	#[inline(always)]
	fn to_str(from: NonNull<u8>, slice_length: usize) -> &'a str
	{
		let range = Self::to_bytes(from, slice_length);
		unsafe { from_utf8_unchecked(range) }
	}
	
	#[inline(always)]
	fn to_bytes(from: NonNull<u8>, slice_length: usize) -> &'a [u8]
	{
		unsafe { from_raw_parts(from.as_ptr(), slice_length) }
	}
	
	#[inline(always)]
	const fn is_ascii(character: char) -> bool
	{
		character <= x7F
	}
}
