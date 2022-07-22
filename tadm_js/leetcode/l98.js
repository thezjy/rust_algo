// https://leetcode.com/problems/validate-binary-search-tree/

import { strict as assert } from 'assert'

function isValidBST(root) {
  if (root == null || (root.left == null && root.right == null)) {
    return true
  }

  if (root.left != null && root.val <= maxVal(root.left)) {
    return false
  } else if (root.right != null && root.val >= minVal(root.right)) {
    return false
  } else {
    return isValidBST(root.left) && isValidBST(root.right)
  }
}

function minVal(node) {
  let min = node.val

  if (node.left != null) {
    let left_min = minVal(node.left)
    if (left_min < min) {
      min = left_min
    }
  }

  if (node.right != null) {
    let right_min = minVal(node.right)
    if (right_min < min) {
      min = right_min
    }
  }

  return min
}

function maxVal(node) {
  let max = node.val

  if (node.left != null) {
    let left_max = maxVal(node.left)
    if (left_max > max) {
      max = left_max
    }
  }

  if (node.right != null) {
    let right_max = maxVal(node.right)
    if (right_max > max) {
      max = right_max
    }
  }

  return max
}

assert(
  isValidBST({
    val: 2,
    left: { val: 1, left: null, right: null },
    right: { val: 3, left: null, right: null },
  }),
)
