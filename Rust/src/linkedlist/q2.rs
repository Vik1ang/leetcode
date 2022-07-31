use crate::data_structure::data_structure::ListNode;

pub struct Solution1 {}

impl Solution1 {
    pub fn add_two_numbers(
        l1: Option<Box<ListNode>>,
        l2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        let mut head = None;
        let mut tail = &mut head;

        let mut tup = (l1, l2, 0, 0); // (l1, l2, sum, carry)

        // pattern match
        // 穷举所有可能

        // [9,9,9,9,9,9,9]
        // [9,9,9,9]
        loop {
            tup = match tup {
                (None, None, _, carry) => match carry {
                    0 => break,
                    _ => (None, None, carry, 0),
                },
                (Some(l), None, _, carry) | (None, Some(l), _, carry) => {
                    (l.next, None, (l.val + carry) % 10, (l.val + carry) / 10)
                }
                (Some(l1), Some(l2), _, carry) => (
                    l1.next,
                    l2.next,
                    (l1.val + l2.val + carry) % 10,
                    (l1.val + l2.val + carry) / 10,
                ),
            };

            *tail = Some(Box::new(ListNode::new(tup.2)));
            tail = &mut tail.as_mut().unwrap().next;
        }

        head
    }
}

pub struct Solution2 {}

impl Solution2 {
    pub fn add_two_numbers(
        l1: Option<Box<ListNode>>,
        l2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        todo!()
    }
    // let mut head: Option<Box<ListNode>> = None;
    // let mut tail: Option<Box<ListNode>> = None;
    //
    // let (mut p1, mut p2) = (l1, l2);
    // let mut carry = 0;
    //
    // while p1.is_some() || p2.is_some() {
    //     let mut sum = carry;
    //     if let Some(v) = p1 {
    //         sum += v.val;
    //         p1 = v.next;
    //     }
    //     if let Some(v) = p2 {
    //         sum += v.val;
    //         p2 = v.next;
    //     }
    //     carry = sum / 10;
    //
    //     match head {
    //         None => {
    //             head = Some(Box::new(ListNode::new(carry)));
    //             tail = head.unwrap().next;
    //         }
    //         Some(_) => {
    //             tail = Some(Box::new(ListNode::new(carry)));
    //             tail = head.unwrap().next;
    //         }
    //     }
    // }
    //
    // if carry > 0 {
    //     tail = Some(Box::new(ListNode::new(carry)));
    //     tail = head.unwrap().next;
    // }
    //
    // head
    // }
}
