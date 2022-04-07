// This file is part of olympus-xmp. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/raphaelcohn/olympus-xmp/master/COPYRIGHT. No part of olympus-xmp, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2022 The developers of olympus-xmp. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/raphaelcohn/olympus-xmp/master/COPYRIGHT.


/// Represents a `STRING_LITERAL_QUOTE`.
#[derive(Debug, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub(super) struct StringLiteral<'a>
{
	pub(super) literal_value: Cow<'a, str>,
	
	pub(super) literal_tag: LiteralTag<'a>
}

impl<'a> StringLiteral<'a>
{
	#[inline(always)]
	fn parse<R>(remaining_bytes: &mut &'a [u8], constructor: impl FnOnce(Self) -> R) -> Result<R, StringLiteralParseError>
	{
		use StringLiteralParseError::*;
		
		let mut string = StringSoFar::new_stack(remaining_bytes);
		
		loop
		{
			const xA: char = 0xA as char;
			const xD: char = 0xD as char;
			
			let (character, utf8_character_length) = decode_next_utf8(remaining_bytes)?.ok_or(DidNotExpectEndParsingBody)?;
			match character
			{
				'"' => break,
				
				'\\' => match get_0(remaining_bytes).ok_or(EndOfFileParsingEscapeSequence)?
				{
					// TODO: This is wrong... we can't push a slice.
					t => string.push_forcing_heap_ascii('\t')?,
					
					b => string.push_forcing_heap_ascii('\x08')?,
					
					n => string.push_forcing_heap_ascii('\n')?,
					
					r => string.push_forcing_heap_ascii('\r')?,
					
					f => string.push_forcing_heap_ascii('\x0A')?,
					
					b'"' => string.push_forcing_heap_ascii('"')?,
					
					b'\\' => string.push_forcing_heap_ascii('\'')?,
					
					u => string.push_forcing_heap_UCHAR4(remaining_bytes).map_err(InvalidUCHAR4EscapeSequence)?,
					
					U => string.push_forcing_heap_UCHAR8(remaining_bytes).map_err(InvalidUCHAR8EscapeSequence)?,
					
					invalid => return Err(InvalidEscapeSequence(invalid)),
				},
				
				invalid @ (xA | xD) => return Err(InvalidCharacter(invalid)),
				
				character @ _ => string.push(character, utf8_character_length)?,
			}
		}
		
		use LiteralTag::*;
		let literal_tag = match get_0(remaining_bytes).ok_or(DidNotExpectEndParsingLiteralTag)?
		{
			Space | Tab => Datatype(AbsoluteInternationalizedResourceIdentifier::Simple),
			
			Caret =>
			{
				let subsequent = get_0(remaining_bytes).ok_or(DidNotExpectEndParsingSecondCaret)?;
				if subsequent != Caret
				{
					return Err(LiteralTagCaretNotFollowedByCaret(subsequent))
				}
				let subsequent = get_0(remaining_bytes).ok_or(DidNotExpectEndParsingOpenAngleBracket)?;
				if subsequent != OpenAngleBracket
				{
					return Err(LiteralTagSecondCaretNotFollowedByOpenAngleBracket(subsequent))
				}
				AbsoluteInternationalizedResourceIdentifier::parse(remaining_bytes, Datatype).map_err(InternationalizedResourceIdentifierParseLiteralTagParse)?
			},
			
			AtSign =>
			{
				let haystack = *remaining_bytes;
				let index = memchr2(Space, Tab, haystack).ok_or(DidNotExpectEndParsingLanguageTag)?;
				
				// TODO: Parse the raw language tag...
				// `LANGTAG ::= '@' [a-zA-Z]+ ('-' [a-zA-Z0-9]+)*`.
				let raw_ietf_bcp_47_language_tag_bytes = haystack.get_unchecked_range_safe( .. index);
				*remaining_bytes = haystack.after_index(index);
				
				// TODO: Replace with simdutf8
				xxxx;
				Language(Cow::Borrowed(from_utf8(raw_ietf_bcp_47_language_tag_bytes).map_err(InvalidLanguageTag)?))
			}
			
			invalid @ _ => return Err(InvalidByteStartsLiteralTag(invalid)),
		};
		
		
		Ok
		(
			constructor
			(
				Self
				{
					literal_value: string.to_cow(),
				
					literal_tag,
				}
			)
		)
	}
}