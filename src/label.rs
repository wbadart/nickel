use std::cell::RefCell;
use std::rc::Rc;

#[derive(Debug, Clone, PartialEq)]
pub enum Label {
    Root(RLabel),
    // Function labels
    Dom(Box<Label>, Rc<RefCell<bool>>),
    Codom(Box<Label>, Rc<RefCell<bool>>),
    // Intersection labels
    Inter(Box<Label>, Rc<RefCell<bool>>, Rc<RefCell<bool>>),
    // Union labels
    Union(Box<Label>, Rc<RefCell<bool>>, Rc<RefCell<bool>>),
    // Guarded label (identical to Codom)
    Guard(Box<Label>, Rc<RefCell<bool>>),
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
            println!("crazy domain");
            solve_label(*l, !pol)
        }
        Label::Codom(l, sb) => {
            if *sb.borrow() {
                println!("Crazy function");
                Ok(())
            } else {
                solve_label(*l, pol)
            }
        }
        Label::Inter(l, sa, sb) => {
            if pol {
                solve_label(*l, pol)
            } else {
                sa.replace(true);
                if *sb.borrow() {
                    solve_label(*l, pol)
                } else {
                    Ok(())
                }
            }
        }
        Label::Union(l, sa, sb) => {
            if !pol {
                solve_label(*l, pol)
            } else {
                sa.replace(true);
                if *sb.borrow() {
                    solve_label(*l, pol)
                } else {
                    Ok(())
                }
            }
        }
        Label::Guard(l, sb) => {
            // Only guards negative context
            if *sb.borrow() || pol {
                println!("Unguarded lbl");
                solve_label(*l, pol)
            } else {
                println!("Guarded lbl");
                Ok(())
            }
        }
    }
}
