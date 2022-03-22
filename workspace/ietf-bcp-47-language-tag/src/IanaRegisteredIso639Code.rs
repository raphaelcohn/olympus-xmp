// This file is part of olympus-xmp. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/raphaelcohn/olympus-xmp/master/COPYRIGHT. No part of olympus-xmp, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2022 The developers of olympus-xmp. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/raphaelcohn/olympus-xmp/master/COPYRIGHT.


/// Where there are 2 and 3 character ISO 639 codes, only the 2 character code will be in the IANA registry unless the 2 character code is defined by ISO 639 after the 3 character code.
/// In other words, codes will be unique.
#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub enum IanaRegisteredIso639Code
{
	#[allow(missing_docs)]
	Alpha2(IanaRegisteredIso639Alpha2Code),
	
	#[allow(missing_docs)]
	Alpha3(IanaRegisteredIso639Alpha3Code),
}

impl const From<IanaRegisteredIso639Alpha2Code> for IanaRegisteredIso639Code
{
	#[inline(always)]
	fn from(value: IanaRegisteredIso639Alpha2Code) -> Self
	{
		IanaRegisteredIso639Code::Alpha2(value)
	}
}

impl const From<IanaRegisteredIso639Alpha3Code> for IanaRegisteredIso639Code
{
	#[inline(always)]
	fn from(value: IanaRegisteredIso639Alpha3Code) -> Self
	{
		IanaRegisteredIso639Code::Alpha3(value)
	}
}

impl<'a> const From<&'a [u8; 2]> for IanaRegisteredIso639Code
{
	#[inline(always)]
	fn from(value: &'a [u8; 2]) -> Self
	{
		IanaRegisteredIso639Code::Alpha2(IanaRegisteredIso639Alpha2Code::from(value))
	}
}

impl const From<[u8; 2]> for IanaRegisteredIso639Code
{
	#[inline(always)]
	fn from(value: [u8; 2]) -> Self
	{
		IanaRegisteredIso639Code::Alpha2(IanaRegisteredIso639Alpha2Code::from(value))
	}
}

impl<'a> const From<&'a [u8; 3]> for IanaRegisteredIso639Code
{
	#[inline(always)]
	fn from(value: &'a [u8; 3]) -> Self
	{
		IanaRegisteredIso639Code::Alpha3(IanaRegisteredIso639Alpha3Code::from(value))
	}
}

impl const From<[u8; 3]> for IanaRegisteredIso639Code
{
	#[inline(always)]
	fn from(value: [u8; 3]) -> Self
	{
		IanaRegisteredIso639Code::Alpha3(IanaRegisteredIso639Alpha3Code::from(value))
	}
}
