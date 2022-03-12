// This file is part of olympus-xmp. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/raphaelcohn/olympus-xmp/master/COPYRIGHT. No part of olympus-xmp, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2022 The developers of olympus-xmp. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/raphaelcohn/olympus-xmp/master/COPYRIGHT.


use super::XmpAttributeValue;
use super::XmpAttributeValueParseError;
use std::error;
use std::fmt;
use std::fmt::Display;
use std::fmt::Debug;
use std::fmt::Formatter;
use Iso3166Dash1Alpha2CountryCode::*;
use Iso3166Dash1Alpha3CountryCode::*;
use Iso3166Dash1AlphaCountryCode::*;
use Iso3166Dash1NumericCountryCode::*;


include!("Iso3166Dash1Country.rs");
include!("Iso3166Dash1Alpha2CountryCode.rs");
include!("Iso3166Dash1Alpha3CountryCode.rs");
include!("Iso3166Dash1NumericCountryCode.rs");
include!("Iso3166Dash1AlphaCountryCode.rs");
include!("Iso3166Dash1AlphaCountryCodeParseError.rs");
include!("UnknownIso3166Dash1CodeError.rs");
