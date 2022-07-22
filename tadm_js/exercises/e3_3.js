import { strict as assert } from 'assert'

// https://leetcode.com/problems/reverse-linked-list/
function reverse_list(l) {
  if (l == null || l.next == null) {
    return l
  }

  let prev = l
  let cur = l.next
  let next

  while (cur != null) {
    next = cur.next

    cur.next = prev

    prev = cur

    cur = next
  }

  l.next = null

  return prev
}

// test reverse

assert(
  list_equal(
    reverse_list({
      val: 1,
      next: { val: 2, next: { val: 3, next: null } },
    }),
    {
      val: 3,
      next: { val: 2, next: { val: 1, next: null } },
    },
  ),
)

// test list_equal

function list_equal(l, r) {
  if (l == null && r == null) {
    return true
  } else if ((l == null && r != null) || (l != null && r == null)) {
    return false
  }

  if (l.val != r.val) {
    return false
  }

  if (l.next == null && r.next == null) {
    return true
  } else if (
    (l.next != null && r.next == null) ||
    (l.next == null && r.next != null)
  ) {
    return false
  } else {
    return list_equal(l.next, r.next)
  }
}

assert(
  list_equal(
    { val: 100, next: { val: 20, next: { val: null } } },
    { val: 100, next: { val: 20, next: { val: null } } },
  ),
)

assert(list_equal(null, null))

assert(!list_equal(null, { val: 2, next: null }))

assert(list_equal({ val: 1, next: null }, { val: 1, next: null }))

assert(!list_equal({ val: 2, next: null }, { val: 1, next: null }))
