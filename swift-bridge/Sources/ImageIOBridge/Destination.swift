import Foundation
import ImageIO

final class DestinationState {
    let destination: CGImageDestination
    let mutableData: NSMutableData?

    init(destination: CGImageDestination, mutableData: NSMutableData?) {
        self.destination = destination
        self.mutableData = mutableData
    }
}

private func destinationProperties(_ raw: UnsafeMutableRawPointer?) -> CFDictionary? {
    guard let raw else {
        return nil
    }
    return unretainedBox(raw, as: NSDictionary.self).value as CFDictionary
}

@_cdecl("imageio_destination_copy_type_identifiers")
public func imageioDestinationCopyTypeIdentifiers() -> UnsafeMutableRawPointer? {
    let identifiers = (CGImageDestinationCopyTypeIdentifiers() as? [String]) ?? []
    return retainBox(identifiers)
}

@_cdecl("imageio_destination_create_with_url")
public func imageioDestinationCreateWithURL(
    _ path: UnsafePointer<CChar>?,
    _ typeIdentifier: UnsafePointer<CChar>?,
    _ imageCount: Int,
    _ errorBuffer: UnsafeMutablePointer<CChar>?,
    _ errorBufferSize: Int
) -> UnsafeMutableRawPointer? {
    guard let path, let typeIdentifier else {
        writeCString("invalid destination path or type identifier", into: errorBuffer, capacity: errorBufferSize)
        return nil
    }
    let url = URL(fileURLWithPath: String(cString: path)) as CFURL
    let type = String(cString: typeIdentifier) as CFString
    guard let destination = CGImageDestinationCreateWithURL(url, type, imageCount, nil) else {
        writeCString("CGImageDestinationCreateWithURL returned nil", into: errorBuffer, capacity: errorBufferSize)
        return nil
    }
    return retainBox(DestinationState(destination: destination, mutableData: nil))
}

@_cdecl("imageio_destination_create_with_data")
public func imageioDestinationCreateWithData(
    _ typeIdentifier: UnsafePointer<CChar>?,
    _ imageCount: Int,
    _ errorBuffer: UnsafeMutablePointer<CChar>?,
    _ errorBufferSize: Int
) -> UnsafeMutableRawPointer? {
    guard let typeIdentifier else {
        writeCString("invalid destination type identifier", into: errorBuffer, capacity: errorBufferSize)
        return nil
    }
    let type = String(cString: typeIdentifier) as CFString
    let data = NSMutableData()
    guard let destination = CGImageDestinationCreateWithData(data as CFMutableData, type, imageCount, nil) else {
        writeCString("CGImageDestinationCreateWithData returned nil", into: errorBuffer, capacity: errorBufferSize)
        return nil
    }
    return retainBox(DestinationState(destination: destination, mutableData: data))
}

@_cdecl("imageio_destination_set_properties")
public func imageioDestinationSetProperties(
    _ raw: UnsafeMutableRawPointer?,
    _ properties: UnsafeMutableRawPointer?
) {
    guard let raw else {
        return
    }
    let state = unretainedBox(raw, as: DestinationState.self).value
    CGImageDestinationSetProperties(state.destination, destinationProperties(properties))
}

@_cdecl("imageio_destination_add_bgra_image")
public func imageioDestinationAddBgraImage(
    _ raw: UnsafeMutableRawPointer?,
    _ bytes: UnsafePointer<UInt8>?,
    _ length: Int,
    _ width: Int,
    _ height: Int,
    _ properties: UnsafeMutableRawPointer?,
    _ errorBuffer: UnsafeMutablePointer<CChar>?,
    _ errorBufferSize: Int
) -> Bool {
    guard let raw, let bytes else {
        writeCString("invalid BGRA image buffer", into: errorBuffer, capacity: errorBufferSize)
        return false
    }
    guard let image = makeCGImage(fromBGRA: bytes, length: length, width: width, height: height) else {
        writeCString("failed to build CGImage from BGRA buffer", into: errorBuffer, capacity: errorBufferSize)
        return false
    }
    let state = unretainedBox(raw, as: DestinationState.self).value
    CGImageDestinationAddImage(state.destination, image, destinationProperties(properties))
    return true
}

/// Add a CGImage directly to the destination without round-tripping through
/// host BGRA bytes. Useful when the caller already holds a CGImage (from
/// CGImageSource, VTCreateCGImageFromCVPixelBuffer, screen-capture APIs, etc.)
/// — skips one decode-encode cycle and lets the OS preserve native pixel
/// formats (e.g. YCbCr 4:2:0) end-to-end into formats that support them
/// natively (JPEG, HEIC).
@_cdecl("imageio_destination_add_cg_image")
public func imageioDestinationAddCgImage(
    _ raw: UnsafeMutableRawPointer?,
    _ cgImageRaw: UnsafeMutableRawPointer?,
    _ properties: UnsafeMutableRawPointer?,
    _ errorBuffer: UnsafeMutablePointer<CChar>?,
    _ errorBufferSize: Int
) -> Bool {
    guard let raw, let cgImageRaw else {
        writeCString("invalid destination or CGImage handle", into: errorBuffer, capacity: errorBufferSize)
        return false
    }
    let state = unretainedBox(raw, as: DestinationState.self).value
    let cgImage = Unmanaged<CGImage>.fromOpaque(cgImageRaw).takeUnretainedValue()
    CGImageDestinationAddImage(state.destination, cgImage, destinationProperties(properties))
    return true
}

