// This file is part of olympus-xmp. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/raphaelcohn/olympus-xmp/master/COPYRIGHT. No part of olympus-xmp, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2022 The developers of olympus-xmp. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/raphaelcohn/olympus-xmp/master/COPYRIGHT.


pub(super) fn generate_rust_code(_out_folder_path: &Path) -> io::Result<()>
{
	cargo_rerun_if_changed!();
	cargo_rerun_if_changed!("csv");
	cargo_rerun_if_changed!("csv_domain");
	cargo_rerun_if_changed!("csv_parser");
	cargo_rerun_if_changed!("excel");
	
	unreachable!()
}
