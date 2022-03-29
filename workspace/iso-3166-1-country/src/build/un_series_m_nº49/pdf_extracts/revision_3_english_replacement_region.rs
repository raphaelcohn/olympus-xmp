// This file is part of olympus-xmp. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/raphaelcohn/olympus-xmp/master/COPYRIGHT. No part of olympus-xmp, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2022 The developers of olympus-xmp. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/raphaelcohn/olympus-xmp/master/COPYRIGHT.


#[inline(always)]
const fn revision_3_english_replacement_region(old_code: &'static [u8; 3], english_name: &'static str, new_code: &'static [u8; 3]) -> (M49Code, &'static str, M49Code)
{
	(M49Code::from(old_code), english_name, M49Code::from(new_code))
}
