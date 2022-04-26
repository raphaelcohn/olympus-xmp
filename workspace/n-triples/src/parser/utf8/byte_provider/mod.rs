// This file is part of olympus-xmp. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/raphaelcohn/olympus-xmp/master/COPYRIGHT. No part of olympus-xmp, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2022 The developers of olympus-xmp. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/raphaelcohn/olympus-xmp/master/COPYRIGHT.


use std::char::CharTryFromError;
use std::convert::Infallible;
use std::error;
use std::fmt;
use std::fmt::Debug;
use std::fmt::Display;
use std::fmt::Formatter;
use std::num::NonZeroU8;
use std::num::NonZeroUsize;
use super::Utf8CharacterLength;
use super::Utf8CharacterLength::*;
use super::utf8_sequence::Utf8Sequence;
use super::utf8_sequence::Utf8Sequence1;
use super::utf8_sequence::Utf8Sequence2;
use super::utf8_sequence::Utf8Sequence3;
use super::utf8_sequence::Utf8Sequence4;
use super::utf8_sequence::Utf8SequenceEnum;
use super::utf8_sequence::Utf8SequenceNonConst;
use swiss_army_knife::a_to_z::f;
use swiss_army_knife::a_to_z::F;
use swiss_army_knife::a_to_z::a;
use swiss_army_knife::a_to_z::A;
use swiss_army_knife::a_to_z::_9;
use swiss_army_knife::a_to_z::_0;
use swiss_army_knife::a_to_z::Percent;
use swiss_army_knife::get_unchecked::GetUnchecked;
use swiss_army_knife::non_zero::new_non_zero_u8;
use swiss_army_knife::non_zero::new_non_zero_usize;


include!("ByteProvider.rs");
include!("BytesByteProvider.rs");
include!("InvalidUtf8ParseError.rs");
include!("PercentDecodeError.rs");
include!("PercentEncodedByteProvider.rs");
