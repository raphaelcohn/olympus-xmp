// This file is part of olympus-xmp. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/raphaelcohn/olympus-xmp/master/COPYRIGHT. No part of olympus-xmp, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2022 The developers of olympus-xmp. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/raphaelcohn/olympus-xmp/master/COPYRIGHT.


/// This is inefficient as it uses a generic str::split() instead of memchr().
#[inline(always)]
fn new_inefficient_record_lines_iterator(line_orientated_record: &'static str) -> impl Iterator<Item=&'static str>
{
	const LineFeed: char = '\n';
	line_orientated_record.split(LineFeed)
}
