// This file is part of olympus-xmp. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/raphaelcohn/olympus-xmp/master/COPYRIGHT. No part of olympus-xmp, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2022 The developers of olympus-xmp. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/raphaelcohn/olympus-xmp/master/COPYRIGHT.


pub(super) const Revision3EnglishCustomsAreas: [(M49Code, &'static str, &'static [M49Code]); 7] =
[
	revision_3_or_4_customs_areas(b"058", "Belgium-Luxembourg", constituents![b"056", b"442"]),
	revision_3_or_4_customs_areas(b"251", "France-Monaco", constituents![b"250", b"492"]),
	revision_3_or_4_customs_areas(b"381", "Italy-San Marino-Holy See", constituents![b"380", b"674", b"336"]),
	revision_3_or_4_customs_areas(b"579", "Norway, Svalbard and Jan Mayen Islands", constituents![b"578", b"744"]),
	revision_3_or_4_customs_areas(b"757", "Switzerland, Liechtenstein", constituents![b"756", b"438"]),
	revision_3_or_4_customs_areas(b"841", "United States, Puerto Rico", constituents![b"840", b"630"]),
	revision_3_or_4_customs_areas(b"842", "United States, Puerto Rico, United States Virgin Islands", constituents![b"840", b"630", b"850"]),
];
