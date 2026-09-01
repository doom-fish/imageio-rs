import CoreGraphics
import Foundation
import ImageIO

final class Box<T> {
    let value: T

    init(_ value: T) {
        self.value = value
    }
}

@inline(__always)
func retainBox<T>(_ value: T) -> UnsafeMutableRawPointer {
    Unmanaged.passRetained(Box(value)).toOpaque()
}

@inline(__always)
func unretainedBox<T>(_ raw: UnsafeRawPointer, as _: T.Type = T.self) -> Box<T> {
    Unmanaged<Box<T>>.fromOpaque(raw).takeUnretainedValue()
}

func writeCString(_ text: String, into buffer: UnsafeMutablePointer<CChar>?, capacity: Int) {
    guard let buffer, capacity > 0 else {
        return
    }
    text.withCString { source in
        strncpy(buffer, source, capacity - 1)
        buffer[capacity - 1] = 0
    }
}

func errorMessage(_ error: Error) -> String {
    (error as NSError).localizedDescription
}

/// `premultipliedFirst | byteOrder32Little` is the little-endian ARGB word that
/// lays out as B, G, R, A in memory — the packing Core Video calls `32BGRA`.
private let bgraBitmapInfo = CGBitmapInfo(rawValue: CGImageAlphaInfo.premultipliedFirst.rawValue)
    .union(.byteOrder32Little)

func decodeCGImageToBGRA(_ image: CGImage) -> Data? {
    let width = image.width
    let height = image.height
    let bytesPerRow = width * 4
    let colorSpace = CGColorSpaceCreateDeviceRGB()
    var data = Data(count: bytesPerRow * height)
    let drew = data.withUnsafeMutableBytes { bytes in
        guard let baseAddress = bytes.baseAddress else {
            return false
        }
        guard let context = CGContext(
            data: baseAddress,
            width: width,
            height: height,
            bitsPerComponent: 8,
            bytesPerRow: bytesPerRow,
            space: colorSpace,
            bitmapInfo: bgraBitmapInfo.rawValue
        ) else {
            return false
        }
        context.draw(
            image,
            in: CGRect(
                x: 0,
                y: 0,
                width: CGFloat(width),
                height: CGFloat(height)
            )
        )
        return true
    }
    return drew ? data : nil
}

func makeCGImage(
    fromBGRA bytes: UnsafePointer<UInt8>,
    length: Int,
    width: Int,
    height: Int
) -> CGImage? {
    let expectedLength = width * height * 4
    guard length >= expectedLength else {
        return nil
    }
    let data = Data(bytes: bytes, count: expectedLength)
    guard let provider = CGDataProvider(data: data as CFData) else {
        return nil
    }
    let colorSpace = CGColorSpaceCreateDeviceRGB()
    return CGImage(
        width: width,
        height: height,
        bitsPerComponent: 8,
        bitsPerPixel: 32,
        bytesPerRow: width * 4,
        space: colorSpace,
        bitmapInfo: bgraBitmapInfo,
        provider: provider,
        decode: nil,
        shouldInterpolate: true,
        intent: .defaultIntent
    )
}

@_cdecl("imageio_retain")
public func imageioRetain(_ raw: UnsafeMutableRawPointer?) -> UnsafeMutableRawPointer? {
    guard let raw else {
        return nil
    }
    let object = Unmanaged<AnyObject>.fromOpaque(raw).takeUnretainedValue()
    return Unmanaged.passRetained(object).toOpaque()
}

@_cdecl("imageio_release")
public func imageioRelease(_ raw: UnsafeMutableRawPointer?) {
    guard let raw else {
        return
    }
    Unmanaged<AnyObject>.fromOpaque(raw).release()
}

@_cdecl("imageio_string_len")
public func imageioStringLen(_ raw: UnsafeMutableRawPointer?) -> Int {
    guard let raw else {
        return 0
    }
    return unretainedBox(raw, as: String.self).value.lengthOfBytes(using: .utf8)
}

@_cdecl("imageio_string_copy_utf8")
public func imageioStringCopyUtf8(
    _ raw: UnsafeMutableRawPointer?,
    _ buffer: UnsafeMutablePointer<UInt8>?,
    _ capacity: Int
) -> Int {
    guard let raw else {
        return 0
    }
    let string = unretainedBox(raw, as: String.self).value
    let bytes = Array(string.utf8)
    if let buffer, capacity > 0 {
        let copied = min(bytes.count, capacity - 1)
        if copied > 0 {
            bytes.withUnsafeBufferPointer { source in
                buffer.initialize(from: source.baseAddress!, count: copied)
            }
        }
        buffer[copied] = 0
    }
    return bytes.count
}

@_cdecl("imageio_data_len")
public func imageioDataLen(_ raw: UnsafeMutableRawPointer?) -> Int {
    guard let raw else {
        return 0
    }
    return unretainedBox(raw, as: Data.self).value.count
}

@_cdecl("imageio_data_copy_bytes")
public func imageioDataCopyBytes(
    _ raw: UnsafeMutableRawPointer?,
    _ buffer: UnsafeMutablePointer<UInt8>?,
    _ capacity: Int
) -> Int {
    guard let raw else {
        return 0
    }
    let data = unretainedBox(raw, as: Data.self).value
    if let buffer, capacity > 0 {
        let copied = min(data.count, capacity)
        data.copyBytes(to: buffer, count: copied)
    }
    return data.count
}

@_cdecl("imageio_string_array_count")
public func imageioStringArrayCount(_ raw: UnsafeMutableRawPointer?) -> Int {
    guard let raw else {
        return 0
    }
    return unretainedBox(raw, as: [String].self).value.count
}

@_cdecl("imageio_string_array_copy_item")
public func imageioStringArrayCopyItem(
    _ raw: UnsafeMutableRawPointer?,
    _ index: Int
) -> UnsafeMutableRawPointer? {
    guard let raw else {
        return nil
    }
    let values = unretainedBox(raw, as: [String].self).value
    guard values.indices.contains(index) else {
        return nil
    }
    return retainBox(values[index])
}
