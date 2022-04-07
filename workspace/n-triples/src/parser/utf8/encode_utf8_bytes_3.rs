// This file is part of olympus-xmp. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/raphaelcohn/olympus-xmp/master/COPYRIGHT. No part of olympus-xmp, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2022 The developers of olympus-xmp. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/raphaelcohn/olympus-xmp/master/COPYRIGHT.


#[inline(always)]
pub(super) const fn encode_utf8_bytes_3(code: u32) -> [u8; 3]
{
	[
		(code >> Shift12 & x0F) as u8 | TAG_THREE_B,
		(code >> Shift6 & x3F) as u8 | TAG_CONT,
		(code & x3F) as u8 | TAG_CONT
	]
}