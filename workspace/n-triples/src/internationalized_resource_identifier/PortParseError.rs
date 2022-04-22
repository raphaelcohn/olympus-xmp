// This file is part of olympus-xmp. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/raphaelcohn/olympus-xmp/master/COPYRIGHT. No part of olympus-xmp, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2022 The developers of olympus-xmp. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/raphaelcohn/olympus-xmp/master/COPYRIGHT.


/// A parse error.
#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub enum PortParseError
{
	#[allow(missing_docs)]
	InvalidCharacterIsNotAColon(u8),
	
	#[allow(missing_docs)]
	ValuesGreaterThan655535AreUnsupported,
	
	#[allow(missing_docs)]
	InvalidCharacterIsNotADigit(u8),
	
	/// A port number of zero is unusable with the Berkeley networking stack, as it selects a random port.
	IsZero,
	
	/// The logic limiting the number to 5 bytes (u16) is deeply embedded in the parser and should not be changed.
	IsGreaterThan65535(NonZeroU32),
	
	#[allow(missing_docs)]
	APortIsNotPermittedForThisScheme,
}

impl Display for PortParseError
{
	#[inline(always)]
	fn fmt(&self, formatter: &mut Formatter<'_>) -> fmt::Result
	{
		Debug::fmt(self, formatter)
	}
}

impl error::Error for PortParseError
{
}
