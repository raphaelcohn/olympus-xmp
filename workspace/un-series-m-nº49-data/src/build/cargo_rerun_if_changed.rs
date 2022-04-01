// This file is part of olympus-xmp. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/raphaelcohn/olympus-xmp/master/COPYRIGHT. No part of olympus-xmp, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2022 The developers of olympus-xmp. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/raphaelcohn/olympus-xmp/master/COPYRIGHT.


macro_rules! cargo_rerun_if_changed
{
	() =>
	{
		cargo_rerun_if_changed!(@ None)
	};
	
	($subpath: literal) =>
	{
		cargo_rerun_if_changed!(@ Some($subpath))
	};
	
	(@ $subpath: expr) =>
	{
		$crate::build::cargo_rerun_if_changed(module_path!(), $subpath)
	};
}

#[inline(always)]
fn cargo_rerun_if_changed(module_path: &'static str, subpath: Option<&str>)
{
	let path = module_path.strip_prefix("build_script_build::").expect("module_path did not start with build_script_build::").replace("::", "/");
	print!("cargo:rerun-if-changed=src/{}", path);
	if let Some(subpath) = subpath
	{
		print!("/{}", subpath);
	}
	println!()
}
