class Solution {
    func minCostClimbingStairsOrigin(_ cost: [Int]) -> Int {
        var dp: [Int] = Array(repeating: 0, count: cost.count)
        dp[0] = cost[0]
        dp[1] = cost[1]
        for i in 2 ..< dp.count {
            dp[i] = min(dp[i - 2] + cost[i], dp[i - 1] + cost[i])
        }
        return min(dp[dp.count - 2], dp[dp.count - 1])
    }

    func minCostClimbingStairs(_ cost: [Int]) -> Int {
        var dp0 = cost[0]
        var dp1 = cost[1]
        for i in 2 ..< cost.count {
            let dp = min(dp0, dp1) + cost[i]
            dp0 = dp1
            dp1 = dp
        }
        return min(dp0, dp1)
    }
}
