// This file is part of olympus-xmp. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/raphaelcohn/olympus-xmp/master/COPYRIGHT. No part of olympus-xmp, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2022 The developers of olympus-xmp. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/raphaelcohn/olympus-xmp/master/COPYRIGHT.


/// Subject.
#[derive(Debug, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub enum Subject<'a>
{
	#[allow(missing_docs)]
	IRI(IRI<'a>),
	
	#[allow(missing_docs)]
	BlankNode(BlankNodeLabel<'a>),
}

impl<'a> Subject<'a>
{
	// #[inline(always)]
	// fn m49(m49_code: StaticM49Code)
	// {
	// 	const SubjectPrefix: &'static str = "http://stats-class.fao.uniroma2.it/geo/m49/";
	// 	let mut string = String::with_capacity(SubjectPrefix.len() + 3);
	// 	string.push_str(SubjectPrefix);
	// 	(unsafe { string.as_mut_vec() }).extend_from_slice(m49_code);
	//
	// 	Subject::IRI(IRIREF::from(string));
	// }
}
