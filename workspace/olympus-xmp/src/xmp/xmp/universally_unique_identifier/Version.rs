// This file is part of olympus-xmp. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/raphaelcohn/olympus-xmp/master/COPYRIGHT. No part of olympus-xmp, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2022 The developers of olympus-xmp. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/raphaelcohn/olympus-xmp/master/COPYRIGHT.


/// Version.
#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
#[repr(u8)]
enum Version
{
	/// Time-based.
	_1 = 1,
	
	/// DCE security with embedded POSIX UIDs.
	_2 = 2,
	
	/// Name-based using MD5 hashing.
	_3 = 3,
	
	/// Randomly (or pseudo-randomly) generated.
	///
	/// The only modern secure version.
	_4 = 4,
	
	/// Name-based using SHA-1 hashing.
	_5 = 5,
}

impl Version
{
	#[inline(always)]
	fn parse(version: u8) -> Result<Self, VersionParseError>
	{
		debug_assert!(version < 15);
		
		match version
		{
			0 => Err(VersionParseError { version }),
			
			1 ..= 5 => Ok(unsafe { transmute(version) }),
			
			6 ..= 15 => Err(VersionParseError { version }),
			
			_ => unreachable_code_const("4-bit version"),
		}
	}
}
