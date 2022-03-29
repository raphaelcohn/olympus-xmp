// This file is part of olympus-xmp. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/raphaelcohn/olympus-xmp/master/COPYRIGHT. No part of olympus-xmp, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2022 The developers of olympus-xmp. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/raphaelcohn/olympus-xmp/master/COPYRIGHT.


#![allow(non_camel_case_types)]
#![allow(non_upper_case_globals)]
#![allow(uncommon_codepoints)]


#![feature(adt_const_params)]
#![feature(const_trait_impl)]
#![feature(explicit_generic_args_with_impl_trait)]
#![feature(generic_arg_infer)]


#[path = "build/mod.rs"]
mod build;


fn main()
{
	println!("cargo:rerun-if-changed=src/build.rs");
	println!("cargo:rerun-if-changed=src/build");
	println!("cargo:rerun-if-changed=src/build/un_series_m_nº49");
	println!("cargo:rerun-if-changed=src/build/un_series_m_nº49/csv");
	
	build::mapping();
}
