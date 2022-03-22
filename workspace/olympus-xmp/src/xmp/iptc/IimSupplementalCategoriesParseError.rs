// This file is part of olympus-xmp. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/raphaelcohn/olympus-xmp/master/COPYRIGHT. No part of olympus-xmp, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2022 The developers of olympus-xmp. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/raphaelcohn/olympus-xmp/master/COPYRIGHT.


/// Obsolete as of IIM version 4.
#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub enum IimSupplementalCategoriesParseError
{
	#[allow(missing_docs)]
	InvalidSupplementalCategory
	{
		index: usize,
		
		cause: IimCategoryCodeParseError,
	},
	
	#[allow(missing_docs)]
	MoreThanEightSupplementalCategories,
}

impl Display for IimSupplementalCategoriesParseError
{
	#[inline(always)]
	fn fmt(&self, formatter: &mut Formatter<'_>) -> fmt::Result
	{
		Debug::fmt(self, formatter)
	}
}

impl error::Error for IimSupplementalCategoriesParseError
{
	#[inline(always)]
	fn source(&self) -> Option<&(dyn error::Error + 'static)>
	{
		use IimSupplementalCategoriesParseError::*;
		match self
		{
			InvalidSupplementalCategory { cause, .. } => Some(cause),
			
			_ => None,
		}
	}
}
