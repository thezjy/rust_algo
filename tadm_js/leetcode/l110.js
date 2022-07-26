function isBalanced(root) {
  if (root == null) {
    return true
  }

  if (Math.abs(height(root.left) - height(root.right)) > 1) {
    return false
  }

  return isBalanced(root.left) && isBalanced(root.right)
}

function height(root) {
  if (root == null) {
    return 0
  }

  return 1 + Math.max(height(root.left), height(root.right))
}
