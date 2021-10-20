import XCTest

@testable import Problem_0746
let minCostClimbingStairs = Solution().minCostClimbingStairs

class MinCostClimbingStairsTests: XCTestCase {
    func testMinCostClimbingStairs() {
        XCTAssertEqual(minCostClimbingStairs([10, 15, 20]), 15)
        XCTAssertEqual(minCostClimbingStairs([1, 100, 1, 1, 1, 100, 1, 1, 100, 1]), 6)
    }
}
