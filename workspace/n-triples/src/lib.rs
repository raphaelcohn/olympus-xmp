// This file is part of olympus-xmp. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/raphaelcohn/olympus-xmp/master/COPYRIGHT. No part of olympus-xmp, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2022 The developers of olympus-xmp. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/raphaelcohn/olympus-xmp/master/COPYRIGHT.


#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(uncommon_codepoints)]
#![deny(absolute_paths_not_starting_with_crate)]
#![deny(invalid_html_tags)]
#![deny(macro_use_extern_crate)]
#![deny(missing_crate_level_docs)]
#![deny(missing_docs)]
#![deny(pointer_structural_match)]
#![deny(unaligned_references)]
#![deny(unconditional_recursion)]
#![deny(unreachable_patterns)]
#![deny(unused_import_braces)]
#![deny(unused_must_use)]
#![deny(unused_qualifications)]
#![deny(unused_results)]
#![deny(unreachable_code)]
#![warn(unreachable_pub)]
#![warn(unused_lifetimes)]
#![warn(unused_crate_dependencies)]


#![feature(adt_const_params)]
#![feature(allocator_api)]
#![feature(const_trait_impl)]
#![feature(const_try)]
#![feature(generic_arg_infer)]
#![feature(nonnull_slice_from_raw_parts)]
#![feature(slice_ptr_get)]
#![feature(slice_ptr_len)]
#![feature(min_specialization)]
#![feature(try_reserve_kind)]
#![feature(untagged_unions)]


//! #n-triples.
//!
//! Domain model and parser for RDF N-triples [version 1.1](https://www.w3.org/TR/n-triples/).


use internationalized_resource_identifier::AbsoluteInternationalizedResourceIdentifier;
use parser::BlankNodeLabelParseError;
use parser::get_0;
use parser::LiteralTag;
use parser::NTriple;
use parser::NTriplesParseError;
use parser::Object;
use parser::StringLiteral;
use parser::StringSoFar;
use parser::utf8::decode_next_utf8;
use std::borrow::Borrow;
use std::borrow::Cow;
use std::collections::HashMap;
use std::collections::TryReserveError;
use std::error;
use std::fmt;
use std::fmt::Debug;
use std::fmt::Display;
use std::fmt::Formatter;
use std::mem::transmute;
use std::num::ParseIntError;
use std::ops::Deref;
use std::str::FromStr;
use try_to_own::MutableKey;
use try_to_own::MutableKeyHashMap;
use try_to_own::try_to_own_in_place_cow;
use try_to_own::TryToOwn;
use try_to_own::TryToOwnInPlace;
use swiss_army_knife::a_to_z::Colon;
use swiss_army_knife::a_to_z::Period;
use swiss_army_knife::get_unchecked::GetUnchecked;
use swiss_army_knife::vec::VecExt;


/// Internationalized Resource Identifier (IRI).
#[allow(unused_qualifications)]
pub mod internationalized_resource_identifier;


/// Parser.
pub mod parser;


/// Try-to-own
pub mod try_to_own;


include!("u8.constants.rs");
include!("BlankNodeLabel.rs");
include!("FromUnchecked.rs");
include!("GetStringPredicateError.rs");
include!("NTriples.rs");
include!("Objects.rs");
include!("PathDepth.rs");
include!("Predicate.rs");
include!("Predicates.rs");
include!("RawIetfBcp47LanguageTag.rs");
include!("Subject.rs");

include!("char.constants.rs");
