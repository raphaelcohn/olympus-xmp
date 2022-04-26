// This file is part of olympus-xmp. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/raphaelcohn/olympus-xmp/master/COPYRIGHT. No part of olympus-xmp, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2022 The developers of olympus-xmp. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/raphaelcohn/olympus-xmp/master/COPYRIGHT.


pub use byte_provider::InvalidUtf8ParseError;
pub use byte_provider::PercentDecodeError;
use byte_provider::BytesByteProvider;
use byte_provider::PercentEncodedByteProvider;
use crate::x7F;
use encode_utf8::EncodeUtf8;
use encode_utf8::PercentEncodeUtf8;
use encode_utf8::TryReserveEncodeUtf8;
use encode_utf8::UnreservedEncodeUtf8;
use utf8_sequence::Utf8SequenceEnum;
use std::borrow::Cow;
use std::collections::TryReserveError;
use std::convert::Infallible;
use std::fmt::Debug;
use std::num::NonZeroUsize;
use swiss_army_knife::non_zero::new_non_zero_usize;


mod byte_provider;


mod encode_utf8;


mod utf8_sequence;


include!("decode_next_percent_encoded_utf8.rs");
include!("decode_next_utf8.rs");
include!("decode_next_utf8_validity_already_checked.rs");
include!("encode_utf8_bytes_1.rs");
include!("encode_utf8_bytes_2.rs");
include!("encode_utf8_bytes_3.rs");
include!("encode_utf8_bytes_4.rs");
include!("encode_utf8_not_reserving_space.rs");
include!("encode_utf8_percent_encoded.rs");
include!("encode_utf8_push_unchecked.rs");
include!("encode_utf8_reserving_space.rs");
include!("is_ascii_character.rs");
include!("Shift6.rs");
include!("Shift12.rs");
include!("Shift18.rs");
include!("TAG_CONT.rs");
include!("TAG_FOUR_B.rs");
include!("TAG_THREE_B.rs");
include!("TAG_TWO_B.rs");
include!("Utf8CharacterLength.rs");
include!("x0F.rs");
include!("x1F.rs");
include!("x3F.rs");
include!("x07.rs");
