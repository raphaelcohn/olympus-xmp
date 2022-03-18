// This file is part of olympus-xmp. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/raphaelcohn/olympus-xmp/master/COPYRIGHT. No part of olympus-xmp, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2022 The developers of olympus-xmp. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/raphaelcohn/olympus-xmp/master/COPYRIGHT.


use parser::PullEventParser;
use parser::PullEventParserError;
use records::FieldError;
use records::Records;
use records::RecordParseError;
use records::RecordsFileHeaderParseError;
use records::Type;
use std::error;
use std::fmt;
use std::fmt::Debug;
use std::fmt::Display;
use std::fmt::Formatter;
use std::io;
use super::open_our_module_file_path;


mod date;


pub(crate) mod records;


mod parser;


include!("LanguageSubtagRegistryFileParseError.rs");
include!("parse_language_subtag_registry.rs");
