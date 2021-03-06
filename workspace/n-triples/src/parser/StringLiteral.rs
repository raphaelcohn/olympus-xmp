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
		
		let mut string = Utf8SequencesParser::new_stack(remaining_bytes);
		
		loop
		{
			let Utf8SequenceAndCharacter(utf8_sequence, _) = remaining_bytes.decode_next_utf8()?.ok_or(DidNotExpectEndParsingBody)?;
			
			use Utf8SequenceEnum::*;
			
			match utf8_sequence
			{
				One([DoubleQuote]) => break,
				
				One([Backslash]) => match remaining_bytes.pop_first_or_error(EndOfFileParsingEscapeSequence)?
				{
					t => string.push_forcing_heap_ascii_byte::<false>(Tab)?,
					
					b => string.push_forcing_heap_ascii_byte::<false>(Backspace)?,
					
					n => string.push_forcing_heap_ascii_byte::<false>(LineFeed)?,
					
					r => string.push_forcing_heap_ascii_byte::<false>(CarriageReturn)?,
					
					f => string.push_forcing_heap_ascii_byte::<false>(FormFeed)?,
					
					DoubleQuote => string.push_forcing_heap_ascii_byte::<false>(DoubleQuote)?,
					
					Backslash => string.push_forcing_heap_ascii_byte::<false>(Backslash)?,
					
					u => string.push_forcing_heap_UCHAR4(remaining_bytes,InvalidUCHAR4EscapeSequence, OutOfMemory)?,
					
					U => string.push_forcing_heap_UCHAR8(remaining_bytes,InvalidUCHAR8EscapeSequence, OutOfMemory)?,
					
					invalid @ _ => return Err(InvalidEscapeSequence(invalid)),
				},
				
				One([LineFeed]) => return Err(InvalidCharacter(LineFeed as char)),
				
				One([CarriageReturn]) => return Err(InvalidCharacter(CarriageReturn as char)),
				
				One([ascii_byte]) => string.push_ascii_byte(ascii_byte)?,
				
				Two(utf8_sequence) => string.push_utf8_sequence(utf8_sequence)?,
				
				Three(utf8_sequence) => string.push_utf8_sequence(utf8_sequence)?,
				
				Four(utf8_sequence) => string.push_utf8_sequence(utf8_sequence)?,
			}
		}
		
		use LiteralTag::*;
		let literal_tag = match remaining_bytes.pop_first_or_error(DidNotExpectEndParsingLiteralTag)?
		{
			Space | Tab => Datatype(AbsoluteInternationalizedResourceIdentifier::http_www_w3_org_2001_xml_schema("string")),
			
			Caret =>
			{
				let subsequent = remaining_bytes.pop_first_or_error(DidNotExpectEndParsingSecondCaret)?;
				if subsequent != Caret
				{
					return Err(LiteralTagCaretNotFollowedByCaret(subsequent))
				}
				let subsequent = remaining_bytes.pop_first_or_error(DidNotExpectEndParsingOpenAngleBracket)?;
				if subsequent != OpenAngleBracket
				{
					return Err(LiteralTagSecondCaretNotFollowedByOpenAngleBracket(subsequent))
				}
				AbsoluteInternationalizedResourceIdentifier::parse(remaining_bytes, Datatype).map_err(InternationalizedResourceIdentifierParseLiteralTagParse)?
			},
			
			AtSign =>
			{
				let haystack = *remaining_bytes;
				let index = haystack.memchr2(Space, Tab).ok_or(DidNotExpectEndParsingLanguageTag)?;
				
				let naive_ietf_bcp_47_language_tag_bytes = haystack.before_index(index);
				*remaining_bytes = haystack.after_index(index);
				
				// 	let language_tag = NaiveIetfBcp47LanguageTag::parse(haystack.after_index(index)).map_err(NaiveIetfBcp47LanguageTagParse)?;
				Language(NaiveIetfBcp47LanguageTag::parse(naive_ietf_bcp_47_language_tag_bytes).map_err(NaiveIetfBcp47LanguageTagParse)?)
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
