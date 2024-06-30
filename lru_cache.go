package main

import (
	"log/slog"
//	"math"
)

type linkedList[T any] struct {
	head *node[T]
	tail *node[T]
}

type node[T any] struct {
	value T
	next  *node[T]
	prev  *node[T]
}

func new_ll[T any]() linkedList[T] {
	return linkedList[T]{
		head: nil,
		tail: nil,
	}
}

func (this *linkedList[T]) insert(value T) {
	n := node[T]{value: value, next: nil, prev: nil}
	if this.head == nil {
		this.head = &n
		this.tail = &n
	} else {
		this.head.prev = &n
		n.next = this.head
		this.head = &n
	}
	if this.head == nil || this.tail == nil {
		panic("trying to remove node, even though linkedlist does not exist")
	}
}

func (this *linkedList[T]) list() []T {
	var res []T
	for temp := this.head; temp != nil; {
		res = append(res, temp.value)
		temp = temp.next
	}
	return res
}

func (this *linkedList[T]) first() *node[T] {
	return this.head
}

func (this *linkedList[T]) last() *node[T] {
	return this.tail
}

func (this *linkedList[T]) removeLast() *node[T] {
	temp := this.tail
	if this.tail == nil {
		return temp
	}
	this.tail = this.tail.prev
	if this.tail == nil {
		return temp
	}
	this.tail.next = nil
	return temp
}

func (this *linkedList[T]) removeNode(node *node[T]) {
	if this.head == nil {
		panic("trying to remove node, even though linkedlist does not exist")
	}
	prev := node.prev
	next := node.next
	if prev == nil {
		this.head = next
	} else {
		prev.next = next
	}
	if next == nil {
		this.tail = prev
	} else {
		next.prev = prev
	}
	node.next = nil
	node.prev = nil

}

type LRUCache struct {
	list     *linkedList[[2]int]
	hash     map[int]*node[[2]int]
	capacity uint
	current  uint
}

func Constructor(capacity int) LRUCache {
	ll := new_ll[[2]int]()
	return LRUCache{
		list:     &ll,
		hash:     make(map[int]*node[[2]int]),
		capacity: uint(capacity),
		current:  0,
	}
}

func (this *LRUCache) getNode(key int) *node[[2]int] {
	value, ok := this.hash[key]
	if !ok {
		return nil
	} else {
		this.list.removeNode(value)
		this.list.insert(value.value)
		this.hash[key] = this.list.head
		return this.list.head
	}
}

func (this *LRUCache) Get(key int) int {
	node := this.getNode(key)
	if node == nil {
		return -1
	}
	return node.value[1]
}

func (this *LRUCache) Put(key int, value int) {
	is_present := this.getNode(key)

	if is_present == nil {
		this.current += 1
	} else {
		this.list.removeNode(is_present)
	}

	this.list.insert([2]int{key, value})
	this.hash[key] = this.list.head

	for this.current > this.capacity {
		val := this.list.removeLast()
		delete(this.hash, val.value[0])
		this.current -= 1
	}

}

/**
 * Your LRUCache object will be instantiated and called as such:
 * obj := Constructor(capacity);
 * param_1 := obj.Get(key);
 * obj.Put(key,value);
 */

func printlruCache(cache LRUCache) {
	slog.Info("-------------START-----")
	defer slog.Info("-------END-------------")
	for k, v := range cache.hash {
		slog.Info("hash_map", "cache_key", k, "cache_value", v.value)
	}
	slog.Info("hash_map", "ll", cache.list.list())
}

// func main() {
// 	lRUCache := Constructor(2)
// 	lRUCache.Put(1, 1) // cache is {1=1}
// 	printlruCache(lRUCache)
// 	lRUCache.Put(2, 2) // cache is {1=1, 2=2}
// 	printlruCache(lRUCache)
// 	slog.Info("result", "value", lRUCache.Get(1)) // return 1
// 	printlruCache(lRUCache)
// 	lRUCache.Put(3, 3) // LRU key was 2, evicts key 2, cache is {1=1, 3=3}
// 	printlruCache(lRUCache)
// 	slog.Info("result", "value", lRUCache.Get(2)) // returns -1 (not found)
// 	printlruCache(lRUCache)
// 	lRUCache.Put(4, 4) // LRU key was 1, evicts key 1, cache is {4=4, 3=3}
// 	printlruCache(lRUCache)
// 	slog.Info("result", "value", lRUCache.Get(1)) // return -1 (not found)
// 	printlruCache(lRUCache)
// 	slog.Info("result", "value", lRUCache.Get(3)) // return 3
// 	printlruCache(lRUCache)
// 	slog.Info("result", "value", lRUCache.Get(4)) // return 4
// 	printlruCache(lRUCache)
// }

