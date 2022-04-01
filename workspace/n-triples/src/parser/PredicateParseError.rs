// This file is part of olympus-xmp. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/raphaelcohn/olympus-xmp/master/COPYRIGHT. No part of olympus-xmp, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2022 The developers of olympus-xmp. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/raphaelcohn/olympus-xmp/master/COPYRIGHT.


/// Parse error.
#[derive(Debug, Clone, Eq, PartialEq)]
pub enum PredicateParseError
{
	#[allow(missing_docs)]
	PredicateParse(IRIParseError),
	
	#[allow(missing_docs)]
	CanNotStartWith(u8),
	
	#[allow(missing_docs)]
	ALineMustContinueWithAPredicate,
}

impl Display for PredicateParseError
{
	#[inline(always)]
	fn fmt(&self, formatter: &mut Formatter<'_>) -> fmt::Result
	{
		Debug::fmt(self, formatter)
	}
}

impl error::Error for PredicateParseError
{
	#[inline(always)]
	fn source(&self) -> Option<&(dyn error::Error + 'static)>
	{
		use PredicateParseError::*;
		
		match self
		{
			PredicateParse(cause) => Some(cause),
			
			_ => None,
		}
	}
}
