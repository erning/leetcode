func judgePoint24(_ nums: [Int]) -> Bool {
    func judgePoint24(_ nums: [Double]) -> Bool {
        if nums.count == 1 {
            return abs(24 - nums[0]) <= 1e-9
        }
        var found = false
        for i in 0 ..< nums.count - 1 {
            for j in i + 1 ..< nums.count {
                let (a, b) = (nums[i], nums[j])
                let nums = nums[..<i] + nums[(i + 1) ..< j] + nums[(j + 1)...]
                found = found || judgePoint24(nums + [a + b])
                found = found || judgePoint24(nums + [a - b])
                found = found || judgePoint24(nums + [b - a])
                found = found || judgePoint24(nums + [a * b])
                found = found || (b != 0 && judgePoint24(nums + [a / b]))
                found = found || (a != 0 && judgePoint24(nums + [b / a]))
                if found {
                    return true
                }
            }
        }
        return false
    }
    return judgePoint24(nums.map { Double($0) })
}
