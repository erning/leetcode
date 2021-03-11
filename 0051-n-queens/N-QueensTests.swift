import XCTest

@testable import Problem_0051
let solveNQueens = Solution().solveNQueens

class SolutionTests: XCTestCase {
    func testSolveNQueens() {
        let rv = solveNQueens(8)
        XCTAssertEqual(rv.count, 92)
    }
}
