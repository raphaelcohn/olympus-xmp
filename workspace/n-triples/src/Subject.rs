// This file is part of olympus-xmp. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/raphaelcohn/olympus-xmp/master/COPYRIGHT. No part of olympus-xmp, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2022 The developers of olympus-xmp. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/raphaelcohn/olympus-xmp/master/COPYRIGHT.


/// Subject.
#[derive(Debug, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub enum Subject<'a>
{
	#[allow(missing_docs)]
	AbsoluteInternationalizedResourceIdentifier(AbsoluteInternationalizedResourceIdentifier<'a, PathDepth>),
	
	#[allow(missing_docs)]
	BlankNode(BlankNodeLabel<'a>),
}

impl<'a> TryToOwnInPlace for Subject<'a>
{
	#[inline(always)]
	fn try_to_own_in_place(&mut self) -> Result<(), TryReserveError>
	{
		use Subject::*;
		
		match self
		{
			AbsoluteInternationalizedResourceIdentifier(absolute_internationalized_resource_identifier) => absolute_internationalized_resource_identifier.try_to_own_in_place(),
			
			BlankNode(blank_node_label) => blank_node_label.try_to_own_in_place(),
		}
	}
}

impl<'a> TryToOwn for Subject<'a>
{
	type TryToOwned = Subject<'static>;
	
	#[inline(always)]
	fn try_to_own(mut self) -> Result<Subject<'static>, TryReserveError>
	{
		self.try_to_own_in_place()?;
		Ok(unsafe { transmute(self) })
	}
}

impl Subject<'static>
{
	#[allow(missing_docs)]
	#[inline(always)]
	pub const fn from_absolute_internationalized_resource_identifier_string(absolute_internationalized_resource_identifier: String) -> Self
	{
		Subject::AbsoluteInternationalizedResourceIdentifier(AbsoluteInternationalizedResourceIdentifier::from(absolute_internationalized_resource_identifier))
	}
	
	#[allow(missing_docs)]
	#[inline(always)]
	pub const fn from_blank_label_node_string(blank_label_node: String) -> Self
	{
		Subject::BlankNode(BlankNodeLabel::from(blank_label_node))
	}
}
