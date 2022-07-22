// https://leetcode.com/problems/lru-cache/

import { strict as assert } from 'node:assert'

/**
 * @param {number} capacity
 */
var LRUCache = function LRUCache(capacity) {
  this.capacity = capacity
  this.store = new Map()
  this.list = {
    head: null,
    tail: null,
  }
}

LRUCache.prototype.moveNodeToTail = function (node) {
  if (node.next != this.list.tail) {
    node.prev.next = node.next
    node.next.prev = node.prev
    this.list.tail.prev.next = node
    node.next = tail
  }
}

/**
 * @param {number} key
 * @return {number}
 */
LRUCache.prototype.get = function (key) {
  let node = this.store.get(key)

  if (node == null) {
    return -1
  } else {
    this.moveNodeToTail(node)

    return node.value
  }
}

/**
 * @param {number} key
 * @param {number} value
 * @return {void}
 */
LRUCache.prototype.put = function (key, value) {}

/**
 * Your LRUCache object will be instantiated and called as such:
 * var obj = new LRUCache(capacity)
 * var param_1 = obj.get(key)
 * obj.put(key,value)
 */

let cache = new LRUCache(3)

assert.equal(cache.get(1), -1)
cache.put(1, 1)
cache.put(2, 2)
assert.equal(cache.get(1), 1)
cache.put(3, 3)
cache.put(4, 4)
assert.equal(cache.get(2), 1)
assert.equal(cache.get(3), 3)
assert.equal(cache.get(4), 4)