@_cdecl("imageio_destination_add_bgra_image_with_metadata")
public func imageioDestinationAddBgraImageWithMetadata(
    _ raw: UnsafeMutableRawPointer?,
    _ bytes: UnsafePointer<UInt8>?,
    _ length: Int,
    _ width: Int,
    _ height: Int,
    _ metadataRaw: UnsafeMutableRawPointer?,
    _ properties: UnsafeMutableRawPointer?,
    _ errorBuffer: UnsafeMutablePointer<CChar>?,
    _ errorBufferSize: Int
) -> Bool {
    guard let raw, let bytes, let metadataRaw else {
        writeCString("invalid BGRA image buffer or metadata", into: errorBuffer, capacity: errorBufferSize)
        return false
    }
    guard let image = makeCGImage(fromBGRA: bytes, length: length, width: width, height: height) else {
        writeCString("failed to build CGImage from BGRA buffer", into: errorBuffer, capacity: errorBufferSize)
        return false
    }
    let state = unretainedBox(raw, as: DestinationState.self).value
    let metadata = unretainedBox(metadataRaw, as: CGImageMetadata.self).value
    CGImageDestinationAddImageAndMetadata(state.destination, image, metadata, destinationProperties(properties))
    return true
}

@_cdecl("imageio_destination_add_image_from_source")
public func imageioDestinationAddImageFromSource(
    _ raw: UnsafeMutableRawPointer?,
    _ sourceRaw: UnsafeMutableRawPointer?,
    _ index: Int,
    _ properties: UnsafeMutableRawPointer?,
    _ errorBuffer: UnsafeMutablePointer<CChar>?,
    _ errorBufferSize: Int
) -> Bool {
    guard let raw, let sourceRaw else {
        writeCString("invalid image source", into: errorBuffer, capacity: errorBufferSize)
        return false
    }
    let state = unretainedBox(raw, as: DestinationState.self).value
    let source = unretainedBox(sourceRaw, as: CGImageSource.self).value
    CGImageDestinationAddImageFromSource(state.destination, source, index, destinationProperties(properties))
    return true
}

@_cdecl("imageio_destination_copy_image_source")
public func imageioDestinationCopyImageSource(
    _ raw: UnsafeMutableRawPointer?,
    _ sourceRaw: UnsafeMutableRawPointer?,
    _ properties: UnsafeMutableRawPointer?,
    _ errorBuffer: UnsafeMutablePointer<CChar>?,
    _ errorBufferSize: Int
) -> Bool {
    guard let raw, let sourceRaw else {
        writeCString("invalid image source", into: errorBuffer, capacity: errorBufferSize)
        return false
    }
    let state = unretainedBox(raw, as: DestinationState.self).value
    let source = unretainedBox(sourceRaw, as: CGImageSource.self).value
    var error: Unmanaged<CFError>?
    let ok = CGImageDestinationCopyImageSource(
        state.destination,
        source,
        destinationProperties(properties),
        &error
    )
    if ok {
        return true
    }
    let message = error?.takeRetainedValue().localizedDescription ?? "CGImageDestinationCopyImageSource returned false"
    writeCString(message, into: errorBuffer, capacity: errorBufferSize)
    return false
}

@_cdecl("imageio_destination_add_auxiliary_data_info")
public func imageioDestinationAddAuxiliaryDataInfo(
    _ raw: UnsafeMutableRawPointer?,
    _ auxiliaryType: UnsafePointer<CChar>?,
    _ infoRaw: UnsafeMutableRawPointer?,
    _ errorBuffer: UnsafeMutablePointer<CChar>?,
    _ errorBufferSize: Int
) -> Bool {
    guard let raw, let auxiliaryType, let infoRaw else {
        writeCString("invalid auxiliary data info", into: errorBuffer, capacity: errorBufferSize)
        return false
    }
    let state = unretainedBox(raw, as: DestinationState.self).value
    let info = unretainedBox(infoRaw, as: AuxiliaryDataInfoBox.self).value
    let type = String(cString: auxiliaryType) as CFString
    CGImageDestinationAddAuxiliaryDataInfo(state.destination, type, info.dictionaryValue())
    return true
}

@_cdecl("imageio_destination_finalize")
public func imageioDestinationFinalize(
    _ raw: UnsafeMutableRawPointer?,
    _ errorBuffer: UnsafeMutablePointer<CChar>?,
    _ errorBufferSize: Int
) -> Bool {
    guard let raw else {
        return false
    }
    let state = unretainedBox(raw, as: DestinationState.self).value
    let ok = CGImageDestinationFinalize(state.destination)
    if !ok {
        writeCString("CGImageDestinationFinalize returned false", into: errorBuffer, capacity: errorBufferSize)
    }
    return ok
}

@_cdecl("imageio_destination_copy_data")
public func imageioDestinationCopyData(_ raw: UnsafeMutableRawPointer?) -> UnsafeMutableRawPointer? {
    guard let raw else {
        return nil
    }
    let state = unretainedBox(raw, as: DestinationState.self).value
    guard let data = state.mutableData else {
        return nil
    }
    return retainBox(data as Data)
}
