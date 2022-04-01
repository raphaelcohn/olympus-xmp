// This file is part of olympus-xmp. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/raphaelcohn/olympus-xmp/master/COPYRIGHT. No part of olympus-xmp, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2022 The developers of olympus-xmp. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/raphaelcohn/olympus-xmp/master/COPYRIGHT.


/// Must be kept in ascending sort order.
///
/// This data is a duplication of continent (region) data, with different names (and regional spatial definition) for "009" (Oceania), the "019" (Americas) and "142" (Asia).
/// Both revisions (3 and 4) fail to exclude "392" (Japan) from "142" (Asia) which seems like a mistake; this has been corrected here.
///
/// TODO: Non-English names.
const DevelopingAndDevelopedRegionsRevisions3Onwards:
(
	(
		M49Code,
		StaticEnglishName,
		[(M49Code, StaticEnglishName); 4],
	),
	(
		M49Code,
		StaticEnglishName,
		[(M49Code, StaticEnglishName, StaticEnglishName, Option<M49Code>); 7]
	),
) =
{
	#[inline(always)]
	const fn developed_region(m49_code: StaticM49Code, english_name_revision_3_and_4: StaticEnglishName) -> (M49Code, StaticEnglishName)
	{
		(M49Code::from(m49_code), english_name_revision_3_and_4)
	}
	
	#[inline(always)]
	const fn developing_region(m49_code: StaticM49Code, english_name_revision_3: StaticEnglishName, english_name_revision_4: StaticEnglishName, excludes: Option<StaticM49Code>) -> (M49Code, StaticEnglishName, StaticEnglishName, Option<M49Code>)
	{
		(M49Code::from(m49_code), english_name_revision_3, english_name_revision_3, excludes.map(const_m49_code_from))
	}
	
	(
		(
			M49Code::from(b"514"),
			"Developed regions",
			[
				developed_region(b"021", "Northern America"),
				developed_region(b"150", "Europe"),
				developed_region(b"392", "Japan"),
				developed_region(b"053", "Australia and New Zealand"),
			]
		),
		
		(
			M49Code::from(b"515"),
			"Developing regions",
			[
				developing_region(b"002", "Africa", "Africa", None),
				developing_region(b"005", "South America", "South America", None),
				developing_region(b"009", "Oceania [exc. Australia and New Zealand]", "Oceania excluding Australia and New Zealand", Some(b"053")),
				developing_region(b"013", "Central America", "Central America", None),
				developing_region(b"019", "Americas [exc. Canada and United States in northern
	America]", "Americas excluding Canada and United States in northern America", Some(b"021")),
				developing_region(b"029", "Caribbean", "Caribbean", None),
				developing_region(b"142", "Asia [exc. Japan]", "Asia excluding Japan", Some(b"392")),
			],
		),
	)
};
