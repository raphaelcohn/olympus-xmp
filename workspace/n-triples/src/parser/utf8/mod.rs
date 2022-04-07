// This file is part of olympus-xmp. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/raphaelcohn/olympus-xmp/master/COPYRIGHT. No part of olympus-xmp, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2022 The developers of olympus-xmp. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/raphaelcohn/olympus-xmp/master/COPYRIGHT.


use Utf8CharacterLength::*;
use std::collections::TryReserveError;
use std::convert::Infallible;
use std::error;
use std::fmt;
use std::fmt::Debug;
use std::fmt::Display;
use std::fmt::Formatter;
use std::mem::size_of;
use std::num::NonZeroU8;
use std::num::NonZeroUsize;
use swiss_army_knife::a_to_z::_0;
use swiss_army_knife::a_to_z::_9;
use swiss_army_knife::a_to_z::A;
use swiss_army_knife::a_to_z::F;
use swiss_army_knife::a_to_z::a;
use swiss_army_knife::a_to_z::f;
use swiss_army_knife::get_unchecked::GetUnchecked;
use swiss_army_knife::non_zero::new_non_zero_u8;
use swiss_army_knife::non_zero::new_non_zero_usize;
use super::get_0;
use super::InvalidUtf8ParseError;
use super::PercentDecodeError;


include!("ByteProvider.rs");
include!("BytesByteProvider.rs");
include!("decode_next_percent_encoded_utf8.rs");
include!("decode_next_utf8.rs");
include!("decode_next_utf8_validity_already_checked.rs");
include!("encode_utf8_bytes_1.rs");
include!("encode_utf8_bytes_2.rs");
include!("encode_utf8_bytes_3.rs");
include!("encode_utf8_bytes_4.rs");
include!("encode_utf8_not_reserving_space.rs");
include!("encode_utf8_push_unchecked.rs");
include!("encode_utf8_reserving_space.rs");
include!("EncodeUtf8.rs");
include!("PercentEncodedByteProvider.rs");
include!("Shift6.rs");
include!("Shift12.rs");
include!("Shift18.rs");
include!("TAG_CONT.rs");
include!("TAG_FOUR_B.rs");
include!("TAG_THREE_B.rs");
include!("TAG_TWO_B.rs");
include!("UnreservedEncodeUtf8.rs");
include!("Utf8CharacterLength.rs");
include!("0F.rs");
include!("1F.rs");
include!("x3F.rs");
include!("x07.rs");
