import XCTest

@testable import LeetCode

extension LeetCodeTests {
    func testIsPalindrome() {
        XCTAssertTrue(solution.isPalindrome(121))
        XCTAssertFalse(solution.isPalindrome(-121))
        XCTAssertFalse(solution.isPalindrome(10))
        XCTAssertFalse(solution.isPalindrome(-101))

        XCTAssertTrue(solution.isPalindrome(0))
        XCTAssertTrue(solution.isPalindrome(12321))
        XCTAssertTrue(solution.isPalindrome(1221))
    }
}
