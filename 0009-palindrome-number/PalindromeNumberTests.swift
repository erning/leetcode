import XCTest

@testable import LeetCode

extension LeetCodeTests {
    func testIsPalindrome() {
        XCTAssertTrue(isPalindrome(121))
        XCTAssertFalse(isPalindrome(-121))
        XCTAssertFalse(isPalindrome(10))
        XCTAssertFalse(isPalindrome(-101))

        XCTAssertTrue(isPalindrome(0))
        XCTAssertTrue(isPalindrome(12321))
        XCTAssertTrue(isPalindrome(1221))
    }
}
