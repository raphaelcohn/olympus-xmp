// This file is part of olympus-xmp. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/raphaelcohn/olympus-xmp/master/COPYRIGHT. No part of olympus-xmp, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2022 The developers of olympus-xmp. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/raphaelcohn/olympus-xmp/master/COPYRIGHT.


use calamine::open_workbook;
use calamine::Error as CalamineError;
use calamine::Xlsx;
use calamine::Reader;

fn example() -> Result<(), CalamineError>
{
	let path = format!("{}/src/build/series_m_nº49/csv/Historical-classification-of-developed-and-developing-regions.xlsx", env!("CARGO_MANIFEST_DIR"));
	
	let mut workbook: Xlsx<_> = open_workbook(path)?;
	let range = workbook.worksheet_range("Distinction as of December 2021").ok_or(CalamineError::Msg("Cannot find 'Sheet1'"))??;
	for row in range.rows()
	{
		eprintln!("row={:?}, row[0]={:?}", row, row[0]);
	}
	
	Ok(())
}