// func main() {
// 	lruCache := Constructor(2)
// 	lruCache.Put(2, 1)
// 	lruCache.Put(2, 2)
// 	lruCache.Get(2)
// 	lruCache.Put(1, 1)
// 	lruCache.Put(4, 1)
// 	lruCache.Get(2)
// }

// func main() {
// 	commands := []string{"LRUCache", "put", "put", "put", "put", "put", "get", "put", "get", "get", "put", "get", "put", "put", "put", "get", "put", "get", "get", "get", "get", "put", "put", "get", "get", "get", "put", "put", "get", "put", "get", "put", "get", "get", "get", "put", "put", "put", "get", "put", "get", "get", "put", "put", "get", "put", "put", "put", "put", "get", "put", "put", "get", "put", "put", "get", "put", "put", "put", "put", "put", "get", "put", "put", "get", "put", "get", "get", "get", "put", "get", "get", "put", "put", "put", "put", "get", "put", "put", "put", "put", "get", "get", "get", "put", "put", "put", "get", "put", "put", "put", "get", "put", "put", "put", "get", "get", "get", "put", "put", "put", "put", "get", "put", "put", "put", "put", "put", "put", "put"}
// 	values := [][]int{{10}, {10, 13}, {3, 17}, {6, 11}, {10, 5}, {9, 10}, {13}, {2, 19}, {2}, {3}, {5, 25}, {8}, {9, 22}, {5, 5}, {1, 30}, {11}, {9, 12}, {7}, {5}, {8}, {9}, {4, 30}, {9, 3}, {9}, {10}, {10}, {6, 14}, {3, 1}, {3}, {10, 11}, {8}, {2, 14}, {1}, {5}, {4}, {11, 4}, {12, 24}, {5, 18}, {13}, {7, 23}, {8}, {12}, {3, 27}, {2, 12}, {5}, {2, 9}, {13, 4}, {8, 18}, {1, 7}, {6}, {9, 29}, {8, 21}, {5}, {6, 30}, {1, 12}, {10}, {4, 15}, {7, 22}, {11, 26}, {8, 17}, {9, 29}, {5}, {3, 4}, {11, 30}, {12}, {4, 29}, {3}, {9}, {6}, {3, 4}, {1}, {10}, {3, 29}, {10, 28}, {1, 20}, {11, 13}, {3}, {3, 12}, {3, 8}, {10, 9}, {3, 26}, {8}, {7}, {5}, {13, 17}, {2, 27}, {11, 15}, {12}, {9, 19}, {2, 15}, {3, 16}, {1}, {12, 17}, {9, 1}, {6, 19}, {4}, {5}, {5}, {8, 1}, {11, 7}, {5, 2}, {9, 28}, {1}, {2, 2}, {7, 4}, {4, 22}, {7, 24}, {9, 26}, {13, 28}, {11, 26}}
// 	null := math.MaxInt
// 	result := []int{null, null, null, null, null, null, -1, null, 19, 17, null, -1, null, null, null, -1, null, -1, 5, -1, 12, null, null, 3, 5, 5, null, null, 1, null, -1, null, 30, 5, 30, null, null, null, -1, null, -1, 24, null, null, 18, null, null, null, null, -1, null, null, 18, null, null, -1, null, null, null, null, null, 18, null, null, -1, null, 4, 29, 30, null, 12, -1, null, null, null, null, 29, null, null, null, null, 17, 22, 18, null, null, null, -1, null, null, null, 20, null, null, null, -1, 18, 18, null, null, null, null, 20, null, null, null, null, null, null, null}
// 	var data *LRUCache
// 	for k, v := range commands {
// 		switch v {
// 		case "LRUCache":
// 			val := Constructor(values[k][0])
// 			data = &val
// 			slog.Info("main", "created value", data)
// 			break
// 		case "put":
// 			data.Put(values[k][0], values[k][1])
// 			slog.Info("main", "putting key", values[k][0], "value", values[k][1])
// 			if values[k][0] == 6 && values[k][1] == 11 {
// 				print()
// 			}
// 			break
// 		case "get":
// 			res := data.Get(values[k][0])
// 			slog.Info("main", "getting key", values[k][0], "result", res)
// 			if result[k] != res {
// 				slog.Error("main", "getting key", values[k][0], "expected", result[k])
// 				panic("assertin failed")
// 			}
// 			break
// 		}
// 		printlruCache(*data)
// 	}
// }
