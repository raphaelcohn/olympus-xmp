// This file is part of olympus-xmp. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/raphaelcohn/olympus-xmp/master/COPYRIGHT. No part of olympus-xmp, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2022 The developers of olympus-xmp. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/raphaelcohn/olympus-xmp/master/COPYRIGHT.


#[inline(always)]
pub(crate) fn decode_next_utf8_validity_already_checked(remaining_utf8_bytes: &mut &[u8]) -> Option<(char, Utf8CharacterLength)>
{
	BytesByteProvider::decode_next_utf8_validity_already_checked(remaining_utf8_bytes)
}
