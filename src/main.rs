use std::{cmp::min, env::args, fmt::Debug, fs::read, str::from_utf8};

use nom::{
	bytes::complete::{tag, take},
	combinator::value,
	multi::many0,
	number::complete::be_u32,
	sequence::{preceded, tuple},
	IResult,
};

// #[derive(Debug)]
struct Chunk {
	r#type: String,
	body: Vec<u8>,
}

impl Debug for Chunk {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(
			f,
			"{}: {:?}",
			&self.r#type,
			&Body(&self.body)
		)
	}
}

struct Body<'a>(&'a [u8]);

impl<'a> Debug for Body<'a> {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		let len = self.0.len();
		write!(
			f,
			"{}",
			self.0
				.iter()
				.take(min(16, len))
				.map(|a| format!("{a:0>2X}"))
				.collect::<Vec<_>>()
				.join(" ")
		)
	}
}

fn main() {
	let f = read(
		args()
			.nth(1)
			.unwrap_or("./sample.png".to_owned()),
	)
	.unwrap();

	let res = preceded(sig, many0(chuck))(&f)
		.unwrap()
		.1;

	println!("{res:#?}");
}

fn sig(i: &[u8]) -> IResult<&[u8], ()> {
	value(
		(),
		tag([137, 80, 78, 71, 13, 10, 26, 10]),
	)(i)
}

fn chuck(i: &[u8]) -> IResult<&[u8], Chunk> {
	let (res, len) = be_u32(i)?;

	let (res, (r#type, body, crc)) =
		tuple((take(4usize), take(len), take(4usize)))(res)?;

	Ok((res, Chunk {
		r#type: from_utf8(r#type).unwrap().to_owned(),
		body: body.to_owned(),
	}))
}
