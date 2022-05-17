class Solution {
    func rob(_ nums: [Int]) -> Int {
        if nums.count <= 2 {
            return nums.max() ?? 0
        }

        var a = nums[0]
        var b = max(nums[0], nums[1])
        for c in nums[2...] {
            (a, b) = (b, max(a + c, b))
        }
        return b
    }

    func rob1(_ nums: [Int]) -> Int {
        nums.reduce((0, 0)) { ($0.1, max($0.0 + $1, $0.1)) }.1
    }
}
