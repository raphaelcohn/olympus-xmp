// This file is part of olympus-xmp. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/raphaelcohn/olympus-xmp/master/COPYRIGHT. No part of olympus-xmp, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2022 The developers of olympus-xmp. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/raphaelcohn/olympus-xmp/master/COPYRIGHT.


#![feature(maybe_uninit_array_assume_init)]
#![feature(maybe_uninit_uninit_array)]
#![feature(maybe_uninit_slice)]


#![allow(non_camel_case_types)]
#![allow(non_upper_case_globals)]


use build::bcp_47_language_tag::parse_language_subtag_registry;


#[path = "src/build/mod.rs"]
mod build;


fn main()
{
	println!("cargo:rerun-if-changed=src/build.rs");
	println!("cargo:rerun-if-changed=src/build");
	println!("cargo:rerun-if-changed=src/build/bcp_47_language_tag");
	println!("cargo:rerun-if-changed=src/build/bcp_47_language_tag/date");
	println!("cargo:rerun-if-changed=src/build/bcp_47_language_tag/parser");
	println!("cargo:rerun-if-changed=src/build/bcp_47_language_tag/records");
	
	parse_language_subtag_registry().unwrap();
}
