// This file is part of olympus-xmp. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/raphaelcohn/olympus-xmp/master/COPYRIGHT. No part of olympus-xmp, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2022 The developers of olympus-xmp. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/raphaelcohn/olympus-xmp/master/COPYRIGHT.


#[derive(Debug, Clone, Eq, PartialEq)]
pub(crate) enum RecordsFileHeaderParseError
{
	MissingFirstLine,
	
	FileDateShouldBeFirstLine,
	
	FileDateShouldBeFirstLineNotNewRecord,
	
	MissingSecondLine,
	
	SecondLineIsNotStartRecord,
	
	PullEvent(PullEventParserError),

	FileDate(DateParseError),
}

impl From<PullEventParserError> for RecordsFileHeaderParseError
{
	#[inline(always)]
	fn from(cause: PullEventParserError) -> Self
	{
		RecordsFileHeaderParseError::PullEvent(cause)
	}
}

impl From<DateParseError> for RecordsFileHeaderParseError
{
	#[inline(always)]
	fn from(cause: DateParseError) -> Self
	{
		RecordsFileHeaderParseError::FileDate(cause)
	}
}

impl Display for RecordsFileHeaderParseError
{
	#[inline(always)]
	fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result
	{
		Debug::fmt(self, f)
	}
}

impl error::Error for RecordsFileHeaderParseError
{
	fn source(&self) -> Option<&(dyn error::Error + 'static)>
	{
		use RecordsFileHeaderParseError::*;
		
		match self
		{
			PullEvent(cause) => Some(cause),
			
			FileDate(cause) => Some(cause),
			
			_ => None,
		}
	}
}
