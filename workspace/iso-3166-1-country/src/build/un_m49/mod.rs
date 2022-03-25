// This file is part of olympus-xmp. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/raphaelcohn/olympus-xmp/master/COPYRIGHT. No part of olympus-xmp, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2022 The developers of olympus-xmp. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/raphaelcohn/olympus-xmp/master/COPYRIGHT.


use memchr::memchr;
use std::collections::BTreeMap;
use std::fmt;
use std::fmt::Debug;
use std::fmt::Display;
use std::fmt::Formatter;
use std::str::from_utf8_unchecked;
use swiss_army_knife::a_to_z::_0;
use swiss_army_knife::a_to_z::_9;
use swiss_army_knife::a_to_z::A;
use swiss_army_knife::a_to_z::Z;
use swiss_army_knife::get_unchecked::GetUnchecked;
use data::Arabic;
use data::Chinese;
use data::English;
use data::French;
use data::Russian;
use data::Spanish;


mod data;


include!("../../letter_to_number_scaled.rs");
include!("../../letter_to_number_unchecked.rs");
include!("Country.rs");
include!("CrudeCsvLineIterator.rs");
include!("Developing.rs");
include!("inefficient_csv_records.rs");
include!("RecordParser.rs");
include!("Iso3166Dash1Alpha2Code.rs");
include!("Iso3166Dash1Alpha3Code.rs");
include!("M49Code.rs");
include!("M49CodeType.rs");
include!("Mapping.rs");
include!("NameAndM49Code.rs");
include!("Names.rs");
include!("Record.rs");
include!("Region.rs");
include!("SubRegion.rs");
