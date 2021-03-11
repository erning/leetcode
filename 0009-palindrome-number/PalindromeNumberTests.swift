import XCTest

@testable import Problem_0009
let isPalindrome = Solution().isPalindrome

class SolutionTests: XCTestCase {
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
