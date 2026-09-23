import Foundation
import ImageIO

func sourceCreationOptions(_ typeHint: UnsafePointer<CChar>?, _ shouldCache: Int8) -> CFDictionary? {
    var options: [CFString: Any] = [:]
    if let typeHint {
        options[kCGImageSourceTypeIdentifierHint] = String(cString: typeHint) as CFString
    }
    if shouldCache >= 0 {
        options[kCGImageSourceShouldCache] = shouldCache != 0
    }
    return options.isEmpty ? nil : options as CFDictionary
}

@_cdecl("imageio_source_copy_type_identifiers")
public func imageioSourceCopyTypeIdentifiers() -> UnsafeMutableRawPointer? {
    let identifiers = (CGImageSourceCopyTypeIdentifiers() as? [String]) ?? []
    return retainBox(identifiers)
}

@_cdecl("imageio_source_create_from_path")
public func imageioSourceCreateFromPath(
    _ path: UnsafePointer<CChar>?,
    _ typeHint: UnsafePointer<CChar>?,
    _ shouldCache: Int8,
    _ errorBuffer: UnsafeMutablePointer<CChar>?,
    _ errorBufferSize: Int
) -> UnsafeMutableRawPointer? {
    guard let path else {
        writeCString("invalid image path", into: errorBuffer, capacity: errorBufferSize)
        return nil
    }
    let url = URL(fileURLWithPath: String(cString: path)) as CFURL
    guard let source = CGImageSourceCreateWithURL(url, sourceCreationOptions(typeHint, shouldCache)) else {
        writeCString("CGImageSourceCreateWithURL returned nil", into: errorBuffer, capacity: errorBufferSize)
        return nil
    }
    return retainBox(source)
}

@_cdecl("imageio_source_create_from_bytes")
public func imageioSourceCreateFromBytes(
    _ bytes: UnsafePointer<UInt8>?,
    _ length: Int,
    _ typeHint: UnsafePointer<CChar>?,
    _ shouldCache: Int8,
    _ errorBuffer: UnsafeMutablePointer<CChar>?,
    _ errorBufferSize: Int
) -> UnsafeMutableRawPointer? {
    guard let bytes, length >= 0 else {
        writeCString("invalid image byte buffer", into: errorBuffer, capacity: errorBufferSize)
        return nil
    }
    let data = Data(bytes: bytes, count: length) as CFData
    guard let source = CGImageSourceCreateWithData(data, sourceCreationOptions(typeHint, shouldCache)) else {
        writeCString("CGImageSourceCreateWithData returned nil", into: errorBuffer, capacity: errorBufferSize)
        return nil
    }
    return retainBox(source)
}

@_cdecl("imageio_source_create_incremental")
public func imageioSourceCreateIncremental(
    _ typeHint: UnsafePointer<CChar>?,
    _ shouldCache: Int8,
    _ errorBuffer: UnsafeMutablePointer<CChar>?,
    _ errorBufferSize: Int
) -> UnsafeMutableRawPointer? {
    let source = CGImageSourceCreateIncremental(sourceCreationOptions(typeHint, shouldCache))
    return retainBox(source)
}

@_cdecl("imageio_data_provider_create_with_bytes")
public func imageioDataProviderCreateWithBytes(
    _ bytes: UnsafePointer<UInt8>?,
    _ length: Int
) -> UnsafeMutableRawPointer? {
    guard length >= 0 else {
        return nil
    }
    let data = bytes.map { Data(bytes: $0, count: length) } ?? Data()
    guard let provider = CGDataProvider(data: data as CFData) else {
        return nil
    }
    return retainBox(provider)
}

@_cdecl("imageio_data_provider_create_with_path")
public func imageioDataProviderCreateWithPath(_ path: UnsafePointer<CChar>?) -> UnsafeMutableRawPointer? {
    guard let path else {
        return nil
    }
    let url = URL(fileURLWithPath: String(cString: path)) as CFURL
    guard let provider = CGDataProvider(url: url) else {
        return nil
    }
    return retainBox(provider)
}

