// https://leetcode.com/problems/balance-a-binary-search-tree/

function balanceBST(root) {
  let vals = []

  function traverse(root) {
    if (root == null) {
      return
    }

    traverse(root.left)

    vals.push(root.val)

    traverse(root.right)
  }

  traverse(root)

  function create(low, high) {
    if (high < low) {
      return null
    } else if (low == high) {
      return {
        val: vals[low],
        left: null,
        right: null,
      }
    } else {
      let mid = Math.floor(low + (high - low) / 2)
      return {
        val: vals[mid],
        left: create(low, mid - 1),
        right: create(mid + 1, high),
      }
    }
  }

  return create(0, vals.length - 1)
}

balanceBST({
  val: 1,
  left: null,
  right: {
    val: 2,
    left: null,
    right: {
      val: 3,
      left: null,
      right: {
        val: 4,
        left: null,
        right: null,
      },
    },
  },
})
