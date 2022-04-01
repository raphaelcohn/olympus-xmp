// This file is part of olympus-xmp. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/raphaelcohn/olympus-xmp/master/COPYRIGHT. No part of olympus-xmp, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2022 The developers of olympus-xmp. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/raphaelcohn/olympus-xmp/master/COPYRIGHT.


/// Parse error.
#[derive(Debug, Clone, Eq, PartialEq)]
pub enum SubjectParseError
{
	#[allow(missing_docs)]
	IRIParse(IRIParseError),
	
	#[allow(missing_docs)]
	BlankNodeLabelParse(BlankNodeLabelParseError),
	
	#[allow(missing_docs)]
	CanNotStartWith(u8),
	
	#[allow(missing_docs)]
	ALineMustStartWithASubject,
}

impl Display for SubjectParseError
{
	#[inline(always)]
	fn fmt(&self, formatter: &mut Formatter<'_>) -> fmt::Result
	{
		Debug::fmt(self, formatter)
	}
}

impl error::Error for SubjectParseError
{
	#[inline(always)]
	fn source(&self) -> Option<&(dyn error::Error + 'static)>
	{
		use SubjectParseError::*;
		
		match self
		{
			IRIParse(cause) => Some(cause),
			
			BlankNodeLabelParse(cause) => Some(cause),
			
			_ => None,
		}
	}
}
