package wordsearch2

import (
	"fmt"
	// "log/slog"
	"strings"
	"testing"
)

type Trie struct {
	Val map[byte]*Trie
	End bool
}

func (self *Trie) String() string {
	if self == nil {
		return ""
	}
	var res []string
	for k, v := range self.Val {
		res = append(res, fmt.Sprintf("%c%s", k, v.String()))
	}
	return strings.Join(res, ",")
}

func Constructor() Trie {
	return Trie{Val: make(map[byte]*Trie, 0)}
}

func (this *Trie) insert(word string) {
	if len(word) == 0 {
		this.End = true
		return
	}
	c := word[0]
	rest := word[1:]
	res, ok := this.Val[c]
	if !ok {
		new_trie := Constructor()
		this.Val[c] = &new_trie
		res = &new_trie
	}
	res.insert(rest)
}

func (this *Trie) remove(word string) {
	if len(word) == 0 {
		this.End = false
		return
	}
	c := word[0]
	rest := word[1:]
	res, ok := this.Val[c]
	if ok {
		res.remove(rest)
		if len(res.Val) == 0 && res.End == false {
			delete(this.Val, c)
		}
	}
}

func findWords(board [][]byte, words []string) []string {
	result := make(map[string]struct{}, 0)
	delta := [][2]int{{0, 1}, {1, 0}, {-1, 0}, {0, -1}}

	visited := make(map[[2]int]struct{}, 0)
	var dfs func([2]int, *Trie, string)
	dfs = func(pos [2]int, t *Trie, res string) {
		if t == nil {
			return
		}
		if t.End {
			// slog.Info("DFS found the word", "result", res)
			result[res] = struct{}{}
			t.remove(res)
		}
		// check pos in bound?
		x := pos[0]
		y := pos[1]
		if x < 0 || x >= len(board) {
			// slog.Error("DFS not in board", "pos", pos)
			return
		}
		if y < 0 || y >= len(board[0]) {
			// slog.Error("DFS not in board", "pos", pos)
			return
		}
		// check if already visited
		_, ok := visited[pos]
		if ok {
			// slog.Error("DFS already visited", "pos", pos)
			return
		}
		// check if we have the value or not
		new_t, ok := t.Val[board[x][y]]
		if !ok {
			return
		}

		// slog.Info("DFS", "pos", pos, "res", res, "trie", t, "visited", visited)

		n := fmt.Sprintf("%s%c", res, board[x][y])
		// slog.Info("DFS", "old", res, "c", fmt.Sprintf("%c", board[x][y]), "new", n, "trie", new_t)

		// traverse neihbor
		visited[pos] = struct{}{}
		for _, d := range delta {
			dx := d[0]
			dy := d[1]
			newPos := [2]int{x + dx, y + dy}
			// slog.Info("DFS", "src", pos, "dest", newPos)
			dfs(newPos, new_t, n)
		}
		delete(visited, pos)
	}

	_trie := Constructor()
	trie := &_trie
	for i := range words {
		trie.insert(words[i])
	}

	// slog.Info("main", "trie", trie)

	for r := range board {
		for c := range board[r] {
			visited = make(map[[2]int]struct{}, 0)
			dfs([2]int{r, c}, trie, "")
		}
	}
	res := make([]string, 0)
	for k := range result {
		res = append(res, k)
	}
	return res
}

func TestSomenthing(t *testing.T) {
	board := [][]byte{{'o', 'a', 'a', 'n'}, {'e', 't', 'a', 'e'}, {'i', 'h', 'k', 'r'}, {'i', 'f', 'l', 'v'}}
	words := []string{"oath", "pea", "eat", "rain"}
	res := findWords(board, words)
	t.Fatalf("result was different: %v", res)
}

func TestSomething2(t *testing.T) {
	board := [][]byte{{'o', 'a', 'a', 'n'}, {'e', 't', 'a', 'e'}, {'i', 'h', 'k', 'r'}, {'i', 'f', 'l', 'v'}}
	words := []string{"oath", "pea", "eat", "rain", "hklf", "hf"}
	res := findWords(board, words)
	t.Fatalf("result was different: %v", res)
}

func TestSomething3(t *testing.T) {
	board := [][]byte{{'a'}}
	words := []string{"a"}
	res := findWords(board, words)
	t.Fatalf("result was different: %v", res)
}

