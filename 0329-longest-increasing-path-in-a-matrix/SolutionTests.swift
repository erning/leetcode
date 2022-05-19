import XCTest

@testable import Problem_0329
let longestIncreasingPath = Solution().longestIncreasingPath

class SolutionTests: XCTestCase {
    func testExample() {
        XCTAssertEqual(longestIncreasingPath([[9, 9, 4], [6, 6, 8], [2, 1, 1]]), 4)
        XCTAssertEqual(longestIncreasingPath([[3, 4, 5], [3, 2, 6], [2, 2, 1]]), 4)
        XCTAssertEqual(longestIncreasingPath([[1]]), 1)
    }
}
