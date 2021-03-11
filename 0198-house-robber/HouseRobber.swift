class Solution {
    func rob(_ nums: [Int]) -> Int {
        if nums.count == 0 {
            return 0
        }
        if nums.count == 1 {
            return nums[0]
        }
        if nums.count == 2 {
            return max(nums[0], nums[1])
        }

        var a = nums[0]
        var b = max(nums[0], nums[1])
        for i in 2 ..< nums.count {
            (a, b) = (b, max(nums[i] + a, b))
        }
        return b
    }
}
