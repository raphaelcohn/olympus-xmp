// This file is part of olympus-xmp. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/raphaelcohn/olympus-xmp/master/COPYRIGHT. No part of olympus-xmp, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2022 The developers of olympus-xmp. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/raphaelcohn/olympus-xmp/master/COPYRIGHT.


/// An XML schema integer.
///
/// Whilst the XML schema definition permits a signed integer of any size, we restrict to an `i128`.
/// This is likely to cover the overwhelming vast majority of scenarios, especially as back in 2001, when this was defined, Java\* was the only common mainstream language to make support for big integers trivial.
///
/// \*Smalltalk was not mainstream.
pub type Integer = i128;
