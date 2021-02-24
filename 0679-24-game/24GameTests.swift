import XCTest

@testable import LeetCode

extension LeetCodeTests {
    func testJudgePoint24() {
        XCTAssertTrue(solution.judgePoint24([4, 1, 8, 7]))
        XCTAssertTrue(solution.judgePoint24([1, 5, 5, 5]))
        XCTAssertTrue(solution.judgePoint24([7, 7, 3, 3]))
        XCTAssertFalse(solution.judgePoint24([1, 1, 1, 7]))
    }
}
