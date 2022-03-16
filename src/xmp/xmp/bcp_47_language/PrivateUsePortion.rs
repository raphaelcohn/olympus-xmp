// This file is part of olympus-xmp. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/raphaelcohn/olympus-xmp/master/COPYRIGHT. No part of olympus-xmp, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2022 The developers of olympus-xmp. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/raphaelcohn/olympus-xmp/master/COPYRIGHT.


/// `"-" (1*8alphanum)`.
#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub enum PrivateUsePortion
{
	#[allow(missing_docs)]
	_1([Alphanumeric; 1]),
	
	#[allow(missing_docs)]
	_2([Alphanumeric; 2]),
	
	#[allow(missing_docs)]
	_3([Alphanumeric; 3]),
	
	#[allow(missing_docs)]
	_4([Alphanumeric; 4]),
	
	#[allow(missing_docs)]
	_5([Alphanumeric; 5]),
	
	#[allow(missing_docs)]
	_6([Alphanumeric; 6]),
	
	#[allow(missing_docs)]
	_7([Alphanumeric; 7]),
	
	#[allow(missing_docs)]
	_8([Alphanumeric; 8]),
}
