// This file is part of olympus-xmp. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/raphaelcohn/olympus-xmp/master/COPYRIGHT. No part of olympus-xmp, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2022 The developers of olympus-xmp. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/raphaelcohn/olympus-xmp/master/COPYRIGHT.


trait EventWriterExt
{
	fn write_robustly(&mut self, xml_write_event: XmlWriteEvent) -> Result<(), io::Error>;
}

impl<W: Write> EventWriterExt for EventWriter<W>
{
	fn write_robustly(&mut self, xml_write_event: XmlWriteEvent) -> Result<(), io::Error>
	{
		use XmlWriteError::*;
		match self.write(xml_write_event)
		{
			Ok(()) => Ok(()),
			
			Err(Io(io)) => Err(io),
			
			Err(unreachable) => unreachable!("Error {} should not be possible", unreachable)
		}
	}
}