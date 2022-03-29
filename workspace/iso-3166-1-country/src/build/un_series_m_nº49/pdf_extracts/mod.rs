// This file is part of olympus-xmp. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/raphaelcohn/olympus-xmp/master/COPYRIGHT. No part of olympus-xmp, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2022 The developers of olympus-xmp. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/raphaelcohn/olympus-xmp/master/COPYRIGHT.


use super::M49Code;
use super::Iso3166Dash1Alpha2Code;
use super::Iso3166Dash1Alpha3Code;
use super::abbreviations::LegacyEightCharacterAbbreviation;
use super::abbreviations::LegacyFourCharacterAbbreviation;
use super::abbreviations::TwelveCharacterAbbreviation;


include!("constituents.rs");


include!("customs_areas.rs");

include!("CommunitiesEnglish.rs");
include!("RegionOrGroupingNotElsewhereSpecified.rs");
include!("CustomsAreasRevision0.rs");
include!("Revision0Abbreviations.rs");
include!("Revision1Abbreviations.rs");
include!("Revision1CustomsAreas.rs");
include!("Revision2Abbreviations.rs");
include!("Revision2CustomsAreas.rs");
include!("Revision2RegionsAndGroupings.rs");
include!("Revision3CustomsAreas.rs");
include!("Revision3English.rs");
include!("Revision3EnglishReplacementRegions.rs");
include!("Revision4Abbreviations.rs");
include!("Revision4CustomsAreas.rs");
