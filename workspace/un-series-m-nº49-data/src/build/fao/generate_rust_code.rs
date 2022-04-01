// This file is part of olympus-xmp. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/raphaelcohn/olympus-xmp/master/COPYRIGHT. No part of olympus-xmp, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2022 The developers of olympus-xmp. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/raphaelcohn/olympus-xmp/master/COPYRIGHT.


#[inline(always)]
pub(super) fn generate_rust_code(out_folder_path: &Path) -> io::Result<()>
{
	cargo_rerun_if_changed!();
	cargo_rerun_if_changed!("n_triples");
	
	for (m49_code, n_triples_string) in NTriplesFiles
	{
		let n_triples = NTriples::parse(n_triples_string);
		eprintln!("{:?} => {:?}", m49_code, n_triples);
	}
	
	unreachable!()
}
