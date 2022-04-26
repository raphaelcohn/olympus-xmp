// This file is part of olympus-xmp. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/raphaelcohn/olympus-xmp/master/COPYRIGHT. No part of olympus-xmp, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2022 The developers of olympus-xmp. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/raphaelcohn/olympus-xmp/master/COPYRIGHT.


#[inline(always)]
pub(crate) fn encode_utf8_percent_encoded(string: &str, percent_encode_ascii: impl Copy + FnOnce(u8) -> bool) -> Result<Cow<str>, TryReserveError>
{
	PercentEncodeUtf8::encode(string, percent_encode_ascii)
}
