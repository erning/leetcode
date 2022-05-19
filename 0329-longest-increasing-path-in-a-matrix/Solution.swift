class Solution {
    func longestIncreasingPath(_ matrix: [[Int]]) -> Int {
        let m = matrix.count
        let n = matrix[0].count
        var dp = [[Int]](repeating: [Int](repeating: 0, count: n), count: m)
        var answer = 0

        func dfs(x: Int, y: Int) {
            if dp[y][x] != 0 {
                return
            }
            var v = 1
            for (dx, dy) in [(0, -1), (1, 0), (0, 1), (-1, 0)] {
                let (nx, ny) = (x + dx, y + dy)
                if nx < 0 || nx >= n || ny < 0 || ny >= m {
                    continue
                }
                if matrix[ny][nx] <= matrix[y][x] {
                    continue
                }
                dfs(x: nx, y: ny)
                v = max(v, dp[ny][nx] + 1)
            }
            dp[y][x] = v
            if answer < v {
                answer = v
            }
        }

        for y in 0 ..< m {
            for x in 0 ..< n {
                dfs(x: x, y: y)
            }
        }

        return answer
    }
}
