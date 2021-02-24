import XCTest

@testable import LeetCode

extension LeetCodeTests {
    func testJudgePoint24() {
        XCTAssertTrue(judgePoint24([4, 1, 8, 7]))
        XCTAssertTrue(judgePoint24([1, 5, 5, 5]))
        XCTAssertTrue(judgePoint24([7, 7, 3, 3]))
        XCTAssertFalse(judgePoint24([1, 1, 1, 7]))
    }
}
