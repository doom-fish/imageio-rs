import Foundation
import ImageIO

typealias ImageAnimationCallback = @convention(c) (Int, Int, Int, UnsafeMutableRawPointer?, UnsafeMutableRawPointer?) -> Bool

private final class AnimationWaitState {
    private let expectedCallbackCount: Int?
    private var callbackCount = 0
    private(set) var finished = false
    private(set) var failureMessage: String?

    init(expectedCallbackCount: Int?) {
        self.expectedCallbackCount = expectedCallbackCount
    }

    func completeFrame(keepGoing: Bool) -> Bool {
        callbackCount += 1
        let reachedNaturalEnd = expectedCallbackCount.map { callbackCount >= $0 } ?? false
        let shouldStop = !keepGoing || reachedNaturalEnd
        if shouldStop {
            finished = true
        }
        return shouldStop
    }

    func fail(_ message: String) {
        failureMessage = message
        finished = true
    }

    func wait() {
        while !finished {
            RunLoop.main.run(until: Date(timeIntervalSinceNow: 0.01))
        }
    }
}

private func pngDictionary(from raw: UnsafeMutableRawPointer?) -> NSDictionary? {
    guard let raw else {
        return nil
    }
    let properties = unretainedBox(raw, as: NSDictionary.self).value
    return properties[kCGImagePropertyPNGDictionary] as? NSDictionary
}

private func animationLoopCount(_ source: CGImageSource) -> Int {
    guard let properties = CGImageSourceCopyProperties(source, nil) as NSDictionary? else {
        return 1
    }
    if let gif = properties[kCGImagePropertyGIFDictionary] as? NSDictionary,
       let loopCount = gif[kCGImagePropertyGIFLoopCount] as? NSNumber
    {
        return loopCount.intValue
    }
    if let png = properties[kCGImagePropertyPNGDictionary] as? NSDictionary,
       let loopCount = png[kCGImagePropertyAPNGLoopCount] as? NSNumber
    {
        return loopCount.intValue
    }
    return 1
}

private func expectedAnimationCallbackCount(
    _ source: CGImageSource
) -> (count: Int?, error: String?) {
    let frameCount = CGImageSourceGetCount(source)
    guard frameCount > 0 else {
        return (nil, "image source contains zero frames")
    }
    let loopCount = animationLoopCount(source)
    if loopCount == 0 {
        return (nil, nil)
    }
    guard loopCount > 0 else {
        return (nil, "animation loop count was negative")
    }
    let (callbackCount, overflow) = frameCount.multipliedReportingOverflow(by: loopCount)
    guard !overflow else {
        return (nil, "animation callback count overflowed Swift Int")
    }
    return (callbackCount, nil)
}

private func animationStatusMessage(_ status: OSStatus) -> String {
    switch status {
    case -22_140:
        return "parameter error"
    case -22_141:
        return "corrupt input image"
    case -22_142:
        return "unsupported format"
    case -22_143:
        return "incomplete input image"
    case -22_144:
        return "allocation failure"
    default:
        return "unknown animation failure"
    }
}

private func runAnimation(
    _ source: CGImageSource,
    userData: UnsafeMutableRawPointer?,
    callback: ImageAnimationCallback,
    errorBuffer: UnsafeMutablePointer<CChar>?,
    errorBufferSize: Int,
    start: (@escaping (Int, CGImage, UnsafeMutablePointer<Bool>) -> Void) -> OSStatus
) -> Bool {
    guard Thread.isMainThread else {
        writeCString(
            "synchronous animation must be started on the process main thread",
            into: errorBuffer,
            capacity: errorBufferSize
        )
        return false
    }

    let plan = expectedAnimationCallbackCount(source)
    if let error = plan.error {
        writeCString(error, into: errorBuffer, capacity: errorBufferSize)
        return false
    }

    let state = AnimationWaitState(expectedCallbackCount: plan.count)
    let status = start { index, image, stop in
        guard Thread.isMainThread else {
            state.fail("CGImageAnimation callback was not delivered on the main queue")
            stop.pointee = true
            return
        }
        guard let data = decodeCGImageToBGRA(image) else {
            state.fail("failed to decode animation frame to BGRA")
            stop.pointee = true
            return
        }
        let keepGoing = callback(index, image.width, image.height, retainBox(data), userData)
        if state.completeFrame(keepGoing: keepGoing) {
            stop.pointee = true
        }
    }

    guard status == 0 else {
        writeCString(
            "CGImageAnimation failed with status \(status): \(animationStatusMessage(status))",
            into: errorBuffer,
            capacity: errorBufferSize
        )
        return false
    }

    state.wait()
    if let failure = state.failureMessage {
        writeCString(failure, into: errorBuffer, capacity: errorBufferSize)
        return false
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
    guard let path else {
        writeCString("invalid animation path", into: errorBuffer, capacity: errorBufferSize)
        return false
    }
    let url = URL(fileURLWithPath: String(cString: path)) as CFURL
    guard let source = CGImageSourceCreateWithURL(url, nil) else {
        writeCString("CGImageSourceCreateWithURL returned nil", into: errorBuffer, capacity: errorBufferSize)
        return false
    }
    return runAnimation(
        source,
        userData: userData,
        callback: callback,
        errorBuffer: errorBuffer,
        errorBufferSize: errorBufferSize
    ) { block in
        CGAnimateImageAtURLWithBlock(url, nil, block)
    }
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
    return runAnimation(
        source,
        userData: userData,
        callback: callback,
        errorBuffer: errorBuffer,
        errorBufferSize: errorBufferSize
    ) { block in
        CGAnimateImageDataWithBlock(data, nil, block)
    }
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
