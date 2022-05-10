// This file is part of olympus-xmp. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/raphaelcohn/olympus-xmp/master/COPYRIGHT. No part of olympus-xmp, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2022 The developers of olympus-xmp. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/raphaelcohn/olympus-xmp/master/COPYRIGHT.


/// A parse error.
#[derive(Debug, Clone, Eq, PartialEq)]
pub enum StringLiteralParseError
{
	#[allow(missing_docs)]
	InvalidUtf8Parse(InvalidUtf8ParseError<Infallible>),
	
	#[allow(missing_docs)]
	DidNotExpectEndParsingBody,
	
	#[allow(missing_docs)]
	DidNotExpectEndParsingLiteralTag,
	
	#[allow(missing_docs)]
	DidNotExpectEndParsingSecondCaret,
	
	#[allow(missing_docs)]
	DidNotExpectEndParsingOpenAngleBracket,
	
	#[allow(missing_docs)]
	LiteralTagSecondCaretNotFollowedByOpenAngleBracket(u8),
	
	#[allow(missing_docs)]
	DidNotExpectEndParsingLanguageTag,
	
	#[allow(missing_docs)]
	InvalidCharacter(char),
	
	#[allow(missing_docs)]
	OutOfMemory(TryReserveError),
	
	#[allow(missing_docs)]
	EndOfFileParsingEscapeSequence,
	
	#[allow(missing_docs)]
	InvalidUCHAR4EscapeSequence(UCHARParseError),
	
	#[allow(missing_docs)]
	InvalidUCHAR8EscapeSequence(UCHARParseError),
	
	#[allow(missing_docs)]
	InvalidEscapeSequence(u8),
	
	#[allow(missing_docs)]
	InvalidByteStartsLiteralTag(u8),
	
	#[allow(missing_docs)]
	LiteralTagCaretNotFollowedByCaret(u8),
	
	#[allow(missing_docs)]
	InternationalizedResourceIdentifierParseLiteralTagParse(AbsoluteInternationalizedResourceIdentifierParseError),
	
	#[allow(missing_docs)]
	NaiveIetfBcp47LanguageTagParse(NaiveIetfBcp47LanguageTagParseError),
}

impl const From<InvalidUtf8ParseError<Infallible>> for StringLiteralParseError
{
	#[inline(always)]
	fn from(cause: InvalidUtf8ParseError<Infallible>) -> Self
	{
		StringLiteralParseError::InvalidUtf8Parse(cause)
	}
}

impl const From<TryReserveError> for StringLiteralParseError
{
	#[inline(always)]
	fn from(cause: TryReserveError) -> Self
	{
		StringLiteralParseError::OutOfMemory(cause)
	}
}

impl Display for StringLiteralParseError
{
	#[inline(always)]
	fn fmt(&self, formatter: &mut Formatter<'_>) -> fmt::Result
	{
		Debug::fmt(self, formatter)
	}
}

impl error::Error for StringLiteralParseError
{
	#[inline(always)]
	fn source(&self) -> Option<&(dyn error::Error + 'static)>
	{
		use StringLiteralParseError::*;
		
		match self
		{
			InvalidUtf8Parse(cause) => Some(cause),
			
			OutOfMemory(cause) => Some(cause),
			
			InvalidUCHAR4EscapeSequence(cause) => Some(cause),
			
			InvalidUCHAR8EscapeSequence(cause) => Some(cause),
			
			InternationalizedResourceIdentifierParseLiteralTagParse(cause) => Some(cause),
			
			NaiveIetfBcp47LanguageTagParse(cause) => Some(cause),
			
			_ => None,
		}
	}
}
