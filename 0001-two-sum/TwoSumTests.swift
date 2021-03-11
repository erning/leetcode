import XCTest

@testable import Problem_0001
let twoSum = Solution().twoSum

class TwoSumTests: XCTestCase {
    func testTwoSum() {
        let num = [2, 7, 11, 15]
        let target = 9
        let expected = [0, 1]
        let output = twoSum(num, target)

        // print("expected: \(expected), output: \(output)")
        XCTAssertEqual(expected, output)
    }
}
