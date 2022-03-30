// This file is part of olympus-xmp. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/raphaelcohn/olympus-xmp/master/COPYRIGHT. No part of olympus-xmp, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2022 The developers of olympus-xmp. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/raphaelcohn/olympus-xmp/master/COPYRIGHT.


use abbreviations::Abbreviations;
use abbreviations::TwelveCharacterAbbreviation;
use std::fmt;
use std::fmt::Debug;
use std::fmt::Display;
use std::fmt::Formatter;
use swiss_army_knife::a_to_z::_0;
use swiss_army_knife::a_to_z::_9;
use swiss_army_knife::a_to_z::A;
use swiss_army_knife::a_to_z::Z;


mod abbreviations;


pub(super) mod csv;


mod excel;


mod pdf_extracts;


include!("../../letter_to_number_scaled.rs");
include!("../../letter_to_number_unchecked.rs");
include!("Country.rs");
include!("Developing.rs");
include!("Iso3166Dash1Alpha2Code.rs");
include!("Iso3166Dash1Alpha3Code.rs");
include!("Iso3166Dash1AlphaCode.rs");
include!("M49Code.rs");
include!("M49CodeType.rs");
include!("Language.rs");
include!("Names.rs");
