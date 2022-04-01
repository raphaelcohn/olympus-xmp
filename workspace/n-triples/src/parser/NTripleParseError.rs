// This file is part of olympus-xmp. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/raphaelcohn/olympus-xmp/master/COPYRIGHT. No part of olympus-xmp, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2022 The developers of olympus-xmp. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/raphaelcohn/olympus-xmp/master/COPYRIGHT.


/// Parse error.
#[derive(Debug, Clone, Eq, PartialEq)]
pub enum NTripleParseError
{
	#[allow(missing_docs)]
	SubjectParse(SubjectParseError),
	
	#[allow(missing_docs)]
	PredicateParse(PredicateParseError),
	
	#[allow(missing_docs)]
	ObjectParse(ObjectParseError),
	
	#[allow(missing_docs)]
	PeriodParse(PeriodParseError),
}

impl Display for NTripleParseError
{
	#[inline(always)]
	fn fmt(&self, formatter: &mut Formatter<'_>) -> fmt::Result
	{
		Debug::fmt(self, formatter)
	}
}

impl error::Error for NTripleParseError
{
	#[inline(always)]
	fn source(&self) -> Option<&(dyn error::Error + 'static)>
	{
		use NTripleParseError::*;
		
		match self
		{
			SubjectParse(cause) => Some(cause),
			
			PredicateParse(cause) => Some(cause),
			
			ObjectParse(cause) => Some(cause),
			
			PeriodParse(cause) => Some(cause),
			
			_ => None,
		}
	}
}
