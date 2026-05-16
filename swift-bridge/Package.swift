// swift-tools-version:5.9
import PackageDescription

let package = Package(
    name: "ImageIOBridge",
    platforms: [
        .macOS(.v13)
    ],
    products: [
        .library(
            name: "ImageIOBridge",
            type: .static,
            targets: ["ImageIOBridge"])
    ],
    targets: [
        .target(
            name: "ImageIOBridge",
            path: "Sources/ImageIOBridge",
            publicHeadersPath: "include")
    ]
)
