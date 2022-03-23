// This file is part of olympus-xmp. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/raphaelcohn/olympus-xmp/master/COPYRIGHT. No part of olympus-xmp, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2022 The developers of olympus-xmp. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/raphaelcohn/olympus-xmp/master/COPYRIGHT.


#[allow(missing_docs)]
#[derive(Debug, Clone, Eq, PartialEq)]
pub enum LanguageSubtagRegistryFileContentsParseError
{
	CouldNotParseKeyValueLine(PullEventParserError),
	
	CouldParseRecordsFileHeader(RecordsFileHeaderParseError),
	
	Field(FieldError),
	
	MissingTypeField,
	
	Record(Type, RecordParseError),
}

impl const From<PullEventParserError> for LanguageSubtagRegistryFileContentsParseError
{
	#[inline(always)]
	fn from(cause: PullEventParserError) -> Self
	{
		LanguageSubtagRegistryFileContentsParseError::CouldNotParseKeyValueLine(cause)
	}
}

impl const From<RecordsFileHeaderParseError> for LanguageSubtagRegistryFileContentsParseError
{
	#[inline(always)]
	fn from(cause: RecordsFileHeaderParseError) -> Self
	{
		LanguageSubtagRegistryFileContentsParseError::CouldParseRecordsFileHeader(cause)
	}
}

impl const From<FieldError> for LanguageSubtagRegistryFileContentsParseError
{
	#[inline(always)]
	fn from(cause: FieldError) -> Self
	{
		LanguageSubtagRegistryFileContentsParseError::Field(cause)
	}
}

impl Display for LanguageSubtagRegistryFileContentsParseError
{
	#[inline(always)]
	fn fmt(&self, formatter: &mut Formatter<'_>) -> fmt::Result
	{
		Debug::fmt(self, formatter)
	}
}

impl error::Error for LanguageSubtagRegistryFileContentsParseError
{
	fn source(&self) -> Option<&(dyn error::Error + 'static)>
	{
		use LanguageSubtagRegistryFileContentsParseError::*;
		
		match self
		{
			CouldNotParseKeyValueLine(cause) => Some(cause),
			
			CouldParseRecordsFileHeader(cause) => Some(cause),
			
			Field(cause) => Some(cause),
			
			MissingTypeField => None,
			
			Record(_, cause) => Some(cause),
		}
	}
}
