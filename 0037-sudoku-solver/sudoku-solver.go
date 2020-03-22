package solution

import (
	"fmt"
	"strings"
)

// i => xy, x, y := i % 9, i / 9
// xy => i, i := y * 9 + x
func solveSudoku(board [][]byte) {
	var b Board
	for i := 0; i < 81; i++ {
		c := board[i/9][i%9]
		if c == '.' {
			b[i] = 0
		} else {
			b[i] = c - '0'
		}
	}
	b.Solve(nil)
	for i := 0; i < 81; i++ {
		c := b[i]
		if c == 0 {
			board[i/9][i%9] = '.'
		} else {
			board[i/9][i%9] = c + '0'
		}
	}
}

//
//
//

type Board [81]byte

func (b *Board) LoadString(data string) {
	n := 0
	for _, c := range []byte(data) {
		if c >= '0' && c <= '9' {
			b[n] = c - '0'
			n++
		} else if c == '.' {
			n++

		}
	}
}

func (b *Board) String() string {
	sb := &strings.Builder{}
	for n, c := range b {
		if n > 0 {
			if n%9 == 0 {
				fmt.Fprintln(sb)
			} else if n%3 == 0 {
				fmt.Fprint(sb, " |")
			}
			if n%27 == 0 {
				fmt.Fprintln(sb, "-------+-------+-------")
			}
		}

		if c == 0 {
			fmt.Fprint(sb, " .")
		} else {
			fmt.Fprintf(sb, " %d", c)
		}
	}
	return sb.String()
}

func (b *Board) IsValid() bool {
	b[0] = 99
	var taken [3][9]int
	for n := 0; n < 81; n++ {
		if b[n] == 0 {
			continue
		}
		bit := 1 << (b[n] - 1)
		for i := 0; i < 3; i++ {
			j := RCB[n][i]
			if (taken[i][j] & bit) != 0 {
				return false
			}
			taken[i][j] |= bit
		}
	}
	return true
}

func (b *Board) PossibleNumbers(n int) []byte {
	taken := 0
	for _, i := range RELATIVES[n] {
		if b[i] > 0 {
			taken |= 1 << (b[i] - 1)
		}
	}
	var nums []byte
	for i := byte(0); i < 9; i++ {
		if (taken & (1 << i)) == 0 {
			nums = append(nums, i+1)
		}
	}
	return nums
}

func (b *Board) Solve(progress func()) bool {
	var findMinimalCell = func() (int, []byte) {
		mc := 255      // index of the minimal cell
		var mpn []byte // possible numbers of the minimal cell
		for i := 0; i < 81; i++ {
			if b[i] != 0 {
				continue
			}
			pn := b.PossibleNumbers(i)
			if len(pn) <= 0 {
				// there's no possible number for this cell
				return -1, nil
			}
			if len(pn) == 1 {
				// there's only one possible number, use it
				return i, pn
			}
			if mpn == nil || len(pn) < len(mpn) {
				mc = i
				mpn = pn
			}
		}
		return mc, mpn
	}

	var solveInternal func() bool
	solveInternal = func() bool {
		if progress != nil {
			progress()
		}
		n, pn := findMinimalCell()
		if pn == nil {
			return n == 255
		}
		for _, s := range pn {
			b[n] = s
			if solveInternal() == true {
				return true
			}
		}
		b[n] = 0
		return false
	}

	return solveInternal()
}

//
//
//

/*
 A board contains only 1-9 and 0 for empty cell.

 The following table shows the index of all cells.

 0>   0  1  2 |  3  4  5 |  6  7  8
 1>   9 10 11 | 12 13 14 | 15 16 17
 2>  18 19 20 | 21 22 23 | 24 25 26
    ----------+----------+----------
 3>  27 28 29 | 30 31 32 | 33 34 35
 4>  36 37 38 | 39 40 41 | 42 43 44
 5>  45 46 47 | 48 49 50 | 51 52 53
    ----------+----------+----------
 6>  54 55 56 | 57 58 59 | 60 61 62
 7>  63 64 65 | 66 67 68 | 69 70 71
 8>  72 73 74 | 75 76 77 | 78 79 80
*/

// the row (column, box) index of each cell
var RCB [81][3]int

// cell index of each row
var ROWS [9][9]int

// cell index of each column
var COLS [9][9]int

// cell index of each box
var BOXES [9][9]int

// the relative cells of each cell
var RELATIVES [81][]int

func init() {
	for i := 0; i < 81; i++ {
		RCB[i][0] = i / 9
		RCB[i][1] = i % 9
		RCB[i][2] = i/27*3 + (i/3)%3
	}

	for i := 0; i < 9; i++ {
		for j := 0; j < 9; j++ {
			ROWS[i][j] = i*9 + j
			COLS[i][j] = i + j*9
			BOXES[i][j] = j/3*9 + j%3 + i/3*27 + (i%3)*3
		}
	}

	for i := 0; i < 81; i++ {
		var m [81]bool
		r, c, b := RCB[i][0], RCB[i][1], RCB[i][2]
		for _, j := range ROWS[r] {
			m[j] = true
		}
		for _, j := range COLS[c] {
			m[j] = true
		}
		for _, j := range BOXES[b] {
			m[j] = true
		}
		for j := range m {
			if m[j] {
				RELATIVES[i] = append(RELATIVES[i], j)
			}
		}
	}
}
