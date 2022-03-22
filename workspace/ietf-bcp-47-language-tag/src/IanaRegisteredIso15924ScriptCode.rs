// This file is part of olympus-xmp. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/raphaelcohn/olympus-xmp/master/COPYRIGHT. No part of olympus-xmp, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2022 The developers of olympus-xmp. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/raphaelcohn/olympus-xmp/master/COPYRIGHT.


#[allow(missing_docs)]
#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub struct IanaRegisteredIso15924ScriptCode(UpperCaseAlpha, [Alpha; 3]);

impl Into<(UpperCaseAlpha, [Alpha; 3])> for IanaRegisteredIso15924ScriptCode
{
	#[inline(always)]
	fn into(self) -> (UpperCaseAlpha, [Alpha; 3])
	{
		(self.0, self.1)
	}
}

impl<'a> const From<&'a [u8; 4]> for IanaRegisteredIso15924ScriptCode
{
	#[inline(always)]
	fn from(value: &'a [u8; 4]) -> Self
	{
		unsafe { transmute_copy(value) }
	}
}

impl const From<[u8; 4]> for IanaRegisteredIso15924ScriptCode
{
	#[inline(always)]
	fn from(value: [u8; 4]) -> Self
	{
		unsafe { transmute(value) }
	}
}

impl const From<(UpperCaseAlpha, [Alpha; 3])> for IanaRegisteredIso15924ScriptCode
{
	#[inline(always)]
	fn from(value: (UpperCaseAlpha, [Alpha; 3])) -> Self
	{
		Self(value.0, value.1)
	}
}