@_cdecl("imageio_source_create_with_data_provider")
public func imageioSourceCreateWithDataProvider(
    _ providerRaw: UnsafeMutableRawPointer?,
    _ typeHint: UnsafePointer<CChar>?,
    _ shouldCache: Int8,
    _ errorBuffer: UnsafeMutablePointer<CChar>?,
    _ errorBufferSize: Int
) -> UnsafeMutableRawPointer? {
    guard let providerRaw else {
        writeCString("invalid data provider", into: errorBuffer, capacity: errorBufferSize)
        return nil
    }
    let provider = unretainedBox(providerRaw, as: CGDataProvider.self).value
    guard let source = CGImageSourceCreateWithDataProvider(provider, sourceCreationOptions(typeHint, shouldCache)) else {
        writeCString("CGImageSourceCreateWithDataProvider returned nil", into: errorBuffer, capacity: errorBufferSize)
        return nil
    }
    return retainBox(source)
}

@_cdecl("imageio_source_update_data_provider")
public func imageioSourceUpdateDataProvider(
    _ raw: UnsafeMutableRawPointer?,
    _ providerRaw: UnsafeMutableRawPointer?,
    _ isFinal: Bool,
    _ errorBuffer: UnsafeMutablePointer<CChar>?,
    _ errorBufferSize: Int
) -> Bool {
    guard let raw, let providerRaw else {
        writeCString("invalid image source or data provider", into: errorBuffer, capacity: errorBufferSize)
        return false
    }
    let source = unretainedBox(raw, as: CGImageSource.self).value
    let provider = unretainedBox(providerRaw, as: CGDataProvider.self).value
    CGImageSourceUpdateDataProvider(source, provider, isFinal)
    return true
}

@_cdecl("imageio_source_copy_type")
public func imageioSourceCopyType(_ raw: UnsafeMutableRawPointer?) -> UnsafeMutableRawPointer? {
    guard let raw else {
        return nil
    }
    let source = unretainedBox(raw, as: CGImageSource.self).value
    guard let type = CGImageSourceGetType(source) else {
        return nil
    }
    return retainBox(type as String)
}

@_cdecl("imageio_source_get_count")
public func imageioSourceGetCount(_ raw: UnsafeMutableRawPointer?) -> Int {
    guard let raw else {
        return 0
    }
    let source = unretainedBox(raw, as: CGImageSource.self).value
    return CGImageSourceGetCount(source)
}

@_cdecl("imageio_source_get_status")
public func imageioSourceGetStatus(_ raw: UnsafeMutableRawPointer?) -> Int32 {
    guard let raw else {
        return 0
    }
    let source = unretainedBox(raw, as: CGImageSource.self).value
    return Int32(CGImageSourceGetStatus(source).rawValue)
}

@_cdecl("imageio_source_get_status_at_index")
public func imageioSourceGetStatusAtIndex(_ raw: UnsafeMutableRawPointer?, _ index: Int) -> Int32 {
    guard let raw else {
        return 0
    }
    let source = unretainedBox(raw, as: CGImageSource.self).value
    return Int32(CGImageSourceGetStatusAtIndex(source, index).rawValue)
}

@_cdecl("imageio_source_update_data")
public func imageioSourceUpdateData(
    _ raw: UnsafeMutableRawPointer?,
    _ bytes: UnsafePointer<UInt8>?,
    _ length: Int,
    _ isFinal: Bool,
    _ errorBuffer: UnsafeMutablePointer<CChar>?,
    _ errorBufferSize: Int
) -> Bool {
    guard let raw, let bytes, length >= 0 else {
        writeCString("invalid incremental image byte buffer", into: errorBuffer, capacity: errorBufferSize)
        return false
    }
    let source = unretainedBox(raw, as: CGImageSource.self).value
    let data = Data(bytes: bytes, count: length) as CFData
    CGImageSourceUpdateData(source, data, isFinal)
    return true
}

@_cdecl("imageio_source_copy_properties")
public func imageioSourceCopyProperties(
    _ raw: UnsafeMutableRawPointer?,
    _ errorBuffer: UnsafeMutablePointer<CChar>?,
    _ errorBufferSize: Int
) -> UnsafeMutableRawPointer? {
    guard let raw else {
        return nil
    }
    let source = unretainedBox(raw, as: CGImageSource.self).value
    guard let properties = CGImageSourceCopyProperties(source, nil) as NSDictionary? else {
        writeCString("CGImageSourceCopyProperties returned nil", into: errorBuffer, capacity: errorBufferSize)
        return nil
    }
    return retainBox(NSDictionary(dictionary: properties))
}

