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

macro_rules! push_utf8_sequence_enum_n
{
	($self: ident, $utf8_sequence: ident, $member: ident) =>
	{
		{
			debug_assert_eq!($utf8_sequence.utf8_character_length(), Utf8CharacterLength::$member);
			
			match $utf8_sequence
			{
				Utf8SequenceEnum::$member(utf8_sequence) => $self.push_utf8_sequence(utf8_sequence),
				
				_ => unsafe { unreachable_unchecked() }
			}
		}
	}
}

impl<'a> StringSoFar<'a>
{
	#[inline(always)]
	pub(super) fn new_percent_encoded_non_empty_path_segment(first_character_of_first_path_segment: (bool, Utf8SequenceEnum), remaining_percent_encoded_path_segment: &'a str) -> Result<Self, TryReserveError>
	{
		let (was_percent_encoded, utf8_sequence) = first_character_of_first_path_segment;
		
		if was_percent_encoded
		{
			Self::new_heap(utf8_sequence)
		}
		else
		{
			Ok(Self::new_stack_rewind_buffer(utf8_sequence, remaining_percent_encoded_path_segment))
		}
	}
	
	#[inline(always)]
	pub(super) fn new_stack(remaining_utf8_bytes: &mut impl AsRef<[u8]>) -> Self
	{
		Self::new_stack_internal((*remaining_utf8_bytes).as_ref().as_ptr(), 0)
	}
	
	#[inline(always)]
	fn new_heap(utf8_sequence: Utf8SequenceEnum) -> Result<Self, TryReserveError>
	{
		let mut string = String::new();
		string.push_utf8_sequence_enum(utf8_sequence)?;
		Ok(StringSoFar::Heap(string))
	}
	
	#[inline(always)]
	fn new_stack_rewind_buffer(utf8_sequence: Utf8SequenceEnum, remaining: &str) -> Self
	{
		let utf8_character_length = utf8_sequence.utf8_character_length();
		
		let rewound_buffer = remaining.rewind_buffer(utf8_character_length);
		
		let slice_length = utf8_character_length.add_from_str(remaining);
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
	pub(super) fn to_cow_bytes(self) -> Cow<'a, [u8]>
	{
		use StringSoFar::*;
		
		match self
		{
			Heap(string) => Cow::Owned(string.into_bytes()),
			
			Stack { from, slice_length, .. } => Cow::Borrowed(Self::to_bytes(from, slice_length))
		}
	}
	
	#[inline(always)]
	pub(super) fn push_ascii_character(&mut self, ascii_character: char) -> Result<(), TryReserveError>
	{
		debug_assert!(is_ascii_character(ascii_character));
		self.push_ascii_byte(ascii_character as u8)
	}
	
	#[inline(always)]
	pub(super) fn push_ascii_byte(&mut self, ascii_byte: u8) -> Result<(), TryReserveError>
	{
		debug_assert!(is_ascii_byte(ascii_byte));
		
		use StringSoFar::*;
		match self
		{
			Heap(string) => string.push_utf8_ascii_byte(ascii_byte),
			
			Stack { slice_length, .. } =>
			{
				*slice_length = *slice_length + 1;
				Ok(())
			},
		}
	}
	
	#[inline(always)]
	pub(super) fn push_utf8_sequence_enum_2(&mut self, utf8_sequence: Utf8SequenceEnum) -> Result<(), TryReserveError>
	{
		push_utf8_sequence_enum_n!(self, utf8_sequence, Two)
	}
	
	#[inline(always)]
	pub(super) fn push_utf8_sequence_enum_3(&mut self, utf8_sequence: Utf8SequenceEnum) -> Result<(), TryReserveError>
	{
		push_utf8_sequence_enum_n!(self, utf8_sequence, Three)
	}
	
	#[inline(always)]
	pub(super) fn push_utf8_sequence_enum_4(&mut self, utf8_sequence: Utf8SequenceEnum) -> Result<(), TryReserveError>
	{
		push_utf8_sequence_enum_n!(self, utf8_sequence, Four)
	}
	
	#[inline(always)]
	pub(super) fn push_utf8_sequence<U8S: Utf8Sequence>(&mut self, utf8_sequence: U8S) -> Result<(), TryReserveError>
	{
		use StringSoFar::*;
		match self
		{
			Heap(string) => string.push_utf8_sequence(utf8_sequence),
			
			Stack { slice_length, .. } =>
			{
				let old_slice_length = *slice_length;
				*slice_length = old_slice_length + U8S::UsizeLength;
				Ok(())
			}
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
	pub(super) fn push_forcing_heap_ascii_character<const to_ascii_lower_case: bool>(&mut self, ascii_character: char) -> Result<(), TryReserveError>
	{
		debug_assert!(is_ascii_character(ascii_character));
		self.push_forcing_heap_ascii_byte::<to_ascii_lower_case>(ascii_character as u8)
	}
	
	#[inline(always)]
	pub(super) fn push_forcing_heap_ascii_byte<const to_ascii_lower_case: bool>(&mut self, ascii_byte: u8) -> Result<(), TryReserveError>
	{
		debug_assert!(is_ascii_byte(ascii_byte));
		
		let ascii_byte = if to_ascii_lower_case
		{
			to_lower_case_ascii_byte(ascii_byte)
		}
		else
		{
			ascii_byte
		};
		
		use StringSoFar::*;
		match self
		{
			Heap(string) => string.push_utf8_sequence([ascii_byte]),
			
			Stack { from, slice_length, .. } =>
			{
				let utf8_sequences = Self::utf8_sequences(from, slice_length);
				let string = String::from_utf8_unchecked_with_utf8_sequence(utf8_sequences, [ascii_byte])?;
				*self = Heap(string);
				Ok(())
			}
		}
	}
	
	#[inline(always)]
	pub(super) fn push_forcing_heap_percent_encoded<const to_ascii_lower_case: bool>(&mut self, remaining_utf8_bytes: &mut &'a str) -> Result<(), OutOfMemoryOrInvalidUtf8PercentDecodeParseError>
	{
		let utf8_sequence_and_character = remaining_utf8_bytes.decode_next_percent_encoded_utf8::<to_ascii_lower_case>()?;
		
		use StringSoFar::*;
		let utf8_sequence = utf8_sequence_and_character.0;
		match self
		{
			Heap(string) => string.push_utf8_sequence_enum(utf8_sequence)?,
			
			Stack { from, slice_length, .. } =>
			{
				let utf8_sequences = Self::utf8_sequences(from, slice_length);
				use Utf8SequenceEnum::*;
				let string = match utf8_sequence
				{
					One(utf8_sequence) => String::from_utf8_unchecked_with_utf8_sequence(utf8_sequences, utf8_sequence),
					
					Two(utf8_sequence) => String::from_utf8_unchecked_with_utf8_sequence(utf8_sequences, utf8_sequence),
					
					Three(utf8_sequence) => String::from_utf8_unchecked_with_utf8_sequence(utf8_sequences, utf8_sequence),
					
					Four(utf8_sequence) => String::from_utf8_unchecked_with_utf8_sequence(utf8_sequences, utf8_sequence),
				}?;
				
				*self = Heap(string);
			}
		}
		Ok(())
	}
	
	#[inline(always)]
	fn push_forcing_heap_UCHAR(&mut self, remaining_bytes: &mut &[u8], parse_UCHAR: impl FnOnce(&mut &[u8]) -> Result<char, UCHARParseError>) -> Result<(), OutOfMemoryOrUCHARParseError>
	{
		let character = parse_UCHAR(remaining_bytes)?;
		self.push_forcing_heap(character)?;
		Ok(())
	}
	
	#[inline(always)]
	fn push_forcing_heap(&mut self, character: char) -> Result<(), TryReserveError>
	{
		use StringSoFar::*;
		match self
		{
			Heap(string) => string.push_utf8_character(character),
			
			Stack { from, slice_length, .. } =>
			{
				let utf8_sequences = Self::utf8_sequences(from, slice_length);
				let string = String::from_utf8_unchecked_with_character(utf8_sequences, character)?;
				*self = Heap(string);
				Ok(())
			}
		}
	}
	
	#[inline(always)]
	fn utf8_sequences(from: &mut NonNull<u8>, slice_length: &mut usize) -> &'a [u8]
	{
		let from = (*from).as_ptr();
		let slice_length = *slice_length;
		unsafe { from_raw_parts(from, slice_length) }
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
}
