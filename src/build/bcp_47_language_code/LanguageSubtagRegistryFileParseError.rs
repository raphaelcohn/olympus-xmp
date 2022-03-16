// This file is part of olympus-xmp. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/raphaelcohn/olympus-xmp/master/COPYRIGHT. No part of olympus-xmp, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2022 The developers of olympus-xmp. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/raphaelcohn/olympus-xmp/master/COPYRIGHT.


#[derive(Debug)]
pub(crate) enum LanguageSubtagRegistryFileParseError
{
	CouldNotOpenFile(io::Error),
	
	CouldNotParseKeyValueLine(PullEventParserError),
	
	CouldParseRecordsFileHeader(RecordsFileHeaderParseError),
	
	Field(FieldError),
	
	MissingTypeField,
	
	Record(Type, RecordParseError),
}

impl From<PullEventParserError> for LanguageSubtagRegistryFileParseError
{
	#[inline(always)]
	fn from(cause: PullEventParserError) -> Self
	{
		LanguageSubtagRegistryFileParseError::CouldNotParseKeyValueLine(cause)
	}
}

impl From<RecordsFileHeaderParseError> for LanguageSubtagRegistryFileParseError
{
	#[inline(always)]
	fn from(cause: RecordsFileHeaderParseError) -> Self
	{
		LanguageSubtagRegistryFileParseError::CouldParseRecordsFileHeader(cause)
	}
}

impl From<FieldError> for LanguageSubtagRegistryFileParseError
{
	#[inline(always)]
	fn from(cause: FieldError) -> Self
	{
		LanguageSubtagRegistryFileParseError::Field(cause)
	}
}

impl Display for LanguageSubtagRegistryFileParseError
{
	#[inline(always)]
	fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result
	{
		Debug::fmt(self, f)
	}
}

impl error::Error for LanguageSubtagRegistryFileParseError
{
	fn source(&self) -> Option<&(dyn error::Error + 'static)>
	{
		use LanguageSubtagRegistryFileParseError::*;
		
		match self
		{
			CouldNotOpenFile(cause) => Some(cause),
			
			CouldNotParseKeyValueLine(cause) => Some(cause),
			
			CouldParseRecordsFileHeader(cause) => Some(cause),
			
			Field(cause) => Some(cause),
			
			MissingTypeField => None,
			
			Record(_, cause) => Some(cause),
		}
	}
}
