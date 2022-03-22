// This file is part of olympus-xmp. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/raphaelcohn/olympus-xmp/master/COPYRIGHT. No part of olympus-xmp, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2022 The developers of olympus-xmp. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/raphaelcohn/olympus-xmp/master/COPYRIGHT.


/// Month (one based).
#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
#[repr(u8)]
pub enum Month
{
	January = 1,
	
	February = 2,
	
	March = 3,
	
	April = 4,
	
	May = 5,
	
	June = 6,
	
	July = 7,
	
	August = 8,
	
	September = 9,
	
	October = 10,
	
	November = 11,
	
	December = 12,
}

impl FromStr for Month
{
	type Err = DateParseError;
	
	fn from_str(s: &str) -> Result<Self, Self::Err>
	{
		use DateParseError::*;
		use Month::*;
		
		match s
		{
			"01" => Ok(January),
			
			"02" => Ok(February),
			
			"03" => Ok(March),
			
			"04" => Ok(April),
			
			"05" => Ok(May),
			
			"06" => Ok(June),
			
			"07" => Ok(July),
			
			"08" => Ok(August),
			
			"09" => Ok(September),
			
			"10" => Ok(October),
			
			"11" => Ok(November),
			
			"12" => Ok(December),
			
			_ => if s.len() == 2
			{
				Err(MonthStringInvalid)
			}
			else
			{
				Err(MonthStringLengthNot2)
			},
		}
	}
}
