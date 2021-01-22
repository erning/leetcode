import XCTest

@testable import LeetCode

extension LeetCodeTests {
    func testSolveNQueens() {
        let rv = solution.solveNQueens(8)
        XCTAssertEqual(rv.count, 92)
    }
}
