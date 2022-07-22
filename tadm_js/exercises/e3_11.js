// https://leetcode.com/problems/maximum-depth-of-binary-tree/
import { strict as assert } from 'assert'

/**
 * Definition for a binary tree node.
 * function TreeNode(val, left, right) {
 *     this.val = (val===undefined ? 0 : val)
 *     this.left = (left===undefined ? null : left)
 *     this.right = (right===undefined ? null : right)
 * }
 */
/**
 * @param {TreeNode} root
 * @return {number}
 */
function maxDepth(root) {
  if (root == null) {
    return 0
  }

  return 1 + Math.max(maxDepth(root.left), maxDepth(root.right))
}

assert.equal(
  maxDepth({
    val: 3,
    left: { val: 9, left: null, right: null },
    right: {
      val: 20,
      left: { val: 50, left: null, right: null },
      right: { val: 7, left: null, right: null },
    },
  }),
  3,
)
