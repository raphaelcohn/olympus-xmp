// This file is part of olympus-xmp. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/raphaelcohn/olympus-xmp/master/COPYRIGHT. No part of olympus-xmp, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2022 The developers of olympus-xmp. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/raphaelcohn/olympus-xmp/master/COPYRIGHT.


/// Remove prefix error.
#[derive(Debug, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub enum RemovePrefixError<'a, const PathDepth: usize>
{
	#[allow(missing_docs)]
	MissingPrefix(AbsoluteInternationalizedResourceIdentifier<'a, PathDepth>),
	
	#[allow(missing_docs)]
	Empty(AbsoluteInternationalizedResourceIdentifier<'a, PathDepth>),
	
	#[allow(missing_docs)]
	MoreThanOne(MoreThanOneError, AbsoluteInternationalizedResourceIdentifier<'a, PathDepth>)
}

impl<'a, const PathDepth: usize> Display for RemovePrefixError<'a, PathDepth>
{
	#[inline(always)]
	fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result
	{
		Debug::fmt(self, f)
	}
}

impl<'a, const PathDepth: usize> error::Error for RemovePrefixError<'a, PathDepth>
{
	#[inline(always)]
	fn source(&self) -> Option<&(dyn error::Error + 'static)>
	{
		use RemovePrefixError::*;
		
		match self
		{
			MoreThanOne(cause, _) => Some(cause),
			
			_ => None,
		}
	}
}
