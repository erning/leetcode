import XCTest

@testable import LeetCode

extension LeetCodeTests {
    func testSolveNQueens() {
        let rv = solveNQueens(8)
        XCTAssertEqual(rv.count, 92)
    }
}
