// This file is part of olympus-xmp. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/raphaelcohn/olympus-xmp/master/COPYRIGHT. No part of olympus-xmp, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2022 The developers of olympus-xmp. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/raphaelcohn/olympus-xmp/master/COPYRIGHT.


/// A non-zero TIFF RATIONAL parse error.
#[derive(Debug, Clone, Eq, PartialEq)]
pub enum NonZeroUnsignedTiffRationalParseError
{
	#[allow(missing_docs)]
	TiffRational(UnsignedTiffRationalParseError),
	
	#[allow(missing_docs)]
	ZeroNumerator,
}

impl Display for NonZeroUnsignedTiffRationalParseError
{
	#[inline(always)]
	fn fmt(&self, formatter: &mut Formatter<'_>) -> fmt::Result
	{
		Debug::fmt(self, formatter)
	}
}

impl error::Error for NonZeroUnsignedTiffRationalParseError
{
	#[inline(always)]
	fn source(&self) -> Option<&(dyn error::Error + 'static)>
	{
		use NonZeroUnsignedTiffRationalParseError::*;
		match self
		{
			TiffRational(cause) => Some(cause),
			
			ZeroNumerator => None,
		}
	}
}
