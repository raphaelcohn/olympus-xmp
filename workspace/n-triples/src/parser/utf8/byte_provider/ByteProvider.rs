// This file is part of olympus-xmp. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/raphaelcohn/olympus-xmp/master/COPYRIGHT. No part of olympus-xmp, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2022 The developers of olympus-xmp. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/raphaelcohn/olympus-xmp/master/COPYRIGHT.


pub(super) trait ByteProvider where InvalidUtf8ParseError<<Self as ByteProvider>::Error>: From<<Self as ByteProvider>::Error>
{
	type Error: error::Error;
	
	const OneSliceLength: NonZeroUsize;
	
	const TwoSliceLength: NonZeroUsize;
	
	const ThreeSliceLength: NonZeroUsize;
	
	const FourSliceLength: NonZeroUsize;
	
	fn first(bytes: &[u8]) -> Result<u8, InvalidUtf8ParseError<Self::Error>>;
	
	fn two(bytes: &[u8]) -> Result<u8, Self::Error>;
	
	fn three(bytes: &[u8]) -> Result<(u8, u8), Self::Error>;
	
	fn four(bytes: &[u8]) -> Result<(u8, u8, u8), Self::Error>;
	
	#[inline(always)]
	fn decode_internal_utf8_validity_already_checked(bytes: &[u8]) -> (char, Utf8SequenceEnum, &[u8])
	{
		let first =
		{
			let first = Self::first(bytes);
			unsafe { first.unwrap_unchecked() }
		};
		
		#[inline(always)]
		const fn parse_more_than_one<BP: ByteProvider, U8SNC: Utf8SequenceNonConst>(first: u8, bytes: &[u8]) -> (Utf8SequenceEnum, NonZeroUsize, char)
		where Utf8SequenceEnum: ~const From<U8SNC>
		{
			let slice_length = U8SNC::slice_length::<BP>();
			debug_assert!(bytes.len() >= slice_length.get());
			
			let remainder = U8SNC::parse::<BP>(bytes);
			let remainder = unsafe { remainder.unwrap_unchecked() };
			let utf8_sequence = U8SNC::construct(first, remainder);
			debug_assert!(utf8_sequence.try_into_char().is_ok());
			
			(Utf8SequenceEnum::from(utf8_sequence), slice_length, unsafe { utf8_sequence.unchecked_into_char() })
		}
		
		let (utf8_sequence, slice_length, character) = if Utf8Sequence1::is(first)
		{
			(Utf8SequenceEnum::One([first]), Utf8Sequence1::slice_length::<Self>(), first as char)
		}
		else if Utf8Sequence2::is(first)
		{
			parse_more_than_one::<Self, Utf8Sequence2>(first, bytes)
		}
		else if Utf8Sequence3::is(first)
		{
			parse_more_than_one::<Self, Utf8Sequence3>(first, bytes)
		}
		else
		{
			debug_assert!(Utf8Sequence4::is(first));
			parse_more_than_one::<Self, Utf8Sequence4>(first, bytes)
		};
		
		(character, utf8_sequence, Self::postamble(bytes, slice_length))
	}
	
	#[inline(always)]
	fn decode_internal(bytes: &[u8]) -> Result<(char, Utf8CharacterLength, &[u8]), InvalidUtf8ParseError<Self::Error>>
	{
		#[inline(always)]
		fn parse_more_than_one<BP: ByteProvider, U8SNC: Utf8SequenceNonConst>(first: u8, bytes: &[u8]) -> Result<(Utf8CharacterLength, NonZeroUsize, char), InvalidUtf8ParseError<BP::Error>>
		{
			let slice_length = U8SNC::slice_length::<BP>();
			
			if bytes.len() < slice_length.get()
			{
				return Err(InvalidUtf8ParseError::DidNotExpectEndParsing(U8SNC::Length))
			}
			
			let remainder = U8SNC::parse::<BP>(bytes)?;
			Ok((U8SNC::Length, slice_length, U8SNC::construct(first, remainder).try_into_char()?))
		}
		
		let first = Self::first(bytes)?;
		
		let (utf8_character_length, slice_length, character) = if Utf8Sequence1::is(first)
		{
			// Different logic to if branches below as:-
			// (a) there is no need to check the slice length
			// (b) is always valid as a `char`.
			(Utf8Sequence1::Length, Utf8Sequence1::slice_length::<Self>(), first as char)
		}
		else if Utf8Sequence2::is(first)
		{
			parse_more_than_one::<Self, Utf8Sequence2>(first, bytes)?
		}
		else if Utf8Sequence3::is(first)
		{
			parse_more_than_one::<Self, Utf8Sequence3>(first, bytes)?
		}
		else if Utf8Sequence4::is(first)
		{
			parse_more_than_one::<Self, Utf8Sequence4>(first, bytes)?
		}
		else
		{
			return Err(InvalidUtf8ParseError::OverlyLongUtf8Sequence)
		};
		
		Ok((character, utf8_character_length, Self::postamble(bytes, slice_length)))
	}
	
	#[inline(always)]
	fn postamble(bytes: &[u8], slice_length: NonZeroUsize) -> &[u8]
	{
		bytes.get_unchecked_range_safe(slice_length.get() .. )
	}
}
