// This file is part of olympus-xmp. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/raphaelcohn/olympus-xmp/master/COPYRIGHT. No part of olympus-xmp, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2022 The developers of olympus-xmp. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/raphaelcohn/olympus-xmp/master/COPYRIGHT.


use std::env::var_os;
use std::fs::File;
use std::io;
use std::io::Write;
use std::path::Path;
use swiss_army_knife::get_unchecked::GetUnchecked;


include!("cargo_rerun_if_changed.rs");


/// UN Comtrade.
mod comtrade;


/// UN FAO.
mod fao;


//noinspection NonAsciiCharacters
#[path = "series_m_nº49/mod.rs"]
/// UN Series M, Nº49.
mod series_m_nº49;


/// UNTERM.
mod term;


include!("generate_rust_code.rs");
include!("m49_code_string.rs");
include!("M49CodeArray.rs");
include!("StaticM49Code.rs");
include!("new_inefficient_record_lines_iterator.rs");
include!("write_key.rs");
