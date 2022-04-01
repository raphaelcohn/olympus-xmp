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
		
		let mut string = StringSoFar::initial(remaining_bytes);
		
		loop
		{
			const xA: char = 0xA as char;
			const xD: char = 0xD as char;
			
			match decode_next_utf8(remaining_bytes)?.ok_or(DidNotExpectEndParsingBody)?
			{
				'"' => break,
				
				'\\' => match get_0(remaining_bytes).ok_or(EndOfFileParsingEscapeSequence)?
				{
					t => string.push_ascii('\t')?,
					
					b => string.push_ascii('\x08')?,
					
					n => string.push_ascii('\n')?,
					
					r => string.push_ascii('\r')?,
					
					f => string.push_ascii('\x0A')?,
					
					b'"' => string.push_ascii('"')?,
					
					b'\\' => string.push_ascii('\'')?,
					
					u => string.push_forcing_heap_UCHAR4(remaining_bytes).map_err(InvalidUCHAR4EscapeSequence)?,
					
					U => string.push_forcing_heap_UCHAR8(remaining_bytes).map_err(InvalidUCHAR8EscapeSequence)?,
					
					invalid => return Err(InvalidEscapeSequence(invalid)),
				},
				
				invalid @ (xA | xD) => return Err(InvalidCharacter(invalid)),
				
				character @ _ => string.push(character)?,
			}
		}
		
		use LiteralTag::*;
		let literal_tag = match get_0(remaining_bytes).ok_or(DidNotExpectEndParsingLiteralTag)?
		{
			Space | Tab => Datatype(IRI::Simple),
			
			Caret =>
			{
				let subsequent = get_0(remaining_bytes).ok_or(DidNotExpectEndParsingSecondCaret)?;
				if subsequent != Caret
				{
					return Err(LiteralTagCaretNotFollowedByCaret(subsequent))
				}
				
				IRI::parse(remaining_bytes, Datatype).map_err(IRILiteralTagParse)?
			},
			
			AtSign =>
			{
				let haystack = *remaining_bytes;
				let index = memchr2(Space, Tab, haystack).ok_or(DidNotExpectEndParsingLanguageTag)?;
				
				// TODO: Parse the raw language tag...
				// `LANGTAG ::= '@' [a-zA-Z]+ ('-' [a-zA-Z0-9]+)*`.
				let raw_ietf_bcp_47_language_tag = haystack.get_unchecked_range_safe( .. index);
				*remaining_bytes = haystack.get_unchecked_range_safe((index + 1) .. );
				Language(raw_ietf_bcp_47_language_tag)
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
