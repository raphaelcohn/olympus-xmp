// This file is part of olympus-xmp. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/raphaelcohn/olympus-xmp/master/COPYRIGHT. No part of olympus-xmp, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2022 The developers of olympus-xmp. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/raphaelcohn/olympus-xmp/master/COPYRIGHT.


use super::super::Country;
use super::super::Developing;
use super::super::Iso3166Dash1Alpha2Code;
use super::super::Iso3166Dash1Alpha3Code;
use super::super::Names;
use super::super::M49Code;
use super::super::pdf_extracts::NamesAndAbbreviationsForCountriesRevision4;
use swiss_army_knife::get_unchecked::GetUnchecked;


include!("NameAndM49Code.rs");
include!("Record.rs");
include!("Region.rs");
include!("SubRegion.rs");
