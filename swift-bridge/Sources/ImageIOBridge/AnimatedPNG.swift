import Foundation
import ImageIO

typealias ImageAnimationCallback = @convention(c) (Int, Int, Int, UnsafeMutableRawPointer?, UnsafeMutableRawPointer?) -> Bool

private let animationSucceeded: Int32 = 0
private let animationFailed: Int32 = 1
private let animationTimedOut: Int32 = 2
private let animationLimitExceeded: Int32 = 3

private struct AnimationFailure {
    let status: Int32
    let message: String
    let width: Int
    let height: Int
}

private final class AnimationWaitState {
    private let lock = NSLock()
    private let expectedCallbackCount: Int?
    private var callbackCount = 0
    private var finished = false
    private var ended = false
    private var detached = false
    private var inCallback = false
    private var failure: AnimationFailure?

    init(expectedCallbackCount: Int?) {
        self.expectedCallbackCount = expectedCallbackCount
    }

    func beginFrame() -> Bool {
        lock.lock()
        defer { lock.unlock() }
        guard !finished, !detached, !inCallback else {
            return false
        }
        inCallback = true
        return true
    }

    func completeFrame(keepGoing: Bool) -> Bool {
        lock.lock()
        defer { lock.unlock() }
        inCallback = false
        callbackCount += 1
        let reachedNaturalEnd = expectedCallbackCount.map { callbackCount >= $0 } ?? false
        let shouldStop = !keepGoing || reachedNaturalEnd
        if shouldStop {
            finished = true
        }
        return shouldStop
    }

    func isReentrant() -> Bool {
        lock.lock()
        defer { lock.unlock() }
        return inCallback
    }

    func fail(_ status: Int32, _ message: String, width: Int = 0, height: Int = 0) {
        lock.lock()
        defer { lock.unlock() }
        if failure == nil {
            failure = AnimationFailure(status: status, message: message, width: width, height: height)
        }
        finished = true
    }

    func animationEnded() {
        lock.lock()
        ended = true
        lock.unlock()
    }

    private func isDone() -> Bool {
        lock.lock()
        defer { lock.unlock() }
        return finished || ended
    }

    func wait(timeoutNanoseconds: UInt64) -> Bool {
        let deadline = timeoutNanoseconds == UInt64.max
            ? Date.distantFuture
            : Date(timeIntervalSinceNow: Double(timeoutNanoseconds) / 1_000_000_000)
        while !isDone() {
            let now = Date()
            if now >= deadline {
                return false
            }
            RunLoop.main.run(until: min(deadline, now.addingTimeInterval(0.01)))
        }
        return true
    }

    func detach() -> AnimationFailure? {
        lock.lock()
        defer { lock.unlock() }
        detached = true
        return failure
    }
}

private final class AnimationEndSentinel {
    private let state: AnimationWaitState

    init(_ state: AnimationWaitState) {
        self.state = state
    }

    deinit {
        state.animationEnded()
    }
}

