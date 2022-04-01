// This file is part of olympus-xmp. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/raphaelcohn/olympus-xmp/master/COPYRIGHT. No part of olympus-xmp, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2022 The developers of olympus-xmp. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/raphaelcohn/olympus-xmp/master/COPYRIGHT.


use std::collections::TryReserveError;
use std::mem::size_of;
use swiss_army_knife::get_unchecked::GetUnchecked;
use super::InvalidUtf8ParseError;


include!("decode_next_utf8.rs");
include!("encode_utf8_not_reserving_space.rs");
include!("encode_utf8_reserving_space.rs");
include!("EncodeUtf8.rs");
