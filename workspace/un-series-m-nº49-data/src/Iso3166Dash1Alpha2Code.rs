// This file is part of olympus-xmp. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/raphaelcohn/olympus-xmp/master/COPYRIGHT. No part of olympus-xmp, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2022 The developers of olympus-xmp. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/raphaelcohn/olympus-xmp/master/COPYRIGHT.


#[allow(missing_docs)]
#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub struct Iso3166Dash1Alpha2Code([u8; 2]);

impl Debug for Iso3166Dash1Alpha2Code
{
	#[inline(always)]
	fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result
	{
		let array = self.0;
		write!(f, "{}{}", array[0] as char, array[1] as char)
	}
}

impl Display for Iso3166Dash1Alpha2Code
{
	#[inline(always)]
	fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result
	{
		Debug::fmt(self, f)
	}
}

impl const From<StaticIso3166Dash1Alpha2Code> for Iso3166Dash1Alpha2Code
{
	#[inline(always)]
	fn from(iso_3166_dash_1_alpha2_code: StaticIso3166Dash1Alpha2Code) -> Self
	{
		Self(*iso_3166_dash_1_alpha2_code)
	}
}
