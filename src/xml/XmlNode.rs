// This file is part of olympus-xmp. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/raphaelcohn/olympus-xmp/master/COPYRIGHT. No part of olympus-xmp, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2022 The developers of olympus-xmp. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/raphaelcohn/olympus-xmp/master/COPYRIGHT.


#[derive(Debug, Clone, PartialEq, Eq)]
enum XmlNode
{
	ProcessingInstruction(XmlProcessingInstruction),
	
	Element(XmlElement),
	
	Text(String),
}

impl XmlNode
{
	#[inline(always)]
	fn write(&self, event_writer: &mut EventWriter<impl Write>) -> Result<(), io::Error>
	{
		use XmlNode::*;
		match self
		{
			ProcessingInstruction(processing_instruction) => processing_instruction.write(event_writer),
			
			Element(element) => element.write(event_writer),
			
			Text(text) => event_writer.write_robustly(XmlWriteEvent::Characters(text.as_str())),
		}
	}
	
	#[inline(always)]
	fn processing_instruction(name: String, data: Option<String>) -> Self
	{
		XmlNode::ProcessingInstruction(XmlProcessingInstruction::new(name, data))
	}
}