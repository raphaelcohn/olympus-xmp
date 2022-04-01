// This file is part of olympus-xmp. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/raphaelcohn/olympus-xmp/master/COPYRIGHT. No part of olympus-xmp, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2022 The developers of olympus-xmp. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/raphaelcohn/olympus-xmp/master/COPYRIGHT.


#[inline(always)]
fn reporterAreas_json_without_byte_order_mark() -> &'static str
{
	const reporterAreasJson: &'static str = include_str!("json/reporterAreas.json");
	
	let bytes = reporterAreasJson.as_bytes();
	if &bytes[0 .. 3] == [0xEF, 0xBB, 0xBF]
	{
		unsafe { from_utf8_unchecked(bytes.get_unchecked_range_safe(3 .. )) }
	}
	else
	{
		reporterAreasJson
	}
}
