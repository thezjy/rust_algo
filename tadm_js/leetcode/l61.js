// https://leetcode.com/problems/rotate-list/

function rotateRight(head, k) {
  if (head == null || head.next == null) {
    return
  }

  let last = head
  let len = 1
  while (last.next != null) {
    last = last.next
    len += 1
  }

  if (len < 2) {
    return
  }

  let rotate_count = k % len

  if (rotate_count == 0) {
    return
  }

  last.next = head

  let new_head_index = len - rotate_count

  let prev_of_new_head = head

  for (let i = 0; i < new_head_index - 1; i++) {
    prev_of_new_head = prev_of_new_head.next
  }

  let new_head = prev_of_new_head.next
  prev_of_new_head.next = null
  return new_head
}

console.log(
  rotateRight(
    {
      value: 0,
      next: {
        value: 1,
        next: { value: 2, next: null },
      },
    },
    4,
  ),
)
