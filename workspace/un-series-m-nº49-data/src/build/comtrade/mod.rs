// This file is part of olympus-xmp. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/raphaelcohn/olympus-xmp/master/COPYRIGHT. No part of olympus-xmp, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2022 The developers of olympus-xmp. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/raphaelcohn/olympus-xmp/master/COPYRIGHT.


use std::collections::BTreeMap;
use std::fs::File;
use std::io;
use std::io::Write;
use std::path::Path;
use std::str::from_utf8_unchecked;
use json::Array;
use json::JsonValue;
use json::parse;
use json::object::Object;
use super::M49CodeArray;
use super::write_key;
use swiss_army_knife::a_to_z::_0;
use swiss_army_knife::a_to_z::_9;
use swiss_army_knife::get_unchecked::GetUnchecked;


include!("first.rs");
include!("JsonObjectExt.rs");
include!("JsonValueExt.rs");
include!("generate_rust_code.rs");
include!("reporter_areas.rs");
include!("reporterAreas_json_without_byte_order_mark.rs");
include!("subsequent.rs");
