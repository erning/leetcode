// swift-tools-version:5.3
// The swift-tools-version declares the minimum version of Swift required to build this package.

import Foundation
import PackageDescription

let packageDir: String = {
    let file = URL(fileURLWithPath: #file)
    let dir = file.deletingLastPathComponent()
    return dir.path
}()

let problems = findProblems()

let package = Package(
    name: "LeetCode",
    targets: findTargets(problems: problems)
)

//
//
//

func findProblems() -> [String] {
    guard
        let contents = try? FileManager.default.contentsOfDirectory(
            at: URL(fileURLWithPath: packageDir),
            includingPropertiesForKeys: [.isDirectoryKey],
            options: .skipsHiddenFiles
        )
    else {
        return []
    }

    return contents.filter { content in
        guard
            let resourceValues = try? content.resourceValues(forKeys: [.isDirectoryKey]),
            let isDirectory = resourceValues.isDirectory
        else {
            return false
        }
        return isDirectory
    }.map { (content) -> String in
        content.lastPathComponent
    }.filter {
        let range = NSRange(location: 0, length: $0.utf16.count)
        let regex = try! NSRegularExpression(pattern: "^[0-9]{4}-")
        return regex.firstMatch(in: $0, options: [], range: range) != nil
    }.sorted()
}

func findTargets(problems: [String]) -> [PackageDescription.Target] {
    problems.reduce(into: []) { targets, problem in
        targets += findTargets(problem: problem)
    }
}

func findTargets(problem: String) -> [PackageDescription.Target] {
    var targets: [PackageDescription.Target] = []

    let problemDir = packageDir + "/" + problem
    guard
        let enumerator = FileManager.default.enumerator(
            at: URL(fileURLWithPath: problemDir),
            includingPropertiesForKeys: [.isDirectoryKey],
            options: .skipsHiddenFiles
        )
    else {
        return targets
    }

    var sources: [String] = []
    var exclude: [String] = []
    var tests: [String] = []

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
            file = String(file.suffix(from: problemDir.endIndex))
            file = String(file.suffix(from: "/".endIndex))
            return file
        }()

        if filename.hasSuffix("Tests.swift") {
            tests.append(filename)
        } else if filename.hasSuffix(".swift") {
            sources.append(filename)
        } else {
            exclude.append(filename)
        }
    }

    let problemName = "Problem-" + problem.prefix(4)
    if sources.isEmpty {
        return targets
    }
    targets.append(
        .target(
            name: problemName,
            dependencies: [],
            path: problem,
            exclude: exclude + tests,
            sources: sources
        )
    )
    if tests.isEmpty {
        return targets
    }
    targets.append(
        .testTarget(
            name: problemName + "-Tests",
            dependencies: [.byName(name: problemName)],
            path: problem,
            exclude: exclude + sources,
            sources: tests
        )
    )
    return targets
}
