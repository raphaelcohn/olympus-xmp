// This file is part of olympus-xmp. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/raphaelcohn/olympus-xmp/master/COPYRIGHT. No part of olympus-xmp, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2022 The developers of olympus-xmp. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/raphaelcohn/olympus-xmp/master/COPYRIGHT.


fn open_our_module_file_path(relative_module_path: Vec<&str>, file_name: impl AsRef<Path>, capacity_in_kilobytes: usize) -> io::Result<String>
{
	let mut buffer = String::with_capacity(capacity_in_kilobytes * BytesPerKilobyte);
	let mut file = File::open(our_module_file_path(relative_module_path, file_name))?;
	file.read_to_string(&mut buffer)?;
	Ok(buffer)
}