func TestSomething4(t *testing.T) {
	board := [][]byte{{'a', 'a', 'a', 'a', 'a', 'a', 'a', 'a', 'a', 'a', 'a', 'a'}, {'a', 'a', 'a', 'a', 'a', 'a', 'a', 'a', 'a', 'a', 'a', 'a'}, {'a', 'a', 'a', 'a', 'a', 'a', 'a', 'a', 'a', 'a', 'a', 'a'}, {'a', 'a', 'a', 'a', 'a', 'a', 'a', 'a', 'a', 'a', 'a', 'a'}, {'a', 'a', 'a', 'a', 'a', 'a', 'a', 'a', 'a', 'a', 'a', 'a'}, {'a', 'a', 'a', 'a', 'a', 'a', 'a', 'a', 'a', 'a', 'a', 'a'}, {'a', 'a', 'a', 'a', 'a', 'a', 'a', 'a', 'a', 'a', 'a', 'a'}, {'a', 'a', 'a', 'a', 'a', 'a', 'a', 'a', 'a', 'a', 'a', 'a'}, {'a', 'a', 'a', 'a', 'a', 'a', 'a', 'a', 'a', 'a', 'a', 'a'}, {'a', 'a', 'a', 'a', 'a', 'a', 'a', 'a', 'a', 'a', 'a', 'a'}, {'a', 'a', 'a', 'a', 'a', 'a', 'a', 'a', 'a', 'a', 'a', 'a'}, {'a', 'a', 'a', 'a', 'a', 'a', 'a', 'a', 'a', 'a', 'a', 'a'}}
	words := []string{"lllllll", "fffffff", "ssss", "s", "rr", "xxxx", "ttt", "eee", "ppppppp", "iiiiiiiii", "xxxxxxxxxx", "pppppp", "xxxxxx", "yy", "jj", "ccc", "zzz", "ffffffff", "r", "mmmmmmmmm", "tttttttt", "mm", "ttttt", "qqqqqqqqqq", "z", "aaaaaaaa", "nnnnnnnnn", "v", "g", "ddddddd", "eeeeeeeee", "aaaaaaa", "ee", "n", "kkkkkkkkk", "ff", "qq", "vvvvv", "kkkk", "e", "nnn", "ooo", "kkkkk", "o", "ooooooo", "jjj", "lll", "ssssssss", "mmmm", "qqqqq", "gggggg", "rrrrrrrrrr", "iiii", "bbbbbbbbb", "aaaaaa", "hhhh", "qqq", "zzzzzzzzz", "xxxxxxxxx", "ww", "iiiiiii", "pp", "vvvvvvvvvv", "eeeee", "nnnnnnn", "nnnnnn", "nn", "nnnnnnnn", "wwwwwwww", "vvvvvvvv", "fffffffff", "aaa", "p", "ddd", "ppppppppp", "fffff", "aaaaaaaaa", "oooooooo", "jjjj", "xxx", "zz", "hhhhh", "uuuuu", "f", "ddddddddd", "zzzzzz", "cccccc", "kkkkkk", "bbbbbbbb", "hhhhhhhhhh", "uuuuuuu", "cccccccccc", "jjjjj", "gg", "ppp", "ccccccccc", "rrrrrr", "c", "cccccccc", "yyyyy", "uuuu", "jjjjjjjj", "bb", "hhh", "l", "u", "yyyyyy", "vvv", "mmm", "ffffff", "eeeeeee", "qqqqqqq", "zzzzzzzzzz", "ggg", "zzzzzzz", "dddddddddd", "jjjjjjj", "bbbbb", "ttttttt", "dddddddd", "wwwwwww", "vvvvvv", "iii", "ttttttttt", "ggggggg", "xx", "oooooo", "cc", "rrrr", "qqqq", "sssssss", "oooo", "lllllllll", "ii", "tttttttttt", "uuuuuu", "kkkkkkkk", "wwwwwwwwww", "pppppppppp", "uuuuuuuu", "yyyyyyy", "cccc", "ggggg", "ddddd", "llllllllll", "tttt", "pppppppp", "rrrrrrr", "nnnn", "x", "yyy", "iiiiiiiiii", "iiiiii", "llll", "nnnnnnnnnn", "aaaaaaaaaa", "eeeeeeeeee", "m", "uuu", "rrrrrrrr", "h", "b", "vvvvvvv", "ll", "vv", "mmmmmmm", "zzzzz", "uu", "ccccccc", "xxxxxxx", "ss", "eeeeeeee", "llllllll", "eeee", "y", "ppppp", "qqqqqq", "mmmmmm", "gggg", "yyyyyyyyy", "jjjjjj", "rrrrr", "a", "bbbb", "ssssss", "sss", "ooooo", "ffffffffff", "kkk", "xxxxxxxx", "wwwwwwwww", "w", "iiiiiiii", "ffff", "dddddd", "bbbbbb", "uuuuuuuuu", "kkkkkkk", "gggggggggg", "qqqqqqqq", "vvvvvvvvv", "bbbbbbbbbb", "nnnnn", "tt", "wwww", "iiiii", "hhhhhhh", "zzzzzzzz", "ssssssssss", "j", "fff", "bbbbbbb", "aaaa", "mmmmmmmmmm", "jjjjjjjjjj", "sssss", "yyyyyyyy", "hh", "q", "rrrrrrrrr", "mmmmmmmm", "wwwww", "www", "rrr", "lllll", "uuuuuuuuuu", "oo", "jjjjjjjjj", "dddd", "pppp", "hhhhhhhhh", "kk", "gggggggg", "xxxxx", "vvvv", "d", "qqqqqqqqq", "dd", "ggggggggg", "t", "yyyy", "bbb", "yyyyyyyyyy", "tttttt", "ccccc", "aa", "eeeeee", "llllll", "kkkkkkkkkk", "sssssssss", "i", "hhhhhh", "oooooooooo", "wwwwww", "ooooooooo", "zzzz", "k", "hhhhhhhh", "aaaaa", "mmmmm"}

	res := findWords(board, words)
	t.Fatalf("result was different: %v", res)
}
