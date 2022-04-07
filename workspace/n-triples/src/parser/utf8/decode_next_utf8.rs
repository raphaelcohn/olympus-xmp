// This file is part of olympus-xmp. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/raphaelcohn/olympus-xmp/master/COPYRIGHT. No part of olympus-xmp, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2022 The developers of olympus-xmp. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/raphaelcohn/olympus-xmp/master/COPYRIGHT.


#[inline(always)]
pub(crate) fn decode_next_utf8(remaining_bytes: &mut &[u8]) -> Result<Option<(char, Utf8CharacterLength)>, InvalidUtf8ParseError<Infallible>>
{
	BytesByteProvider::decode_next_utf8(remaining_bytes)
}
