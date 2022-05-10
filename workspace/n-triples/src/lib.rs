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
#![feature(const_char_convert)]
#![feature(const_convert)]
#![feature(const_deref)]
#![feature(const_discriminant)]
#![feature(const_maybe_uninit_write)]
#![feature(const_mut_refs)]
#![feature(const_option)]
#![feature(const_ptr_offset)]
#![feature(const_ptr_read)]
#![feature(const_ptr_write)]
#![feature(const_refs_to_cell)]
#![feature(const_trait_impl)]
#![feature(const_try)]
#![feature(generic_arg_infer)]
#![feature(nonnull_slice_from_raw_parts)]
#![feature(slice_ptr_get)]
#![feature(slice_ptr_len)]
#![feature(try_reserve_kind)]
#![feature(untagged_unions)]
#![feature(maybe_uninit_uninit_array)]
#![feature(maybe_uninit_array_assume_init)]
#![feature(trusted_len)]
#![feature(once_cell)]
#![feature(arbitrary_enum_discriminant)]


//! #n-triples.
//!
//! Domain model and parser for RDF N-triples [version 1.1](https://www.w3.org/TR/n-triples/).


use chars::x00B7;
use chars::x00C0;
use chars::x00D6;
use chars::x00D8;
use chars::x00F6;
use chars::x00F8;
use chars::x02FF;
use chars::x0300;
use chars::x036F;
use chars::x0370;
use chars::x037D;
use chars::x037F;
use chars::x07FF;
use chars::x0800;
use chars::x10000;
use chars::x1FFF;
use chars::x200C;
use chars::x200D;
use chars::x203F;
use chars::x2040;
use chars::x2070;
use chars::x218F;
use chars::x2C00;
use chars::x2FEF;
use chars::x3001;
use chars::xD7FF;
use chars::xEFFFF;
use chars::xF900;
use chars::xFDCF;
use chars::xFDF0;
use chars::xFFFD;
use internationalized_resource_identifier::AbsoluteInternationalizedResourceIdentifier;
use parser::BlankNodeLabelParseError;
use parser::LiteralTag;
use parser::NaiveIetfBcp47LanguageTagParseError;
use parser::NTriple;
use parser::NTriplesParseError;
use parser::Object;
use parser::StringLiteral;
use predicate::Predicate;
use std::borrow::Cow;
use std::collections::HashMap;
use std::collections::TryReserveError;
use std::fmt;
use std::fmt::Debug;
use std::fmt::Display;
use std::fmt::Formatter;
use std::mem::transmute;
use std::ops::Deref;
use std::str::from_utf8_unchecked;
use string_literals_map::StringLiteralsMap;
use swiss_army_knife::try_to_own::TryToOwn;
use swiss_army_knife::try_to_own::TryToOwnInPlace;
use swiss_army_knife::a_to_z::Colon;
use swiss_army_knife::a_to_z::Hyphen;
use swiss_army_knife::a_to_z::Period;
use swiss_army_knife::a_to_z::_0;
use swiss_army_knife::a_to_z::_9;
use swiss_army_knife::a_to_z::a;
use swiss_army_knife::a_to_z::z;
use swiss_army_knife::a_to_z::A;
use swiss_army_knife::a_to_z::Z;
use swiss_army_knife::a_to_z::Hash;
use swiss_army_knife::a_to_z::QuestionMark;
use swiss_army_knife::a_to_z::Underscore;
use swiss_army_knife::chars::SpaceChar;
use swiss_army_knife::chars::TabChar;
use swiss_army_knife::chars::PeriodChar;
use swiss_army_knife::chars::HyphenChar;
use swiss_army_knife::chars::_0Char;
use swiss_army_knife::chars::_9Char;
use swiss_army_knife::chars::UnderscoreChar;
use swiss_army_knife::chars::ColonChar;
use swiss_army_knife::chars::AChar;
use swiss_army_knife::chars::ZChar;
use swiss_army_knife::chars::aChar;
use swiss_army_knife::chars::zChar;
use swiss_army_knife::const_small_vec::ConstSmallVec;
use swiss_army_knife::from_unchecked::FromUnchecked;
use swiss_army_knife::get_checked::PopFirst;
use swiss_army_knife::get_unchecked::GetUnchecked;
use swiss_army_knife::utf8::Utf8SequencesParser;
use swiss_army_knife::utf8::UnvalidatedDecodeUtf8Sequences;
use swiss_army_knife::utf8::utf8_sequence::Utf8SequenceAndCharacter;
use swiss_army_knife::vec::VecExt;


mod chars;


/// Internationalized Resource Identifier (IRI).
pub mod internationalized_resource_identifier;


/// Parser.
pub mod parser;


/// Predicate.
pub mod predicate;


/// String literals map.
pub mod string_literals_map;


include!("BlankNodeLabel.rs");
include!("NaiveIetfBcp47LanguageTag.rs");
include!("NonEmptyVec.rs");
include!("NTriples.rs");
include!("Objects.rs");
include!("PathDepth.rs");
include!("Subject.rs");
