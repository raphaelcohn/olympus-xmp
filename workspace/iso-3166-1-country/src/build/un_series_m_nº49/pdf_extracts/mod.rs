// This file is part of olympus-xmp. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/raphaelcohn/olympus-xmp/master/COPYRIGHT. No part of olympus-xmp, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2022 The developers of olympus-xmp. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/raphaelcohn/olympus-xmp/master/COPYRIGHT.


use super::M49Code;
use super::Iso3166Dash1Alpha2Code;
use super::Iso3166Dash1Alpha3Code;
use super::abbreviations::LegacyEightCharacterAbbreviation;
use super::abbreviations::LegacyFourCharacterAbbreviation;
use super::abbreviations::TwelveCharacterAbbreviation;


include!("constituents.rs");


include!("NamesAndAbbreviationsForRegionsAndGroupingsRevision2.rs");
include!("NamesAndAbbreviationsForCountriesRevision0.rs");
include!("NamesAndAbbreviationsForCountriesRevision1.rs");
include!("NamesAndAbbreviationsForCountriesRevision2.rs");
include!("NamesAndAbbreviationsForCountriesRevision3.MissingNonEnglish.rs");
include!("NamesAndAbbreviationsForCountriesRevision4.MissingNonEnglish.rs");
include!("Communities.rs");
include!("CustomsAreasRevision0.rs");
include!("CustomsAreasRevision1.rs");
include!("CustomsAreasRevision2.rs");
include!("CustomsAreasRevision3.rs");
include!("CustomsAreasRevision4.rs");
include!("ObsoleteRegionsRevision3Onwards.MissingNonEnglish.rs");
include!("RegionOrGroupingNotElsewhereSpecifiedRevision3Onwards.MissingNonEnglish.rs");
include!("customs_area_revision_0_or_1_or_2.rs");
include!("customs_area_revision_3_or_4.rs");
