// This file is part of olympus-xmp. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/raphaelcohn/olympus-xmp/master/COPYRIGHT. No part of olympus-xmp, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2022 The developers of olympus-xmp. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/raphaelcohn/olympus-xmp/master/COPYRIGHT.


use std::fmt::Debug;
use std::char::CharTryFromError;
use std::mem::discriminant;
use std::mem::transmute;
use std::num::NonZeroUsize;
use super::byte_provider::ByteProvider;
use super::Shift12;
use super::Shift18;
use super::Shift6;
use super::Utf8CharacterLength;
use super::Utf8CharacterLength::*;
use super::x07;
use super::x0F;
use super::x1F;


include!("Utf8Sequence.rs");
include!("Utf8Sequence1.rs");
include!("Utf8Sequence2.rs");
include!("Utf8Sequence3.rs");
include!("Utf8Sequence4.rs");
include!("Utf8SequenceEnum.rs");
include!("Utf8SequenceNonConst.rs");
include!("xE0.rs");
include!("xF0.rs");
