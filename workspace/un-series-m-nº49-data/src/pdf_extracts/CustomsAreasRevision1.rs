// This file is part of olympus-xmp. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/raphaelcohn/olympus-xmp/master/COPYRIGHT. No part of olympus-xmp, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2022 The developers of olympus-xmp. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/raphaelcohn/olympus-xmp/master/COPYRIGHT.


/// Must be kept in ascending sort order.
pub(super) const CustomsAreasRevision1: [(M49Code, StaticEnglishName, StaticConstituents); 7] =
[
	customs_area_revision_0_or_1_or_2(b"056", "Belgium", constituents![b"056", b"442"]),
	customs_area_revision_0_or_1_or_2(b"250", "France", constituents![b"250", b"492"]),
	customs_area_revision_0_or_1_or_2(b"380", "Italy", constituents![b"380", b"674"]),
	customs_area_revision_0_or_1_or_2(b"578", "Norway", constituents![b"578", b"744"]),
	customs_area_revision_0_or_1_or_2(b"710", "South Africa", constituents![b"072", b"426", b"516", b"710", b"748"]),
	//customs_area_revisions_0_1_and_2(b"724", "Spain", constituents![b"724", b"728"]), // This seems to be a mistake in Revision 1, as Spanish North Africa no longer existed in 1975 and was not defined in Revision 1.
	customs_area_revision_0_or_1_or_2(b"756", "Switzerland", constituents![b"438", b"756"]),
	customs_area_revision_0_or_1_or_2(b"840", "United States", constituents![b"630", b"840", b"850"]),
];
