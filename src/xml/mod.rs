// This file is part of olympus-xmp. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/raphaelcohn/olympus-xmp/master/COPYRIGHT. No part of olympus-xmp, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2022 The developers of olympus-xmp. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/raphaelcohn/olympus-xmp/master/COPYRIGHT.


use std::borrow::Cow::Borrowed;
use std::borrow::Cow::Owned;
use std::borrow::Cow;
use std::collections::HashMap;
use std::error;
use std::fmt;
use std::fmt::Debug;
use std::fmt::Display;
use std::fmt::Formatter;
use std::fs::File;
use std::fs::OpenOptions;
use std::io::BufReader;
use std::io::BufWriter;
use std::io::Read;
use std::io::Write;
use std::io;
use std::ops::Deref;
use std::path::Path;
use xml::EmitterConfig;
use xml::EventReader;
use xml::EventWriter;
use xml::ParserConfig;
use xml::attribute::Attribute;
use xml::attribute::OwnedAttribute;
use xml::common::Position;
use xml::common::TextPosition;
use xml::common::XmlVersion;
use xml::name::Name;
use xml::name::OwnedName;
use xml::namespace::Namespace;
use xml::reader::Error as XmlReadError;
use xml::reader::XmlEvent::*;
use xml::writer::Error as XmlWriteError;
use xml::writer::XmlEvent as XmlWriteEvent;


include!("unreachable_cdata_comments_or_whitespace.rs");
include!("unreachable_end_element.rs");
include!("unreachable_only_start_document.rs");
include!("unreachable_start_document.rs");
include!("EventWriterExt.rs");
include!("NotExactlyOneElementError.rs");
include!("NotExactlyOneTextError.rs");
include!("ParseError.rs");
include!("StringToNamespacedXmlName.rs");
include!("XmlDocument.rs");
include!("XmlElement.rs");
include!("XmlElementCommon.rs");
include!("XmlDocumentOrXmlElement.rs");
include!("XmlName.rs");
include!("XmlNode.rs");
include!("XmlProcessingInstruction.rs");
include!("XmlProcessingInstructions.rs");
