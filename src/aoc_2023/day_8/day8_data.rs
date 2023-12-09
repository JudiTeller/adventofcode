use std::cell::RefCell;
use std::rc::Rc;

#[derive(Debug, Clone)]
pub struct GraphNode {
    pub name: String,
    pub left: Option<Rc<RefCell<GraphNode>>>,
    pub right: Option<Rc<RefCell<GraphNode>>>,
}