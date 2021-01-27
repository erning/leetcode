import XCTest

#if !canImport(ObjectiveC)
public func allTests() -> [XCTestCaseEntry] {
    [
        testCase(LeetCodeTests.allTests),
    ]
}
#endif
