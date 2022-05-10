// This file is part of olympus-xmp. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/raphaelcohn/olympus-xmp/master/COPYRIGHT. No part of olympus-xmp, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2022 The developers of olympus-xmp. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/raphaelcohn/olympus-xmp/master/COPYRIGHT.


trait Utf8SequencesParserExt<'a>: Sized
{
	fn new_percent_encoded_non_empty_path_segment(first_character_of_first_path_segment: (bool, Utf8SequenceEnum), remaining_percent_encoded_path_segment: &'a str) -> Result<Self, TryReserveError>;
	
	/// Push an UTF8 sequence decoded from percent-encoded (escaped) data, such as found in URLs.
	///
	/// Forces conversion to a heap (String).
	///
	/// Use `to_ascii_lower_case` to force conversion to lower case.
	fn push_forcing_heap_percent_encoded<E: error::Error + From<TryReserveError> + From<InvalidUtf8ParseError<PercentDecodeError>>, const to_ascii_lower_case: bool>(&mut self, remaining_utf8_bytes: &mut &'a str) -> Result<(), E>;
}

impl<'a> Utf8SequencesParserExt for Utf8SequencesParser<'a>
{
	#[inline(always)]
	fn new_percent_encoded_non_empty_path_segment(first_character_of_first_path_segment: (bool, Utf8SequenceEnum), remaining_percent_encoded_path_segment: &'a str) -> Result<Self, TryReserveError>
	{
		#[inline(always)]
		fn new_stack_rewind_buffer(utf8_sequence: Utf8SequenceEnum, remaining: &str) -> Utf8SequencesParser
		{
			let utf8_character_length = utf8_sequence.utf8_character_length();
			
			let rewound_buffer = remaining.rewind_buffer(utf8_character_length);
			
			let slice_length = utf8_character_length.add_from_str(remaining);
			Utf8SequencesParser::new_stack_raw(rewound_buffer, slice_length)
		}
		
		let (was_percent_encoded, utf8_sequence) = first_character_of_first_path_segment;
		
		if was_percent_encoded
		{
			Self::new_heap(utf8_sequence)
		}
		else
		{
			Ok(new_stack_rewind_buffer(utf8_sequence, remaining_percent_encoded_path_segment))
		}
	}
	
	#[inline(always)]
	fn push_forcing_heap_percent_encoded<E: error::Error + From<TryReserveError> + From<InvalidUtf8ParseError<PercentDecodeError>>, const to_ascii_lower_case: bool>(&mut self, remaining_utf8_bytes: &mut &'a str) -> Result<(), E>
	{
		let Utf8SequenceAndCharacter(utf8_sequence, _) = remaining_utf8_bytes.decode_next_percent_encoded_utf8::<to_ascii_lower_case>()?;
		self.push_forcing_heap_utf8_sequence_enum(utf8_sequence)?;
		Ok(())
	}
}