@_cdecl("imageio_source_copy_properties_at_index")
public func imageioSourceCopyPropertiesAtIndex(
    _ raw: UnsafeMutableRawPointer?,
    _ index: Int,
    _ errorBuffer: UnsafeMutablePointer<CChar>?,
    _ errorBufferSize: Int
) -> UnsafeMutableRawPointer? {
    guard let raw else {
        return nil
    }
    let source = unretainedBox(raw, as: CGImageSource.self).value
    guard let properties = CGImageSourceCopyPropertiesAtIndex(source, index, nil) as NSDictionary? else {
        writeCString("CGImageSourceCopyPropertiesAtIndex returned nil", into: errorBuffer, capacity: errorBufferSize)
        return nil
    }
    return retainBox(NSDictionary(dictionary: properties))
}

@_cdecl("imageio_source_copy_metadata_at_index")
public func imageioSourceCopyMetadataAtIndex(
    _ raw: UnsafeMutableRawPointer?,
    _ index: Int
) -> UnsafeMutableRawPointer? {
    guard let raw else {
        return nil
    }
    let source = unretainedBox(raw, as: CGImageSource.self).value
    guard let metadata = CGImageSourceCopyMetadataAtIndex(source, index, nil) else {
        return nil
    }
    return retainBox(metadata)
}

@_cdecl("imageio_source_copy_auxiliary_data_at_index")
public func imageioSourceCopyAuxiliaryDataAtIndex(
    _ raw: UnsafeMutableRawPointer?,
    _ index: Int,
    _ auxiliaryType: UnsafePointer<CChar>?,
    _ errorBuffer: UnsafeMutablePointer<CChar>?,
    _ errorBufferSize: Int
) -> UnsafeMutableRawPointer? {
    guard let raw, let auxiliaryType else {
        return nil
    }
    let source = unretainedBox(raw, as: CGImageSource.self).value
    let type = String(cString: auxiliaryType) as CFString
    guard let dictionary = CGImageSourceCopyAuxiliaryDataInfoAtIndex(source, index, type) as NSDictionary? else {
        return nil
    }
    return retainBox(AuxiliaryDataInfoBox(dictionary: dictionary))
}

@_cdecl("imageio_source_get_primary_image_index")
public func imageioSourceGetPrimaryImageIndex(_ raw: UnsafeMutableRawPointer?) -> Int {
    guard let raw else {
        return 0
    }
    let source = unretainedBox(raw, as: CGImageSource.self).value
    return CGImageSourceGetPrimaryImageIndex(source)
}

@_cdecl("imageio_source_remove_cache_at_index")
public func imageioSourceRemoveCacheAtIndex(_ raw: UnsafeMutableRawPointer?, _ index: Int) {
    guard let raw else {
        return
    }
    let source = unretainedBox(raw, as: CGImageSource.self).value
    CGImageSourceRemoveCacheAtIndex(source, index)
}

@_cdecl("imageio_source_create_bgra_at_index")
public func imageioSourceCreateBgraAtIndex(
    _ raw: UnsafeMutableRawPointer?,
    _ index: Int,
    _ maxWidth: Int,
    _ maxHeight: Int,
    _ maxBytes: Int,
    _ shouldCache: Int8,
    _ widthOut: UnsafeMutablePointer<Int>?,
    _ heightOut: UnsafeMutablePointer<Int>?,
    _ limitExceeded: UnsafeMutablePointer<Bool>?,
    _ errorBuffer: UnsafeMutablePointer<CChar>?,
    _ errorBufferSize: Int
) -> UnsafeMutableRawPointer? {
    guard let raw else {
        return nil
    }
    let source = unretainedBox(raw, as: CGImageSource.self).value
    let limits = DecodeLimits(maxWidth: maxWidth, maxHeight: maxHeight, maxBytes: maxBytes)
    if let declared = declaredPixelSize(source, at: index),
       !limits.allows(width: declared.width, height: declared.height)
    {
        reportLimitExceeded(
            width: declared.width,
            height: declared.height,
            widthOut: widthOut,
            heightOut: heightOut,
            limitExceeded: limitExceeded,
            errorBuffer: errorBuffer,
            errorBufferSize: errorBufferSize
        )
        return nil
    }
    guard let image = CGImageSourceCreateImageAtIndex(source, index, shouldCacheOptions(shouldCache)) else {
        writeCString("CGImageSourceCreateImageAtIndex returned nil", into: errorBuffer, capacity: errorBufferSize)
        return nil
    }
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
        writeCString("failed to decode image to BGRA", into: errorBuffer, capacity: errorBufferSize)
        return nil
    }
    widthOut?.pointee = image.width
    heightOut?.pointee = image.height
    return retainBox(data)
}
