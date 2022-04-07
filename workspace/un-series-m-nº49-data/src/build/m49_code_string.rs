// This file is part of olympus-xmp. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/raphaelcohn/olympus-xmp/master/COPYRIGHT. No part of olympus-xmp, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2022 The developers of olympus-xmp. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/raphaelcohn/olympus-xmp/master/COPYRIGHT.


#[inline(always)]
fn m49_code_string(m49_code: M49CodeArray) -> String
{
	#[inline(always)]
	fn char<const index: u8>(m49_code: M49CodeArray) -> char
	{
		m49_code.get_unchecked_value_safe(index) as char
	}
	
	format!("{}{}{}", char::<0>(m49_code), char::<1>(m49_code), char::<2>(m49_code))
}
