import XCTest

@testable import LeetCode

class LeetCodeTests: XCTestCase {
    func testExample() {
        // This is an example of a functional test case.
        // Use XCTAssert and related functions to verify your tests produce the correct
        // results.
        XCTAssertTrue(true)
    }

    static var allTests = [
        ("testExample", testExample),
        ("testTwoSum", testTwoSum),
        ("testSolveNQueens", testSolveNQueens),
        ("test24Game", test24Game),
    ]
}

let solution = Solution()
