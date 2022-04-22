// This file is part of olympus-xmp. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/raphaelcohn/olympus-xmp/master/COPYRIGHT. No part of olympus-xmp, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2022 The developers of olympus-xmp. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/raphaelcohn/olympus-xmp/master/COPYRIGHT.


/// A parse error.
#[derive(Debug, Clone, Eq, PartialEq)]
pub(crate) enum OutOfMemoryOrInvalidUtf8PercentDecodeParseError
{
	#[allow(missing_docs)]
	OutOfMemory(TryReserveError),
	
	#[allow(missing_docs)]
	InvalidPercentEncodedUtf8Parse(InvalidUtf8ParseError<PercentDecodeError>),
}

impl const From<TryReserveError> for OutOfMemoryOrInvalidUtf8PercentDecodeParseError
{
	#[inline(always)]
	fn from(cause: TryReserveError) -> Self
	{
		OutOfMemoryOrInvalidUtf8PercentDecodeParseError::OutOfMemory(cause)
	}
}

impl const From<InvalidUtf8ParseError<PercentDecodeError>> for OutOfMemoryOrInvalidUtf8PercentDecodeParseError
{
	#[inline(always)]
	fn from(cause: InvalidUtf8ParseError<PercentDecodeError>) -> Self
	{
		OutOfMemoryOrInvalidUtf8PercentDecodeParseError::InvalidPercentEncodedUtf8Parse(cause)
	}
}

impl Display for OutOfMemoryOrInvalidUtf8PercentDecodeParseError
{
	#[inline(always)]
	fn fmt(&self, formatter: &mut Formatter<'_>) -> fmt::Result
	{
		Debug::fmt(self, formatter)
	}
}

impl error::Error for OutOfMemoryOrInvalidUtf8PercentDecodeParseError
{
	#[inline(always)]
	fn source(&self) -> Option<&(dyn error::Error + 'static)>
	{
		use OutOfMemoryOrInvalidUtf8PercentDecodeParseError::*;
		
		match self
		{
			OutOfMemory(cause) => Some(cause),
			
			InvalidPercentEncodedUtf8Parse(cause) => Some(cause),
		}
	}
}

impl OutOfMemoryOrInvalidUtf8PercentDecodeParseError
{
	#[inline(always)]
	pub(crate) fn into_either<E: error::Error>(self, out_of_memory: impl FnOnce(TryReserveError) -> E, invalid_percent_encoded_utf8_parse: impl FnOnce(InvalidUtf8ParseError<PercentDecodeError>) -> E) -> E
	{
		use OutOfMemoryOrInvalidUtf8PercentDecodeParseError::*;
		match self
		{
			OutOfMemory(error) => out_of_memory(error),
			
			InvalidPercentEncodedUtf8Parse(error) => invalid_percent_encoded_utf8_parse(error),
		}
	}
}
