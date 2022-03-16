// This file is part of olympus-xmp. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/raphaelcohn/olympus-xmp/master/COPYRIGHT. No part of olympus-xmp, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2022 The developers of olympus-xmp. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/raphaelcohn/olympus-xmp/master/COPYRIGHT.


use std::env::var_os;
use std::fs::File;
use std::io;
use std::io::Read;
use std::path::Path;
use std::path::PathBuf;


include!("relative_module_path.rs");


pub(super) mod bcp_47_language_code;


include!("BytesPerKilobyte.rs");
include!("manifest_path.rs");
include!("open_our_module_file_path.rs");
include!("our_module_file_path.rs");
include!("our_module_path.rs");
include!("relative_module_path_.rs");
