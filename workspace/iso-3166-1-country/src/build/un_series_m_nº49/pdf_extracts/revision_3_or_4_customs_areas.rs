// This file is part of olympus-xmp. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/raphaelcohn/olympus-xmp/master/COPYRIGHT. No part of olympus-xmp, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2022 The developers of olympus-xmp. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/raphaelcohn/olympus-xmp/master/COPYRIGHT.


#[inline(always)]
const fn revision_3_or_4_customs_areas(code: &[u8; 3], english_name: &'static str, constituents: &'static [M49Code]) -> (M49Code, &'static str, &'static [M49Code])
{
	(M49Code::from(code), english_name, constituents)
}
