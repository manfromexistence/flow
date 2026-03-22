use anyhow::Result;
use yazi_macro::{render, succ};
use yazi_parser::ArrowOpt;
use yazi_shared::data::Data;

use crate::{Actor, Ctx};

pub struct ParentArrow;

impl Actor for ParentArrow {
	type Options = ArrowOpt;

	const NAME: &str = "parent_arrow";

	fn act(cx: &mut Ctx, opt: Self::Options) -> Result<Data> {
		let Some(parent) = cx.parent_mut() else { succ!() };
		if !parent.arrow(opt.step) {
			succ!();
		}
		succ!(render!());
	}
}
