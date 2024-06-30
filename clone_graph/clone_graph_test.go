package main

import "testing"

func TestCloneGraph(t *testing.T) {
	a := Node{Val: 1}
	b := Node{Val: 2}
	c := Node{Val: 3}
	d := Node{Val: 4}
  a.Neighbors = append(a.Neighbors, &b)
  a.Neighbors = append(a.Neighbors, &d)
  b.Neighbors = append(b.Neighbors, &a)
  b.Neighbors = append(b.Neighbors, &c)
  c.Neighbors = append(c.Neighbors, &b)
  c.Neighbors = append(c.Neighbors, &d)
  d.Neighbors = append(d.Neighbors, &a)
  d.Neighbors = append(d.Neighbors, &c)
  _ = cloneGraph(&a)
  t.Fail()
}

