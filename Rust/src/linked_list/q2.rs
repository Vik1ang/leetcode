pub use crate::data_structure::ListNode;

trait Q2 {
    fn add_two_numbers(
        &self,
        l1: Option<Box<ListNode>>,
        l2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>>;
}

struct Solution1;
impl Q2 for Solution1 {
    fn add_two_numbers(
        &self,
        l1: Option<Box<ListNode>>,
        l2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        let mut head = None;
        let mut tail = &mut head;

        let mut tup = (l1, l2, 0, 0); // (l1, l2, sum, carry)

        // pattern match
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

pub fn solution() {
    println!("q2")
}
