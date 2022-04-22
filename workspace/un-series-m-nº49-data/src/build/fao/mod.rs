// This file is part of olympus-xmp. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/raphaelcohn/olympus-xmp/master/COPYRIGHT. No part of olympus-xmp, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2022 The developers of olympus-xmp. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/raphaelcohn/olympus-xmp/master/COPYRIGHT.


use n_triples::NTriples;
use n_triples::PathDepth;
use n_triples::Subject;
use n_triples::internationalized_resource_identifier::AbsoluteInternationalizedResourceIdentifier;
use n_triples::internationalized_resource_identifier::RemovePrefixError;
use n_triples::parser::NTriplesParseError;
use n_triples::predicate::GetXmlSchemaValueError;
use n_triples::predicate::OptionalAbsoluteInternationalizedResourceIdentifierError;
use n_triples::predicate::Predicate;
use n_triples::predicate::Predicates;
use n_triples::string_literals_map::Integer;
use n_triples::string_literals_map::ParseDateTimeError;
use n_triples::string_literals_map::OnlyOneXmlSchemaValueError;
use n_triples::string_literals_map::OptionalXmlSchemaValueError;
use std::convert::Infallible;
use std::num::ParseIntError;
use std::str::ParseBoolError;
use std::path::Path;
use super::StaticM49Code;
use swiss_army_knife::get_unchecked::GetUnchecked;


include!("generate_rust_code.rs");
include!("host_name.rs");
include!("NTriplesFiles.rs");
include!("path_segment.rs");
include!("ProcessNTriplesError.rs");
