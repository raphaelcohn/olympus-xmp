// This file is part of olympus-xmp. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/raphaelcohn/olympus-xmp/master/COPYRIGHT. No part of olympus-xmp, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2022 The developers of olympus-xmp. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/raphaelcohn/olympus-xmp/master/COPYRIGHT.


use un_series_m_nº49::csv::Parser;


pub(super) mod un_comtrade;


//noinspection NonAsciiCharacters
#[path = "un_series_m_nº49/mod.rs"]
pub(super) mod un_series_m_nº49;


pub(super) mod unterm;


pub(super) fn mapping()
{
	let mapping = Parser::parse();
	
	for (key, value) in mapping
	{
		eprintln!("{:?} => {:?}", key, value)
	}
	panic!("Do stuff");
}
