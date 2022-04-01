// This file is part of olympus-xmp. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/raphaelcohn/olympus-xmp/master/COPYRIGHT. No part of olympus-xmp, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2022 The developers of olympus-xmp. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/raphaelcohn/olympus-xmp/master/COPYRIGHT.


struct CustomsAreaRevision
{
	english: &'static str,

	constituents: Constituents,
}

impl CustomsAreaRevision
{
	#[inline(always)]
	fn new(english: &'static str, constituents: &'static [M49Code]) -> Self
	{
		Self
		{
			english,
			constituents: Constituents::from(constituents)
		}
	}
}

struct CustomsArea
{
	extant: bool,

	revision_4: CustomsAreaRevision,
}

impl CustomsArea
{
	#[inline(always)]
	fn extant(english: &'static str, constituents: &'static [M49Code]) -> Self
	{
		Self
		{
			extant: true,
		
			revision_4: CustomsAreaRevision::new(english, constituents),
		}
	}
}

fn add_customs_areas()
{
	// Three kinds of name lists:-
		// By revision;
		// Deduplicated
		// Not the previous name.
	
	// TODO: Constituents are ALWAYS countries, but we really should check that.
	
	let mut customs_areas = HashMap::with_capacity(CustomsAreasRevision4.len() * 2);
	for (m49_code, english, constituents) in CustomsAreasRevision4
	{
		customs_areas.insert(m49_code, CustomsArea::extant(english, constituents))
	}
	
	for (m49_code, english, constituents) in CustomsAreasRevision3
	{
		// TODO: What do we want as the final output? This dictats what we do here!
		// We MUST merge entries with the same name & same constituents
		customs_areas.entry(m49_code).and_modify().or_insert_with()
	}
	
	for (m49_code, english, constituents) in CustomsAreasRevision2
	{
		customs_areas.entry(m49_code).and_modify().or_insert_with()
	}
	
	for (m49_code, english, constituents) in CustomsAreasRevision1
	{
		customs_areas.entry(m49_code).and_modify().or_insert_with()
	}
	
	for (m49_code, english, constituents) in CustomsAreasRevision0
	{
		customs_areas.entry(m49_code).and_modify().or_insert_with()
	}
	
	// TODO: Add UN Comtrade name, if any.
}
