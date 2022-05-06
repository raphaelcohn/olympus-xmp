// This file is part of olympus-xmp. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/raphaelcohn/olympus-xmp/master/COPYRIGHT. No part of olympus-xmp, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2022 The developers of olympus-xmp. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/raphaelcohn/olympus-xmp/master/COPYRIGHT.


use std::borrow::Cow;
use std::collections::TryReserveError;
use std::mem::size_of;
use std::ptr::NonNull;
use super::x80;
use super::utf8_sequence::Utf8Sequence;
use super::utf8_sequence::Utf8Sequence1;
use super::utf8_sequence::Utf8Sequence2;
use super::utf8_sequence::Utf8Sequence3;
use super::utf8_sequence::Utf8Sequence4;
use super::utf8_sequence::Utf8SequenceEnum;
use super::utf8_sequence::Utf8SequenceAndCharacter;
use swiss_army_knife::a_to_z::_0;
use swiss_army_knife::a_to_z::A;
use swiss_army_knife::a_to_z::Percent;
use swiss_army_knife::get_unchecked::GetUnchecked;
use swiss_army_knife::non_zero::new_non_null;
use swiss_army_knife::unreachable_code_const;
use swiss_army_knife::vec::VecExt;


include!("encode_utf8_callback.rs");
include!("MAX_ONE_B.rs");
include!("MAX_THREE_B.rs");
include!("MAX_TWO_B.rs");
include!("PercentEncodedUtf8ByteSize.rs");
include!("PercentEncodeUtf8.rs");
