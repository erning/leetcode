extension Solution {
    func isPalindrome(_ x: Int) -> Bool {
        if x < 0 {
            return false
        }
        var n = x
        var d: [Int] = []
        while n > 0 {
            d.append(n % 10)
            n /= 10
        }
        for i in 0..<d.count/2 where d[i] != d[d.count-i-1] {
            return false
        }
        return true
    }
}
