// This file is part of olympus-xmp. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/raphaelcohn/olympus-xmp/master/COPYRIGHT. No part of olympus-xmp, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2022 The developers of olympus-xmp. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/raphaelcohn/olympus-xmp/master/COPYRIGHT.


trait AbsoluteInternationalizedResourceIdentifierNTripleParser<'a>: Sized
{
	fn parse<R>(remaining_bytes: &mut &'a [u8], constructor: impl FnOnce(Self) -> R) -> Result<R, AbsoluteInternationalizedResourceIdentifierParseError>;
	
	fn parse_escaped_string(remaining_bytes: &mut &'a [u8]) -> Result<Utf8SequencesParser<'a>, AbsoluteInternationalizedResourceNTripleEscapedIdentifierParseError>;
}

impl<'a, const PathDepth: usize> AbsoluteInternationalizedResourceIdentifierNTripleParser<'a> for AbsoluteInternationalizedResourceIdentifier<'a, PathDepth>
{
	fn parse<R>(remaining_bytes: &mut &'a [u8], constructor: impl FnOnce(Self) -> R) -> Result<R, AbsoluteInternationalizedResourceIdentifierParseError>
	{
		let string = Self::parse_escaped_string(remaining_bytes)?;
		let string = string.to_cow();
		let this = Self::parse_string(string)?;
		Ok(constructor(this))
	}
	
	#[inline(always)]
	fn parse_escaped_string(remaining_bytes: &mut &'a [u8]) -> Result<Utf8SequencesParser<'a>, AbsoluteInternationalizedResourceNTripleEscapedIdentifierParseError>
	{
		use AbsoluteInternationalizedResourceNTripleEscapedIdentifierParseError::*;
		
		let mut string = Utf8SequencesParser::new_stack(remaining_bytes);
		
		macro_rules! ascii_control_characters_c0
		{
			() =>
			{
				0x00 ..= 0x20
			}
		}
		
		loop
		{
			let Utf8SequenceAndCharacter(utf8_sequence, _) = remaining_bytes.decode_next_utf8()?.ok_or(DidNotExpectEndParsingBody)?;
			use Utf8SequenceEnum::*;
			match utf8_sequence
			{
				One([ascii_byte]) => match ascii_byte
				{
					CloseAngleBracket => break,
					
					invalid @ (ascii_control_characters_c0!() | DoubleQuote | OpenBrace | CloseBrace | Pipe | Backtick) => return Err(InvalidAsciiByte(invalid)),
					
					Backslash => match remaining_bytes.pop_first_or_error(EndOfFileParsingEscapeSequence)?
					{
						u => string.push_forcing_heap_UCHAR4(remaining_bytes, InvalidUCHAR4EscapeSequence, OutOfMemory)?,
						
						U => string.push_forcing_heap_UCHAR8(remaining_bytes, InvalidUCHAR8EscapeSequence, OutOfMemory)?,
						
						invalid => return Err(InvalidEscapeSequence(invalid)),
					}
					
					_ => string.push_ascii_byte(ascii_byte)?,
				}
				
				Two(utf8_sequence) => string.push_utf8_sequence(utf8_sequence)?,
				
				Three(utf8_sequence) => string.push_utf8_sequence(utf8_sequence)?,
				
				Four(utf8_sequence) => string.push_utf8_sequence(utf8_sequence)?,
			}
		}
		
		Ok(string)
	}
}
