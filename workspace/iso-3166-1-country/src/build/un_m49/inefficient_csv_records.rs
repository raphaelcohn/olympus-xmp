// This file is part of olympus-xmp. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/raphaelcohn/olympus-xmp/master/COPYRIGHT. No part of olympus-xmp, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2022 The developers of olympus-xmp. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/raphaelcohn/olympus-xmp/master/COPYRIGHT.


/// It is inefficient as it uses a generic str::split() instead of memchr().
fn inefficient_csv_records(csv: &'static str) -> impl Iterator<Item=&'static str>
{
	const LineFeed: char = '\n';
	let mut lines = csv.split(LineFeed);
	
	let _heading =
	{
		let next = lines.next();
		unsafe { next.unwrap_unchecked() }
	};
	
	lines
}
