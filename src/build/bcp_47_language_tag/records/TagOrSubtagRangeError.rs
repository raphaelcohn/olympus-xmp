// This file is part of olympus-xmp. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/raphaelcohn/olympus-xmp/master/COPYRIGHT. No part of olympus-xmp, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2022 The developers of olympus-xmp. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/raphaelcohn/olympus-xmp/master/COPYRIGHT.


#[derive(Debug, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub(crate) enum TagOrSubtagRangeError
{
	RangeKeysDifferentLengths
	{
		inclusive_from: String,
		
		inclusive_to: String,
	},
	
	TypeDoesNotHaveRangeSupportImplemented
	{
		inclusive_from: String,
		
		inclusive_to: String,
	},
	
	TypeDoesNotSupportSpecificRange
	{
		inclusive_from: String,
		
		inclusive_to: String,
	},
}

impl Display for TagOrSubtagRangeError
{
	#[inline(always)]
	fn fmt(&self, formatter: &mut Formatter<'_>) -> fmt::Result
	{
		Debug::fmt(self, formatter)
	}
}

impl error::Error for TagOrSubtagRangeError
{
}
