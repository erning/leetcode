// swift-tools-version:5.3
// The swift-tools-version declares the minimum version of Swift required to build this package.

import Foundation
import PackageDescription

let (exclude, sources, tests) = { () -> ([String], [String], [String]) in
    let packageDir: String = {
        let file = URL(fileURLWithPath: #file)
        let dir = file.deletingLastPathComponent()
        return dir.path
    }()

    let ignores: Set<String> = [
        "Package.swift",
        "LeetCodeTests.swift",
        "XCTestManifests.swift",
        "LinuxMain.swift"
    ]

    var exclude: [String] = [
        "Package.swift",
        "LinuxMain.swift"
    ]
    var sources: [String] = []
    var tests: [String] = [
        "LeetCodeTests.swift",
        "XCTestManifests.swift"
    ]

    guard
        let enumerator = FileManager.default.enumerator(
            at: URL(fileURLWithPath: packageDir),
            includingPropertiesForKeys: [.isDirectoryKey],
            options: .skipsHiddenFiles
        )
    else {
        return (exclude, sources, tests)
    }

    for case let fileURL as URL in enumerator {
        guard
            let resourceValues = try? fileURL.resourceValues(forKeys: [.isDirectoryKey]),
            let isDirectory = resourceValues.isDirectory
        else {
            continue
        }

        if isDirectory {
            continue
        }

        let filename: String = {
            var file = fileURL.path
            file = String(file.suffix(from: packageDir.endIndex))
            file = String(file.suffix(from: "/".endIndex))
            return file
        }()

        if ignores.contains(filename) {
            continue
        }

        if filename.hasSuffix("Tests.swift") {
            tests.append(filename)
        } else if filename.hasSuffix(".swift") {
            sources.append(filename)
        } else {
            exclude.append(filename)
        }
    }

    return (exclude, sources, tests)
}()

let package = Package(
    name: "LeetCode",
    products: [
        // Products define the executables and libraries a package produces, and make them visible to other packages.
        .library(
            name: "LeetCode",
            targets: ["LeetCode"])
    ],
    dependencies: [
        // Dependencies declare other packages that this package depends on.
        // .package(url: /* package url */, from: "1.0.0"),
    ],
    targets: [
        // Targets are the basic building blocks of a package. A target can define a module or a test suite.
        // Targets can depend on other targets in this package, and on products in packages this package depends on.
        .target(
            name: "LeetCode",
            dependencies: [],
            path: "",
            exclude: exclude + tests,
            sources: sources
        ),
        .testTarget(
            name: "LeetCodeTests",
            dependencies: ["LeetCode"],
            path: "",
            exclude: exclude + sources,
            sources: tests
        ),
    ]
)
