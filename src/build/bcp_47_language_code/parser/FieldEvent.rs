// This file is part of olympus-xmp. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/raphaelcohn/olympus-xmp/master/COPYRIGHT. No part of olympus-xmp, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2022 The developers of olympus-xmp. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/raphaelcohn/olympus-xmp/master/COPYRIGHT.


#[derive(Debug, PartialEq, Eq)]
pub(super) struct FieldEvent<'a>
{
	line: Cow<'a, str>,
	
	field_name_exclusive_end_index: usize,
	
	field_body_inclusive_start_index: usize,
}

impl<'a> FieldEvent<'a>
{
	#[inline(always)]
	pub(super) fn field_name(&self) -> &str
	{
		&self.line.as_ref()[.. self.field_name_exclusive_end_index]
	}
	
	#[inline(always)]
	pub(super) fn field_body(&self) -> &str
	{
		&self.line.as_ref()[self.field_body_inclusive_start_index .. ]
	}
}
