// This file is part of olympus-xmp. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/raphaelcohn/olympus-xmp/master/COPYRIGHT. No part of olympus-xmp, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2022 The developers of olympus-xmp. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/raphaelcohn/olympus-xmp/master/COPYRIGHT.


use memchr::memchr;
use std::str::from_utf8_unchecked;
use super::super::Developing;
use super::super::Iso3166Dash1Alpha2Code;
use super::super::Iso3166Dash1Alpha3Code;
use super::super::M49Code;
use super::domain::NameAndM49Code;
use super::domain::Record;
use super::domain::Region;
use super::domain::SubRegion;
use swiss_army_knife::a_to_z::_0;
use swiss_army_knife::a_to_z::_9;
use swiss_army_knife::a_to_z::A;
use swiss_army_knife::a_to_z::Z;
use swiss_army_knife::get_unchecked::GetUnchecked;


include!("CrudeCsvLineIterator.rs");
include!("inefficient_csv_records.rs");
include!("RecordParser.rs");
