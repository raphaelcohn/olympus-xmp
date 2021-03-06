// This file is part of olympus-xmp. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/raphaelcohn/olympus-xmp/master/COPYRIGHT. No part of olympus-xmp, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2022 The developers of olympus-xmp. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/raphaelcohn/olympus-xmp/master/COPYRIGHT.


pub(super) fn generate_rust_code()
{
	cargo_rerun_if_changed!();
	
	let out_dir = var_os("OUT_DIR").expect("OUT_DIR environment variable not set");
	let out_folder_path = Path::new(&out_dir);
	
	comtrade::generate_rust_code(out_folder_path).expect("Could not generate comtrade Rust code");
	fao::generate_rust_code(out_folder_path).expect("Could not generate n_triples Rust code");
	series_m_nº49::generate_rust_code(out_folder_path).expect("Could not generate series_m_nº49 Rust code");
	term::generate_rust_code(out_folder_path).expect("Could not generate term Rust code");
}
