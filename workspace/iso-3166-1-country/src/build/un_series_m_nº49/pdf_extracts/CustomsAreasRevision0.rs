// This file is part of olympus-xmp. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/raphaelcohn/olympus-xmp/master/COPYRIGHT. No part of olympus-xmp, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2022 The developers of olympus-xmp. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/raphaelcohn/olympus-xmp/master/COPYRIGHT.


/// Must be kept in sort order.
///
/// This list is implicit from the text of the standard.
pub(super) const CustomsAreasRevision0: [(M49Code, &'static str, &'static [M49Code]); 8] =
[
	customs_area(b"056", "Belgium", constituents![b"056", b"442"]),
	customs_area(b"250", "France", constituents![b"250", b"492"]),
	customs_area(b"380", "Italy", constituents![b"380", b"674"]),
	customs_area(b"578", "Norway", constituents![b"578", b"744"]),
	customs_area(b"710", "South Africa", constituents![b"710", b"072", b"426", b"516", b"748"]),
	customs_area(b"724", "Spain", constituents![b"724", b"728"]),
	customs_area(b"756", "Switzerland", constituents![b"756", b"438"]),
	customs_area(b"840", "United States", constituents![b"840", b"630", b"850"]),
];
