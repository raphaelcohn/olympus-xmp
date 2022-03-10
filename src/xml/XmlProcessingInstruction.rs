// This file is part of olympus-xmp. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/olympus-xmp/master/COPYRIGHT. No part of olympus-xmp, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2022 The developers of olympus-xmp. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/olympus-xmp/master/COPYRIGHT.


#[derive(Debug, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
struct XmlProcessingInstruction
{
	name: String,
	
	data: Option<String>,
}

impl XmlProcessingInstruction
{
	#[inline(always)]
	fn new(name: String, data: Option<String>) -> Self
	{
		Self
		{
			name,
			data,
		}
	}
	
	#[inline(always)]
	fn write(&self, event_writer: &mut EventWriter<impl Write>) -> Result<(), io::Error>
	{
		event_writer.write_robustly(XmlWriteEvent::ProcessingInstruction { name: &self.name, data: self.data.as_ref().map(|string| string.as_str()) })
	}
}