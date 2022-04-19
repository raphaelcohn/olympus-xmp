// This file is part of olympus-xmp. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/raphaelcohn/olympus-xmp/master/COPYRIGHT. No part of olympus-xmp, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2022 The developers of olympus-xmp. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/raphaelcohn/olympus-xmp/master/COPYRIGHT.


/// N-Triples.
#[derive(Default, Debug, Clone, Eq, PartialEq)]
pub struct NTriples<'a>(HashMap<Subject<'a>, HashMap<Predicate<'a>, Objects<'a>>>);

impl<'a> NTriples<'a>
{
	/// Predicates.
	#[inline(always)]
	pub fn predicates<'b, 's>(&'s self, subject: &Subject<'b>) -> Option<&'s HashMap<Predicate<'a>, Objects<'a>>>
	{
		self.0.get(unsafe { transmute(subject) })
	}
	
	/// Parses using the official specification at <https://www.w3.org/TR/n-triples/>.
	#[inline(always)]
	pub fn parse(n_triples_file_bytes: &'a [u8]) -> Result<Self, NTriplesParseError>
	{
		let mut this = Self::default();
		
		let mut remaining_bytes = n_triples_file_bytes;
		while !remaining_bytes.is_empty()
		{
			let (n_triple, option_remaining_bytes) = NTriple::parse(remaining_bytes).map_err(NTriplesParseError::NTripleParse)?;
			
			this.push(n_triple)?;
			
			match option_remaining_bytes
			{
				None => break,
				
				Some(option_remaining_bytes) => remaining_bytes = option_remaining_bytes,
			}
		}
		
		Ok(this)
	}
	
	#[inline(always)]
	fn push(&mut self, n_triple: NTriple<'a>) -> Result<(), NTriplesParseError>
	{
		let NTriple { subject, predicate, object } = n_triple;
		
		let predicates = self.0.entry(subject).or_default();
		let objects = predicates.entry(predicate).or_default();
		objects.push(object).map_err(NTriplesParseError::OutOfMemory)
	}
}
