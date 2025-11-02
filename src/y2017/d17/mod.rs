use std::{cell::RefCell, rc::Rc};

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: u32,
    pub next: Option<Rc<RefCell<ListNode>>>,
}

impl ListNode {
    pub fn new(val: u32) -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(ListNode { val, next: None }))
    }
}

fn get_value(step: u32) -> u32 {
    let node_0 = ListNode::new(0);
    node_0.borrow_mut().next = Some(node_0.clone());

    let mut num = 1;
    let mut ptr = node_0.clone();

    for _ in 0..2017 {
        for _ in 0..step as usize {
            let next = Rc::clone(ptr.borrow().next.as_ref().unwrap());
            ptr = next;
        }

        let node = ListNode::new(num);
        num += 1;
        node.borrow_mut().next = Some(ptr.borrow().next.as_ref().unwrap().clone());
        ptr.borrow_mut().next = Some(node.clone());
        ptr = node;
    }

    ptr = node_0.clone();
    while ptr.borrow().val != 2017 {
        let next = ptr.borrow().next.as_ref().unwrap().clone();
        ptr = next;
    }
    ptr.borrow().next.as_ref().unwrap().borrow().val
}

fn after_zero(step: u32, total_count: usize) -> u32 {
    let mut len = 1;
    let mut curr_idx = 0;
    let mut prev: u32 = 0;

    for _ in 0..total_count {
        curr_idx = (curr_idx + step) % len + 1;
        if curr_idx == 1 {
            prev = len;
        }
        len += 1;
    }

    prev
}

pub fn run(input: &str) {
    let step: u32 = input.parse().unwrap();
    println!("{}", get_value(step));
    println!("{}", after_zero(step, 50_000_000));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(get_value(3), 638);
    }

    #[test]
    fn test2() {
        let testcases = [
            [3, 4, 2],
            [3, 5, 5],
            [3, 6, 5],
            [3, 7, 5],
            [3, 8, 5],
            [3, 9, 9],
        ];

        for tc in testcases {
            assert_eq!(after_zero(tc[0], tc[1] as usize), tc[2]);
        }
    }
}
