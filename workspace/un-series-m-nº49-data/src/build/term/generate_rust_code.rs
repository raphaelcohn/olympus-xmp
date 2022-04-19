// This file is part of olympus-xmp. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/raphaelcohn/olympus-xmp/master/COPYRIGHT. No part of olympus-xmp, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2022 The developers of olympus-xmp. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/raphaelcohn/olympus-xmp/master/COPYRIGHT.


pub(super) fn generate_rust_code(_out_folder_path: &Path) -> io::Result<()>
{
	cargo_rerun_if_changed!();
	cargo_rerun_if_changed!("excel");
	
	unreachable!()
}


// /*
// * Write a csv_parser for EFSRCA.xlsx which creates the official name mappings for
//   * UNTERMS short
//   * UNTERMS long
//   * Note that some states end ` *` or ` **`.
//   * Note that not all states will be present that are in ISO / M.49!
// * Try to create an alogrithm to match up EFSRCA terms with the UNSD terms
//   * This may be some sort of closests string match algo as used in the rust compiler for 'did you mean'
//   * This may involve stripping things in brackets
//  */
// fn example() -> Result<(), CalamineError>
// {
// 	let path = format!("{}/src/build/term/Historical-classification-of-developed-and-developing-regions.xlsx", env!("CARGO_MANIFEST_DIR"));
//
// 	let mut workbook: Xlsx<_> = open_workbook(path)?;
// 	let range = workbook.worksheet_range("Worksheet1").ok_or(CalamineError::Msg("Cannot find 'Worksheet1'"))??;
// 	for row in range.rows()
// 	{
// 		eprintln!("row={:?}, row[0]={:?}", row, row[0]);
// 	}
//
// 	Ok(())
// }
