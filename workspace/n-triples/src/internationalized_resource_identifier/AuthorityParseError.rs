// This file is part of olympus-xmp. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/raphaelcohn/olympus-xmp/master/COPYRIGHT. No part of olympus-xmp, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2022 The developers of olympus-xmp. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/raphaelcohn/olympus-xmp/master/COPYRIGHT.


/// A parse error.
#[derive(Debug, Clone, Eq, PartialEq)]
pub enum AuthorityParseError
{
	#[allow(missing_docs)]
	UserInformationParse(UserInformationParseError),
	
	#[allow(missing_docs)]
	HostParse(HostParseError),
	
	#[allow(missing_docs)]
	PortParse(PortParseError),
}

impl const From<UserInformationParseError> for AuthorityParseError
{
	#[inline(always)]
	fn from(cause: UserInformationParseError) -> Self
	{
		AuthorityParseError::UserInformationParse(cause)
	}
}

impl const From<HostParseError> for AuthorityParseError
{
	#[inline(always)]
	fn from(cause: HostParseError) -> Self
	{
		AuthorityParseError::HostParse(cause)
	}
}

impl const From<PortParseError> for AuthorityParseError
{
	#[inline(always)]
	fn from(cause: PortParseError) -> Self
	{
		AuthorityParseError::PortParse(cause)
	}
}

impl Display for AuthorityParseError
{
	#[inline(always)]
	fn fmt(&self, formatter: &mut Formatter<'_>) -> fmt::Result
	{
		Debug::fmt(self, formatter)
	}
}

impl error::Error for AuthorityParseError
{
	#[inline(always)]
	fn source(&self) -> Option<&(dyn error::Error + 'static)>
	{
		use AuthorityParseError::*;
		
		match self
		{
			UserInformationParse(cause) => Some(cause),
			
			HostParse(cause) => Some(cause),
			
			PortParse(cause) => Some(cause),
		}
	}
}
