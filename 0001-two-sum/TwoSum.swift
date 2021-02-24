    func twoSum(_ nums: [Int], _ target: Int) -> [Int] {
        var m = [Int: Int]()
        for (i, v) in nums.enumerated() {
            let complement = target - v
            if let j = m[complement] {
                return [j, i]
            }
            m[v] = i
        }
        return []
    }
