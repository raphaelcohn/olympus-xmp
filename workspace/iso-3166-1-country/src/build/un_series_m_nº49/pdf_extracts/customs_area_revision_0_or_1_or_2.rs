// This file is part of olympus-xmp. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/raphaelcohn/olympus-xmp/master/COPYRIGHT. No part of olympus-xmp, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2022 The developers of olympus-xmp. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/raphaelcohn/olympus-xmp/master/COPYRIGHT.


#[inline(always)]
const fn customs_area_revision_0_or_1_or_2(m49_code: &'static [u8; 3], english_name: &'static str, constitutents: &'static [M49Code]) -> (M49Code, &'static str, &'static [M49Code])
{
	let code = M49Code::from(m49_code);
	
	// Weird but official!
	if code != constitutents[0]
	{
		panic!("First constituent must be the customs code")
	}
	(code, english_name, constitutents)
}
