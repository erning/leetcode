extension Solution {
    func solveNQueens(_ n: Int) -> [[String]] {
        var m1 = [Bool](repeating: false, count: n)  // row
        var m2 = [Bool](repeating: false, count: n)  // column
        var m3 = [Bool](repeating: false, count: n * 2)  // slash
        var m4 = [Bool](repeating: false, count: n * 2)  // backslash
        var board = [[Bool]](repeating: [Bool](repeating: false, count: n), count: n)

        var rv: [[String]] = []
        func solve(_ y: Int) {
            if y >= n {
                var ss = [String]()
                for i in 0..<n {
                    var cs = [Character](repeating: ".", count: n)
                    for j in 0..<n {
                        if board[i][j] {
                            cs[j] = "Q"
                        }
                    }
                    ss.append(String(cs))
                }
                rv.append(ss)
                return
            }
            for x in 0..<n {
                if m1[x] || m2[y] || m3[x + y] || m4[n + x - y] {
                    continue
                }
                board[x][y] = true
                (m1[x], m2[y], m3[x + y], m4[n + x - y]) = (true, true, true, true)
                solve(y + 1)
                (m1[x], m2[y], m3[x + y], m4[n + x - y]) = (false, false, false, false)
                board[x][y] = false
            }
        }
        solve(0)
        return rv
    }
}
