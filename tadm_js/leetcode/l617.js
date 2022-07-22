/**
 * Definition for a binary tree node.
 * function TreeNode(val, left, right) {
 *     this.val = (val===undefined ? 0 : val)
 *     this.left = (left===undefined ? null : left)
 *     this.right = (right===undefined ? null : right)
 * }
 */
/**
 * @param {TreeNode} root1
 * @param {TreeNode} root2
 * @return {TreeNode}
 */
var mergeTrees = function (root1, root2) {
  if (root1 == null && root2 == null) {
    return null
  } else if (root1 == null) {
    return root2
  } else if (root2 == null) {
    return root1
  } else {
    // root1 != null && roo2 != null

    root1.val += root2.val

    root1.left = mergeTrees(root1.left, root2.left)
    root1.right = mergeTrees(root1.right, root2.right)

    return root1
  }
}

console.log(
  mergeTrees(
    {
      val: 1,
      left: { val: 3, left: { val: 5, left: null, right: null }, right: null },
      right: { val: 2, left: null, right: null },
    },
    {
      val: 2,
      left: {
        val: 1,
        left: null,
        right: { val: 4, left: null, right: null },
      },
      right: {
        val: 3,
        left: null,
        right: {
          val: 7,
          left: null,
          right: null,
        },
      },
    },
  ),
)
