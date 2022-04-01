// This file is part of olympus-xmp. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/raphaelcohn/olympus-xmp/master/COPYRIGHT. No part of olympus-xmp, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2022 The developers of olympus-xmp. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/raphaelcohn/olympus-xmp/master/COPYRIGHT.


use std::collections::{BTreeSet, HashMap};
use std::ops::Deref;
use std::mem::{discriminant, transmute};
use swiss_army_knife::get_unchecked::GetUnchecked;
use crate::build::un_series_m_nº49::csv::{CsvCountry, NamesInUnitedNationsOfficialLanguages};
use crate::build::un_series_m_nº49::pdf_extracts::{CustomsAreasRevision0, CustomsAreasRevision1, CustomsAreasRevision2, CustomsAreasRevision3, CustomsAreasRevision4};

fn main()
{
	let x = Parser::parse();
}

/*

	#[inline(always)]
	fn add_private_use_codes(&mut self)
	{
		M49Code::private_use_codes(|private_use_code|
		{
			let was = self.0.insert(private_use_code, CsvM49CodeType::PrivateUse);
			assert!(was.is_none())
		})
	}
	
	// add Private use codes.
	
	
 */

trait MainM49Code
{
	const Discriminant: MainM49CodeDiscriminant;
}

struct Global
{
	names: NamesInUnitedNationsOfficialLanguages,
}

impl MainM49Code for Global
{
	const Discriminant: MainM49CodeDiscriminant = MainM49CodeDiscriminant::Global;
}

struct Region
{
	names: NamesInUnitedNationsOfficialLanguages,

	revision_2_abbreviations:
}

impl MainM49Code for Region
{
	const Discriminant: MainM49CodeDiscriminant = MainM49CodeDiscriminant::Region;
}

struct SubRegion;

impl MainM49Code for SubRegion
{
	const Discriminant: MainM49CodeDiscriminant = MainM49CodeDiscriminant::SubRegion;
}

struct IntermediateRegion;

impl MainM49Code for IntermediateRegion
{
	const Discriminant: MainM49CodeDiscriminant = MainM49CodeDiscriminant::IntermediateRegion;
}

struct Country;

impl MainM49Code for Country
{
	const Discriminant: MainM49CodeDiscriminant = MainM49CodeDiscriminant::Country;
}

struct PrivateUse;

impl MainM49Code for PrivateUse
{
	const Discriminant: MainM49CodeDiscriminant = MainM49CodeDiscriminant::PrivateUse;
}

#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
#[repr(u8)]
enum MainM49CodeDiscriminant
{
	Global = 0,
	
	Region = 1,
	
	SubRegion = 2,
	
	IntermediateRegion = 3,
	
	Country = 4,
	
	PrivateUse = 5,
	
	CustomsArea = 6,
	
	DevelopingAndDevelopedRegions = 7,
	
	FormerCountry = 8,
	
	ObsoleteRegionsRevision3Onwards = 9,
	
	OtherGroupings = 10,
	
	RegionOrOtherGroupingNotElsewhereSpecified = 11,
	
	StatisticalCountryLike = 12,
	
	UnofficialCountrySubdivision = 13,
}

impl<'a> const From<&'a MainM49CodeVariant> for MainM49CodeDiscriminant
{
	#[inline(always)]
	const fn from(value: &'a MainM49CodeVariant) -> Self
	{
		let x = discriminant(value);
		unsafe { transmute(x) }
	}
}

#[derive(Debug, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
#[repr(u8)]
enum MainM49CodeVariant
{
	Global(NamesInUnitedNationsOfficialLanguages) = 0,
	
	Region(NamesInUnitedNationsOfficialLanguages) = 1,
	
	SubRegion(NamesInUnitedNationsOfficialLanguages) = 2,
	
	IntermediateRegion(NamesInUnitedNationsOfficialLanguages) = 3,
	
	Country(CsvCountry) = 4,

	PrivateUse = 5,

	CustomsArea = 6,
	
	DevelopingAndDevelopedRegions = 7,

	FormerCountry = 8,

	ObsoleteRegionsRevision3Onwards = 9,

	OtherGroupings = 10,

	RegionOrOtherGroupingNotElsewhereSpecified = 11,

	StatisticalCountryLike = 12,

	UnofficialCountrySubdivision = 13,
}
