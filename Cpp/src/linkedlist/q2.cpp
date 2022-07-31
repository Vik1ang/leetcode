#include "data_structure.h"

namespace q2 {
	class Solution {
	public:
		ListNode* addTwoNumbers(ListNode* l1, ListNode* l2) {
			ListNode* head = nullptr;
			ListNode* tail = nullptr;

			int carry = 0;
			while (l1 || l2)
			{
				int node1 = l1 ? l1->val : 0;
				int node2 = l2 ? l2->val : 0;
				int sum = node1 + node2 + carry;

				if (head) {
					tail->next = new ListNode(sum % 10);
					tail = tail->next;
				}
				else {
					head = tail = new ListNode(sum % 10);
				}

				carry = sum / 10;

				if (l1) l1 = l1->next;
				if (l2) l2 = l2->next;
			}

			if (carry > 0) {
				tail->next = new ListNode(carry);
			}

			return head;
		}
	};
}