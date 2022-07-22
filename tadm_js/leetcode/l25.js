// https://leetcode.com/problems/reverse-nodes-in-k-group/

function switchVal(a, b) {
  let temp = a.val
  a.val = b.val
  b.val = temp
}

function reverseKGroup(head, k) {
  let current = head

  while (true) {
    // k or 0
    let nodesToReverse = []
    let walkedCount = 0

    while (walkedCount < k && current != null) {
      nodesToReverse.push(current)
      current = current.next
      walkedCount += 1
    }

    if (nodesToReverse.length != k) {
      break
    } else {
      for (let i = 0; i < k / 2; i++) {
        switchVal(nodesToReverse[i], nodesToReverse[k - i - 1])
      }
    }
  }

  return head
}

console.log(
  reverseKGroup(
    {
      val: 1,
      next: {
        val: 2,
        next: null,
      },
    },
    2,
  ),
)

console.log(
  reverseKGroup(
    {
      val: 1,
      next: {
        val: 2,
        next: {
          val: 3,
          next: {
            val: 4,
            next: {
              val: 5,
              next: null,
            },
          },
        },
      },
    },
    2,
  ),
)
