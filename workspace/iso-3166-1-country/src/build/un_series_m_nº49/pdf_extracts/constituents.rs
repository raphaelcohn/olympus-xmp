// This file is part of olympus-xmp. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/raphaelcohn/olympus-xmp/master/COPYRIGHT. No part of olympus-xmp, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2022 The developers of olympus-xmp. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/raphaelcohn/olympus-xmp/master/COPYRIGHT.


macro_rules! constituents
{
	($first: literal$(, $subsequent: literal)*) =>
	{
		{
			const Constituents: &'static [M49Code] = &[M49Code::from($first)$(, M49Code::from($subsequent))*];
			Constituents
		}
	}
}
