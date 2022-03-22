// This file is part of olympus-xmp. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/olympus-xmp/master/COPYRIGHT. No part of olympus-xmp, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2022 The developers of olympus-xmp. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/olympus-xmp/master/COPYRIGHT.


#[derive(Debug, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
struct XmlProcessingInstructions(Vec<XmlProcessingInstruction>);

impl XmlProcessingInstructions
{
	#[inline(always)]
	fn new() -> Self
	{
		Self(Vec::new())
	}
	
	#[inline(always)]
	fn push(&mut self, name: String, data: Option<String>)
	{
		self.0.push(XmlProcessingInstruction::new(name, data))
	}
	
	#[inline(always)]
	fn write(&self, event_writer: &mut EventWriter<impl Write>) -> Result<(), io::Error>
	{
		for xml_processing_instruction in &self.0
		{
			xml_processing_instruction.write(event_writer)?
		}
		Ok(())
	}
}