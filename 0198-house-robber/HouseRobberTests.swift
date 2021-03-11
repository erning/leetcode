import XCTest

@testable import Problem_0198
let rob = Solution().rob

class HouseRobberTests: XCTestCase {
    func testRob() {
        XCTAssertEqual(rob([]), 0)
        XCTAssertEqual(rob([1]), 1)
        XCTAssertEqual(rob([1, 2]), 2)

        XCTAssertEqual(rob([1, 2, 3, 1]), 4)
        XCTAssertEqual(rob([2, 7, 9, 3, 1]), 12)

        XCTAssertEqual(rob([2, 1, 1, 2]), 4)

        XCTAssertEqual(
            rob([
                5, 7, 1, 2, 7, 6, 3, 1, 3, 8,
                2, 2, 1, 5, 4, 1, 6, 9, 1, 4,
                5, 2, 7, 7, 3, 3, 7, 3, 4, 1,
                4, 5, 2, 9, 6, 6, 1, 4, 7, 5,
                9, 1, 1, 3, 2, 9, 2, 7, 4, 7,
            ]),
            130
        )
    }
}
