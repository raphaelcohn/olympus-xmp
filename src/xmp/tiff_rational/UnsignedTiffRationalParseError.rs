// This file is part of olympus-xmp. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/raphaelcohn/olympus-xmp/master/COPYRIGHT. No part of olympus-xmp, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2022 The developers of olympus-xmp. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/raphaelcohn/olympus-xmp/master/COPYRIGHT.


/// A TIFF RATIONAL parse error.
#[derive(Debug, Clone, Eq, PartialEq)]
pub enum UnsignedTiffRationalParseError
{
	#[allow(missing_docs)]
	MissingDenominator,
	
	#[allow(missing_docs)]
	InvalidU32Numerator(ParseIntError),
	
	#[allow(missing_docs)]
	InvalidNonZeroU32Denominator(ParseIntError),
}

impl Display for UnsignedTiffRationalParseError
{
	#[inline(always)]
	fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result
	{
		Debug::fmt(self, f)
	}
}

impl error::Error for UnsignedTiffRationalParseError
{
	#[inline(always)]
	fn source(&self) -> Option<&(dyn error::Error + 'static)>
	{
		use UnsignedTiffRationalParseError::*;
		match self
		{
			MissingDenominator => None,
			
			InvalidU32Numerator(cause) => Some(cause),
			
			InvalidNonZeroU32Denominator(cause) => Some(cause),
		}
	}
}
