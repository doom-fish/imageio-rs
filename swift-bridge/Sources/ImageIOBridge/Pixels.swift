import CoreGraphics
import Foundation
import ImageIO

private func straightRgba8(_ image: CGImage) -> Data? {
    let bits = image.bitsPerComponent
    let model = image.colorSpace?.model
    guard (bits == 8 || bits == 16), model == .rgb || model == .monochrome,
          !image.bitmapInfo.contains(.floatComponents),
          let data = image.dataProvider?.data,
          let source = CFDataGetBytePtr(data) else {
        return nil
    }
    let channels = model == .rgb ? 3 : 1
    let components = image.bitsPerPixel / bits
    let first: Int
    let alpha: Int?
    switch image.alphaInfo {
    case .none:
        first = 0
        alpha = nil
        guard components == channels else { return nil }
    case .last:
        first = 0
        alpha = channels
        guard components == channels + 1 else { return nil }
    case .first:
        first = 1
        alpha = 0
        guard components == channels + 1 else { return nil }
    case .noneSkipLast:
        first = 0
        alpha = nil
        guard components == channels + 1 else { return nil }
    case .noneSkipFirst:
        first = 1
        alpha = nil
        guard components == channels + 1 else { return nil }
    default:
        return nil
    }
    let sampleBytes = bits / 8
    let pixelBytes = components * sampleBytes
    let (required, overflow) = image.bytesPerRow.multipliedReportingOverflow(by: image.height)
    guard !overflow, required <= CFDataGetLength(data),
          image.width <= image.bytesPerRow / pixelBytes else {
        return nil
    }
    let order = image.bitmapInfo.intersection(.byteOrderMask)
    let reverseComponents = bits == 8 && components == 4 && order == .byteOrder32Little
    let littleSamples = order == .byteOrder16Little
    var output = Data(count: image.width * image.height * 4)
    output.withUnsafeMutableBytes { (target: UnsafeMutableRawBufferPointer) in
        for y in 0..<image.height {
            for x in 0..<image.width {
                let base = y * image.bytesPerRow + x * pixelBytes
                func sample(_ component: Int) -> UInt8 {
                    let component = reverseComponents ? components - 1 - component : component
                    let offset = base + component * sampleBytes
                    if bits == 8 { return source[offset] }
                    let high = Int(source[offset + (littleSamples ? 1 : 0)])
                    let low = Int(source[offset + (littleSamples ? 0 : 1)])
                    return UInt8(((high * 256 + low) * 255 + 32767) / 65535)
                }
                let targetBase = (y * image.width + x) * 4
                target[targetBase] = sample(first)
                target[targetBase + 1] = sample(first + (channels == 3 ? 1 : 0))
                target[targetBase + 2] = sample(first + (channels == 3 ? 2 : 0))
                target[targetBase + 3] = alpha.map(sample) ?? 255
            }
        }
    }
    return output
}

@_cdecl("imageio_source_create_rgba8_at_index")
public func imageioSourceCreateRgba8AtIndex(
    _ raw: UnsafeMutableRawPointer?,
    _ index: Int,
    _ maxWidth: Int,
    _ maxHeight: Int,
    _ maxBytes: Int,
    _ widthOut: UnsafeMutablePointer<Int>?,
    _ heightOut: UnsafeMutablePointer<Int>?,
    _ limitExceeded: UnsafeMutablePointer<Bool>?,
    _ errorBuffer: UnsafeMutablePointer<CChar>?,
    _ errorBufferSize: Int
) -> UnsafeMutableRawPointer? {
    guard let raw else { return nil }
    let source = unretainedBox(raw, as: CGImageSource.self).value
    let limits = DecodeLimits(maxWidth: maxWidth, maxHeight: maxHeight, maxBytes: maxBytes)
    if let declared = declaredPixelSize(source, at: index),
       !limits.allows(width: declared.width, height: declared.height) {
        reportLimitExceeded(
            width: declared.width, height: declared.height,
            widthOut: widthOut, heightOut: heightOut, limitExceeded: limitExceeded,
            errorBuffer: errorBuffer, errorBufferSize: errorBufferSize
        )
        return nil
    }
    guard let image = CGImageSourceCreateImageAtIndex(source, index, nil) else {
        writeCString("CGImageSourceCreateImageAtIndex returned nil", into: errorBuffer, capacity: errorBufferSize)
        return nil
    }
    guard limits.allows(width: image.width, height: image.height) else {
        reportLimitExceeded(
            width: image.width, height: image.height,
            widthOut: widthOut, heightOut: heightOut, limitExceeded: limitExceeded,
            errorBuffer: errorBuffer, errorBufferSize: errorBufferSize
        )
        return nil
    }
    guard let pixels = straightRgba8(image) else {
        writeCString("unsupported straight RGB or grayscale pixel layout", into: errorBuffer, capacity: errorBufferSize)
        return nil
    }
    widthOut?.pointee = image.width
    heightOut?.pointee = image.height
    return retainBox(pixels)
}

@_cdecl("imageio_encode_pixels")
public func imageioEncodePixels(
    _ bytes: UnsafePointer<UInt8>?,
    _ length: Int,
    _ width: Int,
    _ height: Int,
    _ channels: Int,
    _ typeIdentifier: UnsafePointer<CChar>?,
    _ errorBuffer: UnsafeMutablePointer<CChar>?,
    _ errorBufferSize: Int
) -> UnsafeMutableRawPointer? {
    guard let bytes, let typeIdentifier, width > 0, height > 0,
          channels == 3 || channels == 4 else {
        writeCString("invalid pixel buffer", into: errorBuffer, capacity: errorBufferSize)
        return nil
    }
    let (stride, rowOverflow) = width.multipliedReportingOverflow(by: channels)
    let (required, lengthOverflow) = stride.multipliedReportingOverflow(by: height)
    guard !rowOverflow, !lengthOverflow, required == length else {
        writeCString("invalid pixel count", into: errorBuffer, capacity: errorBufferSize)
        return nil
    }
    let data = Data(bytes: bytes, count: length)
    guard let provider = CGDataProvider(data: data as CFData),
          let colorSpace = CGColorSpace(name: CGColorSpace.sRGB),
          let image = CGImage(
              width: width, height: height, bitsPerComponent: 8,
              bitsPerPixel: channels * 8, bytesPerRow: stride, space: colorSpace,
              bitmapInfo: CGBitmapInfo(rawValue: channels == 4 ? CGImageAlphaInfo.last.rawValue : CGImageAlphaInfo.none.rawValue),
              provider: provider, decode: nil, shouldInterpolate: false, intent: .defaultIntent
          ) else {
        writeCString("could not create the pixel image", into: errorBuffer, capacity: errorBufferSize)
        return nil
    }
    let output = NSMutableData()
    guard let destination = CGImageDestinationCreateWithData(output, String(cString: typeIdentifier) as CFString, 1, nil) else {
        writeCString("could not create the image destination", into: errorBuffer, capacity: errorBufferSize)
        return nil
    }
    CGImageDestinationAddImage(destination, image, nil)
    guard CGImageDestinationFinalize(destination) else {
        writeCString("could not finalize the image destination", into: errorBuffer, capacity: errorBufferSize)
        return nil
    }
    return retainBox(output as Data)
}
