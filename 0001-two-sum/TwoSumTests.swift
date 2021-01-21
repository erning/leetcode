import XCTest

@testable import LeetCode

extension LeetCodeTests {
    func testTwoSum() {
        let num = [2, 7, 11, 15]
        let target = 9
        let expected = [0, 1]
        let output = solution.twoSum(num, target)

        print("expected: \(expected), output: \(output)")
        XCTAssertEqual(expected, output)
    }
}
