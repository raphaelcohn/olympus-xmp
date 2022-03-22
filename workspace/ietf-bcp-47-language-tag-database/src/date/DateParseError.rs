// This file is part of olympus-xmp. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/raphaelcohn/olympus-xmp/master/COPYRIGHT. No part of olympus-xmp, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2022 The developers of olympus-xmp. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/raphaelcohn/olympus-xmp/master/COPYRIGHT.


#[allow(missing_docs)]
#[derive(Debug, Clone, Eq, PartialEq)]
pub enum DateParseError
{
	YearStringLengthNot4,
	
	YearStringInvalid(ParseIntError),
	
	MissingMonth,
	
	MonthStringLengthNot2,
	
	MonthStringInvalid,

	MissingDayOfMonth,
	
	DayOfMonthStringLengthNot2,
	
	DayOfMonthStringInvalid,
	
	TooManyDaysForMonth,
	
	TooManyHyphens,
}

impl Display for DateParseError
{
	#[inline(always)]
	fn fmt(&self, formatter: &mut Formatter<'_>) -> fmt::Result
	{
		Debug::fmt(self, formatter)
	}
}

impl error::Error for DateParseError
{
	#[inline(always)]
	fn source(&self) -> Option<&(dyn error::Error + 'static)>
	{
		use DateParseError::*;
		match self
		{
			YearStringInvalid(cause) => Some(cause),
			
			_ => None,
		}
	}
}
