use anyhow::Result;
use yazi_macro::{act, render, succ};
use yazi_parser::ArrowOpt;
use yazi_shared::data::Data;

use crate::{Actor, Ctx};

pub struct ParentArrow;

impl Actor for ParentArrow {
	type Options = ArrowOpt;

	const NAME: &str = "parent-arrow";

	fn act(cx: &mut Ctx, opt: Self::Options) -> Result<Data> {
		let tab = cx.tab_mut();
		
		// Scroll the parent folder if it exists
		if let Some(parent) = &mut tab.parent {
			// Call arrow to update offset and cursor
			parent.arrow(opt.step);
			
			// Force render to update the view
			succ!(render!());
		}
		
		succ!();
	}
}
