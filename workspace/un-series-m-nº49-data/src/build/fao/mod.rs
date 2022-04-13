// This file is part of olympus-xmp. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/raphaelcohn/olympus-xmp/master/COPYRIGHT. No part of olympus-xmp, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2022 The developers of olympus-xmp. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/raphaelcohn/olympus-xmp/master/COPYRIGHT.


use n_triples::NTriples;
use n_triples::parser::NTriplesParseError;
use n_triples::Predicate;
use n_triples::Predicates;
use n_triples::Subject;
use std::path::Path;
use super::m49_code_string;
use super::StaticM49Code;


include!("generate_rust_code.rs");
include!("NTriplesFiles.rs");
