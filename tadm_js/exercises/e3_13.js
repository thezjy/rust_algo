// https://leetcode.com/problems/recover-binary-search-tree/

import { strict as assert } from 'assert'

function recoverTree(root) {
  if (root == null || (root.left == null && root.right == null)) {
    return
  }

  let root_and_left_max
  if (root.left == null) {
    root_and_left_max = root
  } else {
    let left_max = max(root.left)
    root_and_left_max = left_max.val > root.val ? left_max : root
  }

  let root_and_right_min
  if (root.right == null) {
    root_and_right_min = root
  } else {
    let right_min = min(root.right)
    root_and_right_min = right_min.val < root.val ? right_min : root
  }

  if (root_and_left_max.val > root_and_right_min.val) {
    let temp = root_and_left_max.val
    root_and_left_max.val = root_and_right_min.val
    root_and_right_min.val = temp
  } else {
    recoverTree(root.left)
    recoverTree(root.right)
  }
}

function isValidBST(root) {
  if (root == null || (root.left == null && root.right == null)) {
    return true
  }

  let root_and_left_max
  if (root.left == null) {
    root_and_left_max = root
  } else {
    let left_max = max(root.left)
    root_and_left_max = left_max.val > root.val ? left_max : root
  }

  let root_and_right_min
  if (root.right == null) {
    root_and_right_min = root
  } else {
    let right_min = min(root.right)
    root_and_right_min = right_min.val < root.val ? right_min : root
  }

  if (root_and_left_max.val > root_and_right_min.val) {
    return false
  } else {
    return isValidBST(root.left) && isValidBST(root.right)
  }
}

function min(node) {
  let min_node = node

  if (node.left != null) {
    let left_min = min(node.left)
    if (left_min.val < min_node.val) {
      min_node = left_min
    }
  }

  if (node.right != null) {
    let right_min = min(node.right)
    if (right_min.val < min_node.val) {
      min_node = right_min
    }
  }

  return min_node
}

function max(node) {
  let max_node = node

  if (node.left != null) {
    let left_max = max(node.left)
    if (left_max.val > max_node.val) {
      max_node = left_max
    }
  }

  if (node.right != null) {
    let right_max = max(node.right)
    if (right_max.val > max_node.val) {
      max_node = right_max
    }
  }

  return max_node
}

function treeEqual(l, r) {
  if ((l == null && r != null) || (l != null && r == null)) {
    return false
  } else if (l == null && r == null) {
    return true
  } else {
    // l != null && r != null

    if (l.val != r.val) {
      return false
    }

    return treeEqual(l.left, r.left) && treeEqual(l.right, r.right)
  }
}

let t1 = {
  val: 1,
  left: {
    val: 3,
    left: null,
    right: {
      val: 2,
      left: null,
      right: null,
    },
  },
  right: null,
}

assert(
  treeEqual(t1, {
    val: 1,
    left: {
      val: 3,
      left: null,
      right: {
        val: 2,
        left: null,
        right: null,
      },
    },
    right: null,
  }),
)

recoverTree(t1)

assert(
  treeEqual(t1, {
    val: 3,
    left: {
      val: 1,
      left: null,
      right: {
        val: 2,
        left: null,
        right: null,
      },
    },
    right: null,
  }),
)

let l2 = {
  val: 3,
  left: {
    val: 1,
    left: null,
    right: null,
  },
  right: {
    val: 4,
    left: {
      val: 2,
      left: null,
      right: null,
    },
    right: null,
  },
}

recoverTree(l2)

assert(
  treeEqual(l2, {
    val: 2,
    left: {
      val: 1,
      left: null,
      right: null,
    },
    right: {
      val: 4,
      left: {
        val: 3,
        left: null,
        right: null,
      },
      right: null,
    },
  }),
)

let l3 = {
  val: 2,
  left: null,
  right: {
    val: 4,
    left: {
      val: 3,
      left: {
        val: 1,
        left: null,
        right: null,
      },
      right: null,
    },
    right: null,
  },
}

recoverTree(l3)

assert(
  treeEqual(l3, {
    val: 1,
    left: null,
    right: {
      val: 4,
      left: {
        val: 3,
        left: {
          val: 2,
          left: null,
          right: null,
        },
        right: null,
      },
      right: null,
    },
  }),
)
