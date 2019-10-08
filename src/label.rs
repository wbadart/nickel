use std::cell::RefCell;
use std::rc::Rc;

#[derive(Debug, Clone, PartialEq)]
pub enum Label {
    Root(RLabel),
    Dom(Box<Label>, Rc<RefCell<bool>>),
    Codom(Box<Label>, Rc<RefCell<bool>>),
}

#[derive(Debug, Clone, PartialEq)]
pub struct RLabel {
    pub tag: String,
    pub polarity: bool,
    pub l: usize,
    pub r: usize,
}

pub fn solve_label(l: Label, pol: bool) -> Result<(), RLabel> {
    match l {
        Label::Root(mut rl) => {
            rl.polarity = pol;
            Err(rl)
        }
        Label::Dom(l, sb) => {
            sb.replace(true);
            solve_label(*l, !pol)
        }
        Label::Codom(l, sb) => {
            if *sb.borrow() {
                Ok(())
            } else {
                solve_label(*l, pol)
            }
        }
    }
}
