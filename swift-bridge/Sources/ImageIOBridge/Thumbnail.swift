import Foundation
import ImageIO

@_cdecl("imageio_source_create_thumbnail_bgra_at_index")
public func imageioSourceCreateThumbnailBgraAtIndex(
    _ raw: UnsafeMutableRawPointer?,
    _ index: Int,
    _ maxPixelSize: Int,
    _ alwaysCreate: Bool,
    _ transform: Bool,
    _ maxWidth: Int,
    _ maxHeight: Int,
    _ maxBytes: Int,
    _ widthOut: UnsafeMutablePointer<Int>?,
    _ heightOut: UnsafeMutablePointer<Int>?,
    _ limitExceeded: UnsafeMutablePointer<Bool>?,
    _ errorBuffer: UnsafeMutablePointer<CChar>?,
    _ errorBufferSize: Int
) -> UnsafeMutableRawPointer? {
    guard let raw, maxPixelSize > 0 else {
        writeCString("invalid image source or thumbnail size", into: errorBuffer, capacity: errorBufferSize)
        return nil
    }
    let source = unretainedBox(raw, as: CGImageSource.self).value
    let options = NSMutableDictionary()
    options[kCGImageSourceThumbnailMaxPixelSize] = NSNumber(value: maxPixelSize)
    options[alwaysCreate ? kCGImageSourceCreateThumbnailFromImageAlways : kCGImageSourceCreateThumbnailFromImageIfAbsent] = kCFBooleanTrue
    if transform {
        options[kCGImageSourceCreateThumbnailWithTransform] = kCFBooleanTrue
    }
    guard let image = CGImageSourceCreateThumbnailAtIndex(source, index, options) else {
        writeCString("CGImageSourceCreateThumbnailAtIndex returned nil", into: errorBuffer, capacity: errorBufferSize)
        return nil
    }
    let limits = DecodeLimits(maxWidth: maxWidth, maxHeight: maxHeight, maxBytes: maxBytes)
    guard limits.allows(width: image.width, height: image.height) else {
        reportLimitExceeded(
            width: image.width,
            height: image.height,
            widthOut: widthOut,
            heightOut: heightOut,
            limitExceeded: limitExceeded,
            errorBuffer: errorBuffer,
            errorBufferSize: errorBufferSize
        )
        return nil
    }
    guard let data = decodeCGImageToBGRA(image) else {
        writeCString("failed to decode thumbnail to BGRA", into: errorBuffer, capacity: errorBufferSize)
        return nil
    }
    widthOut?.pointee = image.width
    heightOut?.pointee = image.height
    return retainBox(data)
}
