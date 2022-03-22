// This file is part of olympus-xmp. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/raphaelcohn/olympus-xmp/master/COPYRIGHT. No part of olympus-xmp, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2022 The developers of olympus-xmp. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/raphaelcohn/olympus-xmp/master/COPYRIGHT.


#[allow(missing_docs)]
#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub(super) struct InvalidSingletonError
{
	pub(crate) length: usize,
	
	pub(crate) index: usize,
	
	pub(crate) byte: u8,
}

impl Display for InvalidSingletonError
{
	#[inline(always)]
	fn fmt(&self, formatter: &mut Formatter<'_>) -> fmt::Result
	{
		Debug::fmt(self, formatter)
	}
}

impl error::Error for InvalidSingletonError
{
}
