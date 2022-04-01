// This file is part of olympus-xmp. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/raphaelcohn/olympus-xmp/master/COPYRIGHT. No part of olympus-xmp, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2022 The developers of olympus-xmp. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/raphaelcohn/olympus-xmp/master/COPYRIGHT.


#[inline(always)]
pub(super) fn generate_rust_code(out_folder_path: &Path) -> io::Result<()>
{
	cargo_rerun_if_changed!();
	cargo_rerun_if_changed!("json");
	
	let mut file = File::create(out_folder_path.join("comtrade.rs"))?;
	
	let reporter_areas = reporter_areas();
	
	writeln!(file, "/// Generated Rust code.")?;
	writeln!(file, "pub const UnComtrade: [(M49Code, StaticEnglishName); {}] =", reporter_areas.len())?;
	writeln!(file, "[")?;
	for (key, value) in reporter_areas
	{
		write!(file, "\tcomtrade(")?;
		write_key(&mut file, key)?;
		writeln!(file, ", \"{}\"),", value)?
	}
	writeln!(file, "];")
}