private func makeAnimationBlock(
    state: AnimationWaitState,
    limits: DecodeLimits,
    userData: UnsafeMutableRawPointer?,
    callback: @escaping ImageAnimationCallback
) -> (Int, CGImage, UnsafeMutablePointer<Bool>) -> Void {
    let sentinel = AnimationEndSentinel(state)
    return { index, image, stop in
        withExtendedLifetime(sentinel) {
            guard Thread.isMainThread else {
                state.fail(animationFailed, "CGImageAnimation callback was not delivered on the main queue")
                stop.pointee = true
                return
            }
            if state.isReentrant() {
                state.fail(animationFailed, "animation callback re-entered the main run loop")
                stop.pointee = true
                return
            }
            guard state.beginFrame() else {
                stop.pointee = true
                return
            }
            guard limits.allows(width: image.width, height: image.height) else {
                _ = state.completeFrame(keepGoing: false)
                state.fail(
                    animationLimitExceeded,
                    "animation frame dimensions exceed the decode limits",
                    width: image.width,
                    height: image.height
                )
                stop.pointee = true
                return
            }
            guard let data = decodeCGImageToBGRA(image) else {
                _ = state.completeFrame(keepGoing: false)
                state.fail(animationFailed, "failed to decode animation frame to BGRA")
                stop.pointee = true
                return
            }
            let keepGoing = callback(index, image.width, image.height, retainBox(data), userData)
            if state.completeFrame(keepGoing: keepGoing) {
                stop.pointee = true
            }
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
    callback: @escaping ImageAnimationCallback,
    limits: DecodeLimits,
    timeoutNanoseconds: UInt64,
    widthOut: UnsafeMutablePointer<Int>?,
    heightOut: UnsafeMutablePointer<Int>?,
    errorBuffer: UnsafeMutablePointer<CChar>?,
    errorBufferSize: Int,
    start: (@escaping (Int, CGImage, UnsafeMutablePointer<Bool>) -> Void) -> OSStatus
) -> Int32 {
    guard Thread.isMainThread else {
        writeCString(
            "synchronous animation must be started on the process main thread",
            into: errorBuffer,
            capacity: errorBufferSize
        )
        return animationFailed
    }

    let plan = expectedAnimationCallbackCount(source)
    if let error = plan.error {
        writeCString(error, into: errorBuffer, capacity: errorBufferSize)
        return animationFailed
    }

    let state = AnimationWaitState(expectedCallbackCount: plan.count)
    let status = start(makeAnimationBlock(state: state, limits: limits, userData: userData, callback: callback))

    guard status == 0 else {
        _ = state.detach()
        writeCString(
            "CGImageAnimation failed with status \(status): \(animationStatusMessage(status))",
            into: errorBuffer,
            capacity: errorBufferSize
        )
        return animationFailed
    }

    let completed = state.wait(timeoutNanoseconds: timeoutNanoseconds)
    if let failure = state.detach() {
        widthOut?.pointee = failure.width
        heightOut?.pointee = failure.height
        writeCString(failure.message, into: errorBuffer, capacity: errorBufferSize)
        return failure.status
    }
    guard completed else {
        writeCString("animation did not finish before the timeout", into: errorBuffer, capacity: errorBufferSize)
        return animationTimedOut
    }
    return animationSucceeded
}

@_cdecl("imageio_animate_image_at_path")
func imageioAnimateImageAtPath(
    _ path: UnsafePointer<CChar>?,
    _ userData: UnsafeMutableRawPointer?,
    _ callback: ImageAnimationCallback?,
    _ maxWidth: Int,
    _ maxHeight: Int,
    _ maxBytes: Int,
    _ timeoutNanoseconds: UInt64,
    _ widthOut: UnsafeMutablePointer<Int>?,
    _ heightOut: UnsafeMutablePointer<Int>?,
    _ errorBuffer: UnsafeMutablePointer<CChar>?,
    _ errorBufferSize: Int
) -> Int32 {
    guard let callback else {
        writeCString("animation callback was nil", into: errorBuffer, capacity: errorBufferSize)
        return animationFailed
    }
    guard let path else {
        writeCString("invalid animation path", into: errorBuffer, capacity: errorBufferSize)
        return animationFailed
    }
    let url = URL(fileURLWithPath: String(cString: path)) as CFURL
    guard let source = CGImageSourceCreateWithURL(url, nil) else {
        writeCString("CGImageSourceCreateWithURL returned nil", into: errorBuffer, capacity: errorBufferSize)
        return animationFailed
    }
    return runAnimation(
        source,
        userData: userData,
        callback: callback,
        limits: DecodeLimits(maxWidth: maxWidth, maxHeight: maxHeight, maxBytes: maxBytes),
        timeoutNanoseconds: timeoutNanoseconds,
        widthOut: widthOut,
        heightOut: heightOut,
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
    _ maxWidth: Int,
    _ maxHeight: Int,
    _ maxBytes: Int,
    _ timeoutNanoseconds: UInt64,
    _ widthOut: UnsafeMutablePointer<Int>?,
    _ heightOut: UnsafeMutablePointer<Int>?,
    _ errorBuffer: UnsafeMutablePointer<CChar>?,
    _ errorBufferSize: Int
) -> Int32 {
    guard let bytes, let callback, length >= 0 else {
        writeCString("invalid animation byte buffer or callback", into: errorBuffer, capacity: errorBufferSize)
        return animationFailed
    }
    let data = Data(bytes: bytes, count: length) as CFData
    guard let source = CGImageSourceCreateWithData(data, nil) else {
        writeCString("CGImageSourceCreateWithData returned nil", into: errorBuffer, capacity: errorBufferSize)
        return animationFailed
    }
    return runAnimation(
        source,
        userData: userData,
        callback: callback,
        limits: DecodeLimits(maxWidth: maxWidth, maxHeight: maxHeight, maxBytes: maxBytes),
        timeoutNanoseconds: timeoutNanoseconds,
        widthOut: widthOut,
        heightOut: heightOut,
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
