// This file is part of olympus-xmp. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/raphaelcohn/olympus-xmp/master/COPYRIGHT. No part of olympus-xmp, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2022 The developers of olympus-xmp. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/raphaelcohn/olympus-xmp/master/COPYRIGHT.


#[derive(Default, Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub(super) struct Names
{
	arabic: Option<&'static str>,
	
	chinese: Option<&'static str>,
	
	english: Option<&'static str>,
	
	french: Option<&'static str>,
	
	russian: Option<&'static str>,
	
	spanish: Option<&'static str>,
}

impl AsMut<Names> for Names
{
	#[inline(always)]
	fn as_mut(&mut self) -> &mut Self
	{
		self
	}
}
