// This file is part of olympus-xmp. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/raphaelcohn/olympus-xmp/master/COPYRIGHT. No part of olympus-xmp, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2022 The developers of olympus-xmp. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/raphaelcohn/olympus-xmp/master/COPYRIGHT.


#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub(super) enum Abbreviations
{
	AbsentRevision3Onwards,
	
	Revision3Onwards(Revision3Or4Abbreviations),
	
	/// Present in Revision 1 (1975) and Revision 2 (1982).
	Revision1AndRevision2(Revision1Or2Abbreviations),
	
	/// Present only in the original (1970).
	Revision0(Revision0Abbreviations),
}
