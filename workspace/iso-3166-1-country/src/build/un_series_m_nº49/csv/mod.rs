// This file is part of olympus-xmp. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/raphaelcohn/olympus-xmp/master/COPYRIGHT. No part of olympus-xmp, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2022 The developers of olympus-xmp. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/raphaelcohn/olympus-xmp/master/COPYRIGHT.


use std::collections::BTreeMap;
use domain::NameAndM49Code;
use domain::Record;
use parser::inefficient_csv_records;
use parser::RecordParser;
use super::Country;
use super::Iso3166Dash1AlphaCode;
use super::Iso3166Dash1Alpha2Code;
use super::Iso3166Dash1Alpha3Code;
use super::Language;
use super::M49Code;
use super::M49CodeType;
use super::Names;
use super::abbreviations::Abbreviations;


mod domain;


#[path = "parser/mod.rs"]
mod parser;


include!("ExtantCountriesAbsentFromCsvForPoliticalReasons.rs");
include!("Parser.rs");
include!("Series_M_Nº49_Arabic.rs");
include!("Series_M_Nº49_Chinese.rs");
include!("Series_M_Nº49_English.rs");
include!("Series_M_Nº49_French.rs");
include!("Series_M_Nº49_Russian.rs");
include!("Series_M_Nº49_Spanish.rs");
