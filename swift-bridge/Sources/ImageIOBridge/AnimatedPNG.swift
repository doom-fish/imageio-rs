import Foundation
import ImageIO

typealias ImageAnimationCallback = @convention(c) (Int, Int, Int, UnsafeMutableRawPointer?, UnsafeMutableRawPointer?) -> Bool

private func pngDictionary(from raw: UnsafeMutableRawPointer?) -> NSDictionary? {
    guard let raw else {
        return nil
    }
    let properties = unretainedBox(raw, as: NSDictionary.self).value
    return properties[kCGImagePropertyPNGDictionary] as? NSDictionary
}

private func animateSource(
    _ source: CGImageSource,
    userData: UnsafeMutableRawPointer?,
    callback: ImageAnimationCallback,
    errorBuffer: UnsafeMutablePointer<CChar>?,
    errorBufferSize: Int
) -> Bool {
    let count = CGImageSourceGetCount(source)
    guard count > 0 else {
        writeCString("image source contains zero frames", into: errorBuffer, capacity: errorBufferSize)
        return false
    }
    for index in 0 ..< count {
        guard let image = CGImageSourceCreateImageAtIndex(source, index, nil) else {
            writeCString("CGImageSourceCreateImageAtIndex returned nil", into: errorBuffer, capacity: errorBufferSize)
            return false
        }
        guard let data = decodeCGImageToBGRA(image) else {
            writeCString("failed to decode animation frame to BGRA", into: errorBuffer, capacity: errorBufferSize)
            return false
        }
        let keepGoing = callback(index, image.width, image.height, retainBox(data), userData)
        if !keepGoing {
            return true
        }
    }
    return true
}

@_cdecl("imageio_animate_image_at_path")
func imageioAnimateImageAtPath(
    _ path: UnsafePointer<CChar>?,
    _ userData: UnsafeMutableRawPointer?,
    _ callback: ImageAnimationCallback?,
    _ errorBuffer: UnsafeMutablePointer<CChar>?,
    _ errorBufferSize: Int
) -> Bool {
    guard let callback else {
        writeCString("animation callback was nil", into: errorBuffer, capacity: errorBufferSize)
        return false
    }
    guard let source = sourceFromPath(path) else {
        writeCString("CGImageSourceCreateWithURL returned nil", into: errorBuffer, capacity: errorBufferSize)
        return false
    }
    return animateSource(source, userData: userData, callback: callback, errorBuffer: errorBuffer, errorBufferSize: errorBufferSize)
}

@_cdecl("imageio_animate_image_data")
func imageioAnimateImageData(
    _ bytes: UnsafePointer<UInt8>?,
    _ length: Int,
    _ userData: UnsafeMutableRawPointer?,
    _ callback: ImageAnimationCallback?,
    _ errorBuffer: UnsafeMutablePointer<CChar>?,
    _ errorBufferSize: Int
) -> Bool {
    guard let bytes, let callback, length >= 0 else {
        writeCString("invalid animation byte buffer or callback", into: errorBuffer, capacity: errorBufferSize)
        return false
    }
    let data = Data(bytes: bytes, count: length) as CFData
    guard let source = CGImageSourceCreateWithData(data, nil) else {
        writeCString("CGImageSourceCreateWithData returned nil", into: errorBuffer, capacity: errorBufferSize)
        return false
    }
    return animateSource(source, userData: userData, callback: callback, errorBuffer: errorBuffer, errorBufferSize: errorBufferSize)
}

@_cdecl("imageio_apng_copy_dictionary")
public func imageioApngCopyDictionary(_ raw: UnsafeMutableRawPointer?) -> UnsafeMutableRawPointer? {
    guard let dictionary = pngDictionary(from: raw) else {
        return nil
    }
    return retainBox(NSDictionary(dictionary: dictionary))
}

@_cdecl("imageio_apng_get_loop_count")
public func imageioApngGetLoopCount(_ raw: UnsafeMutableRawPointer?, _ outValue: UnsafeMutablePointer<Int64>?) -> Bool {
    guard let dictionary = pngDictionary(from: raw), let outValue, let number = dictionary[kCGImagePropertyAPNGLoopCount] as? NSNumber else {
        return false
    }
    outValue.pointee = number.int64Value
    return true
}

@_cdecl("imageio_apng_get_delay_time")
public func imageioApngGetDelayTime(_ raw: UnsafeMutableRawPointer?, _ outValue: UnsafeMutablePointer<Double>?) -> Bool {
    guard let dictionary = pngDictionary(from: raw), let outValue, let number = dictionary[kCGImagePropertyAPNGDelayTime] as? NSNumber else {
        return false
    }
    outValue.pointee = number.doubleValue
    return true
}

@_cdecl("imageio_apng_get_unclamped_delay_time")
public func imageioApngGetUnclampedDelayTime(_ raw: UnsafeMutableRawPointer?, _ outValue: UnsafeMutablePointer<Double>?) -> Bool {
    guard let dictionary = pngDictionary(from: raw), let outValue, let number = dictionary[kCGImagePropertyAPNGUnclampedDelayTime] as? NSNumber else {
        return false
    }
    outValue.pointee = number.doubleValue
    return true
}

@_cdecl("imageio_apng_get_canvas_width")
public func imageioApngGetCanvasWidth(_ raw: UnsafeMutableRawPointer?, _ outValue: UnsafeMutablePointer<Int64>?) -> Bool {
    guard let dictionary = pngDictionary(from: raw), let outValue, let number = dictionary[kCGImagePropertyAPNGCanvasPixelWidth] as? NSNumber else {
        return false
    }
    outValue.pointee = number.int64Value
    return true
}

@_cdecl("imageio_apng_get_canvas_height")
public func imageioApngGetCanvasHeight(_ raw: UnsafeMutableRawPointer?, _ outValue: UnsafeMutablePointer<Int64>?) -> Bool {
    guard let dictionary = pngDictionary(from: raw), let outValue, let number = dictionary[kCGImagePropertyAPNGCanvasPixelHeight] as? NSNumber else {
        return false
    }
    outValue.pointee = number.int64Value
    return true
}

@_cdecl("imageio_apng_get_frame_info_count")
public func imageioApngGetFrameInfoCount(_ raw: UnsafeMutableRawPointer?) -> Int {
    guard let dictionary = pngDictionary(from: raw), let frames = dictionary[kCGImagePropertyAPNGFrameInfoArray] as? [Any] else {
        return 0
    }
    return frames.count
}
