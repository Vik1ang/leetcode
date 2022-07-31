using Csharp.DataStructure;

namespace Csharp.LinkedList.Q2;

public class Solution1
{
    public ListNode AddTwoNumbers(ListNode l1, ListNode l2)
    {
        ListNode? head = null;
        ListNode? tail = null;

        int carry = 0;

        while (l1 != null || l2 != null)
        {
            int node1 = l1?.val ?? 0;
            int node2 = l2?.val ?? 0;
            int sum = node1 + node2 + carry;
            if (head == null)
            {
                head = tail = new ListNode(sum % 10);
            }
            else
            {
                tail!.next = new ListNode(sum % 10);
                tail = tail.next;
            }

            carry = sum / 10;

            l1 = l1?.next!;
            l2 = l2?.next!;
        }

        if (carry > 0)
        {
            tail!.next = new ListNode(carry);
        }

        return head!;
    }
}