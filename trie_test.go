package main

import "math"

type Trie struct {
	Val       [math.MaxUint8]*Trie
	Tombstone bool
}

func Constructor() Trie {
	return Trie{}
}

func (this *Trie) Insert(word string) {
	t := &this
	for _, i := range word {
		if *t == nil {
			*t = &Trie{}
		}
		t = &(*t).Val[i]
	}
	if *t == nil {
		*t = &Trie{}
	}
	(*t).Tombstone = true
}

func (this *Trie) Search(word string) bool {
	for i := range word {
		if this == nil {
			return false
		}
		this = this.Val[word[i]]
	}
	return this != nil && this.Tombstone
}

func (this *Trie) StartsWith(prefix string) bool {
	for i := range prefix {
		if this == nil {
			return false
		}
		this = this.Val[prefix[i]]
	}
	return this != nil
}

/**
 * Your Trie object will be instantiated and called as such:
 * obj := Constructor();
 * obj.Insert(word);
 * param_2 := obj.Search(word);
 * param_3 := obj.StartsWith(prefix);
 */
