// swift-tools-version:5.3

import Foundation
import PackageDescription

let packageDir: String = {
    let file = URL(fileURLWithPath: #file)
    let dir = file.deletingLastPathComponent()
    return dir.path
}()

let groups = findProblemGroups()

var problems: [(String, String)] = []
for group in groups {
    let p = findProblems(group: group)
    problems.append(contentsOf: p)
}

let p = findProblems(group: "")
problems.append(contentsOf: p)

let package = Package(
    name: "LeetCode",
    targets: findTargets(problems: problems)
)

//
//
//
func findProblemGroups() -> [String] {
    guard
        let contents = try? FileManager.default.contentsOfDirectory(
            at: URL(fileURLWithPath: packageDir),
            includingPropertiesForKeys: [.isDirectoryKey],
            options: .skipsHiddenFiles
        )
    else {
        return []
    }

    return contents.filter {
        guard
            let resourceValues = try? $0.resourceValues(forKeys: [.isDirectoryKey]),
            let isDirectory = resourceValues.isDirectory
        else {
            return false
        }
        return isDirectory
    }.map(\.lastPathComponent).filter {
        let range = NSRange(location: 0, length: $0.utf16.count)
        let regex = try? NSRegularExpression(pattern: "^[0-9]{4}-[0-9]{4}$")
        return regex?.firstMatch(in: $0, options: [], range: range) != nil
    }.sorted()
}

func findProblems(group: String) -> [(String, String)] {
    guard
        let contents = try? FileManager.default.contentsOfDirectory(
            at: URL(fileURLWithPath: "\(packageDir)/\(group)"),
            includingPropertiesForKeys: [.isDirectoryKey],
            options: .skipsHiddenFiles
        )
    else {
        return []
    }

    return contents.filter {
        guard
            let resourceValues = try? $0.resourceValues(forKeys: [.isDirectoryKey]),
            let isDirectory = resourceValues.isDirectory
        else {
            return false
        }
        return isDirectory
    }.map(\.lastPathComponent).filter {
        let range = NSRange(location: 0, length: $0.utf16.count)
        let regex = try? NSRegularExpression(pattern: "^[0-9]{4}-[0-9]{4}$")
        return regex?.firstMatch(in: $0, options: [], range: range) == nil
    }
    .filter {
        let range = NSRange(location: 0, length: $0.utf16.count)
        let regex = try? NSRegularExpression(pattern: "^[0-9]{4}-")
        return regex?.firstMatch(in: $0, options: [], range: range) != nil
    }.sorted().map {
        (group, $0)
    }
}

func findTargets(problems: [(String, String)]) -> [PackageDescription.Target] {
    problems.reduce(into: []) { targets, problem in
        targets += findTargets(problem: problem)
    }
}

func findTargets(problem: (String, String)) -> [PackageDescription.Target] {
    var targets: [PackageDescription.Target] = []
    let files = findFiles(group: problem.0, problem: problem.1)

    let problemName = "Problem-" + problem.1.prefix(4)
    if files.sources.isEmpty {
        return targets
    }
    let path = problem.0.isEmpty ? problem.1 : "\(problem.0)/\(problem.1)"

    targets.append(
        .target(
            name: problemName,
            path: path,
            exclude: files.exclude + files.tests,
            sources: files.sources
        )
    )
    if files.tests.isEmpty {
        return targets
    }
    targets.append(
        .testTarget(
            name: problemName + "-Tests",
            dependencies: [.byName(name: problemName)],
            path: path,
            exclude: files.exclude + files.sources,
            sources: files.tests
        )
    )
    return targets
}

typealias ProblemFiles = (sources: [String], exclude: [String], tests: [String])

func findFiles(group: String, problem: String) -> ProblemFiles {
    var files: ProblemFiles = (
        sources: [],
        exclude: [],
        tests: []
    )

    let problemDir: String
    if group.isEmpty {
        problemDir = packageDir + "/" + problem
    } else {
        problemDir = packageDir + "/" + group + "/" + problem
    }
    guard
        let enumerator = FileManager.default.enumerator(
            at: URL(fileURLWithPath: problemDir),
            includingPropertiesForKeys: [.isDirectoryKey],
            options: .skipsHiddenFiles
        )
    else {
        return files
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

        let filename = fileURL.path.suffix(
            from: .init(utf16Offset: problemDir.utf16.count + 1, in: fileURL.path)
        )

        if filename.hasSuffix("Tests.swift") {
            files.tests.append(String(filename))
        } else if filename.hasSuffix(".swift") {
            files.sources.append(String(filename))
        } else {
            files.exclude.append(String(filename))
        }
    }

    return files
}
