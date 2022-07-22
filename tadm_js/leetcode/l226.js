// https://leetcode.com/problems/invert-binary-tree/

function invertTree(root) {
  if (root == null) {
    return root
  }

  let left = root.left
  root.left = invertTree(root.right)
  root.right = invertTree(left)

  return root
}

console.log(
  invertTree({
    val: 4,
    left: {
      val: 2,
      left: {
        val: 1,
        left: null,
        right: null,
      },
      right: {
        val: 3,
        left: null,
        right: null,
      },
    },
    right: {
      val: 7,
      left: {
        val: 6,
        left: null,
        right: null,
      },
      right: {
        val: 9,
        left: null,
        right: null,
      },
    },
  }),
)
