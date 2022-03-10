// This file is part of olympus-xmp. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/raphaelcohn/olympus-xmp/master/COPYRIGHT. No part of olympus-xmp, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2022 The developers of olympus-xmp. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/raphaelcohn/olympus-xmp/master/COPYRIGHT.


/// Number of subseconds (zero based).
#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub enum Subsecond
{
	#[allow(missing_docs)]
	Decisecond(Decisecond),
	
	#[allow(missing_docs)]
	Centisecond(Centisecond),
	
	#[allow(missing_docs)]
	Millisecond(Millisecond),
	
	/// Or Dimisecond.
	Decimillisecond(Decimillisecond),
	
	/// Obscure.
	Centimillisecond(Centimillisecond),
	
	#[allow(missing_docs)]
	Microsecond(Microsecond),
}
