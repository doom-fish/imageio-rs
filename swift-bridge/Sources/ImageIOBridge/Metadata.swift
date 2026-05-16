import Foundation
import ImageIO

typealias MetadataEnumerateCallback = @convention(c) (UnsafeMutableRawPointer?, UnsafeMutableRawPointer?, UnsafeMutableRawPointer?) -> Bool

@_cdecl("imageio_metadata_create_from_xmp_data")
public func imageioMetadataCreateFromXmpData(
    _ bytes: UnsafePointer<UInt8>?,
    _ length: Int,
    _ errorBuffer: UnsafeMutablePointer<CChar>?,
    _ errorBufferSize: Int
) -> UnsafeMutableRawPointer? {
    guard let bytes, length >= 0 else {
        writeCString("invalid XMP byte buffer", into: errorBuffer, capacity: errorBufferSize)
        return nil
    }
    let data = Data(bytes: bytes, count: length) as CFData
    guard let metadata = CGImageMetadataCreateFromXMPData(data) else {
        writeCString("CGImageMetadataCreateFromXMPData returned nil", into: errorBuffer, capacity: errorBufferSize)
        return nil
    }
    return retainBox(metadata)
}

@_cdecl("imageio_metadata_create_xmp_data")
public func imageioMetadataCreateXmpData(
    _ raw: UnsafeMutableRawPointer?,
    _ errorBuffer: UnsafeMutablePointer<CChar>?,
    _ errorBufferSize: Int
) -> UnsafeMutableRawPointer? {
    guard let raw else {
        return nil
    }
    let metadata = unretainedBox(raw, as: CGImageMetadata.self).value
    guard let data = CGImageMetadataCreateXMPData(metadata, nil) else {
        writeCString("CGImageMetadataCreateXMPData returned nil", into: errorBuffer, capacity: errorBufferSize)
        return nil
    }
    return retainBox(data as Data)
}

@_cdecl("imageio_mutable_metadata_create")
public func imageioMutableMetadataCreate(
    _ errorBuffer: UnsafeMutablePointer<CChar>?,
    _ errorBufferSize: Int
) -> UnsafeMutableRawPointer? {
    let metadata = CGImageMetadataCreateMutable()
    return retainBox(metadata)
}

@_cdecl("imageio_mutable_metadata_create_copy")
public func imageioMutableMetadataCreateCopy(
    _ raw: UnsafeMutableRawPointer?,
    _ errorBuffer: UnsafeMutablePointer<CChar>?,
    _ errorBufferSize: Int
) -> UnsafeMutableRawPointer? {
    guard let raw else {
        return nil
    }
    let metadata = unretainedBox(raw, as: CGImageMetadata.self).value
    guard let mutable = CGImageMetadataCreateMutableCopy(metadata) else {
        writeCString("CGImageMetadataCreateMutableCopy returned nil", into: errorBuffer, capacity: errorBufferSize)
        return nil
    }
    return retainBox(mutable)
}

@_cdecl("imageio_mutable_metadata_into_immutable")
public func imageioMutableMetadataIntoImmutable(_ raw: UnsafeMutableRawPointer?) -> UnsafeMutableRawPointer? {
    guard let raw else {
        return nil
    }
    let metadata = unretainedBox(raw, as: CGMutableImageMetadata.self).value
    return retainBox(metadata as CGImageMetadata)
}

@_cdecl("imageio_metadata_copy_tags")
public func imageioMetadataCopyTags(_ raw: UnsafeMutableRawPointer?) -> UnsafeMutableRawPointer? {
    guard let raw else {
        return nil
    }
    let metadata = unretainedBox(raw, as: CGImageMetadata.self).value
    let tags = (CGImageMetadataCopyTags(metadata) as? [CGImageMetadataTag]) ?? []
    return retainBox(tags)
}

@_cdecl("imageio_metadata_tag_array_count")
public func imageioMetadataTagArrayCount(_ raw: UnsafeMutableRawPointer?) -> Int {
    guard let raw else {
        return 0
    }
    return unretainedBox(raw, as: [CGImageMetadataTag].self).value.count
}

@_cdecl("imageio_metadata_tag_array_copy_item")
public func imageioMetadataTagArrayCopyItem(
    _ raw: UnsafeMutableRawPointer?,
    _ index: Int
) -> UnsafeMutableRawPointer? {
    guard let raw else {
        return nil
    }
    let tags = unretainedBox(raw, as: [CGImageMetadataTag].self).value
    guard tags.indices.contains(index) else {
        return nil
    }
    return retainBox(tags[index])
}

@_cdecl("imageio_metadata_copy_tag_with_path")
public func imageioMetadataCopyTagWithPath(
    _ raw: UnsafeMutableRawPointer?,
    _ path: UnsafePointer<CChar>?,
    _ errorBuffer: UnsafeMutablePointer<CChar>?,
    _ errorBufferSize: Int
) -> UnsafeMutableRawPointer? {
    guard let raw, let path else {
        return nil
    }
    let metadata = unretainedBox(raw, as: CGImageMetadata.self).value
    let pathString = String(cString: path) as CFString
    guard let tag = CGImageMetadataCopyTagWithPath(metadata, nil, pathString) else {
        return nil
    }
    return retainBox(tag)
}

@_cdecl("imageio_metadata_copy_string_value_with_path")
public func imageioMetadataCopyStringValueWithPath(
    _ raw: UnsafeMutableRawPointer?,
    _ path: UnsafePointer<CChar>?,
    _ errorBuffer: UnsafeMutablePointer<CChar>?,
    _ errorBufferSize: Int
) -> UnsafeMutableRawPointer? {
    guard let raw, let path else {
        return nil
    }
    let metadata = unretainedBox(raw, as: CGImageMetadata.self).value
    let pathString = String(cString: path) as CFString
    guard let value = CGImageMetadataCopyStringValueWithPath(metadata, nil, pathString) else {
        return nil
    }
    return retainBox(value as String)
}

@_cdecl("imageio_metadata_enumerate_tags")
func imageioMetadataEnumerateTags(
    _ raw: UnsafeMutableRawPointer?,
    _ rootPath: UnsafePointer<CChar>?,
    _ recursive: Bool,
    _ userData: UnsafeMutableRawPointer?,
    _ callback: MetadataEnumerateCallback?,
    _ errorBuffer: UnsafeMutablePointer<CChar>?,
    _ errorBufferSize: Int
) -> Bool {
    guard let raw, let callback else {
        writeCString("metadata enumeration callback was nil", into: errorBuffer, capacity: errorBufferSize)
        return false
    }
    let metadata = unretainedBox(raw, as: CGImageMetadata.self).value
    let root = rootPath.map { String(cString: $0) as CFString }
    let options: CFDictionary?
    if recursive {
        options = [kCGImageMetadataEnumerateRecursively: kCFBooleanTrue] as CFDictionary
    } else {
        options = nil
    }
    CGImageMetadataEnumerateTagsUsingBlock(metadata, root, options) { path, tag in
        let pathHandle = retainBox(path as String)
        let tagHandle = retainBox(tag)
        return callback(pathHandle, tagHandle, userData)
    }
    return true
}

@_cdecl("imageio_metadata_error_domain")
public func imageioMetadataErrorDomain() -> UnsafeMutableRawPointer? {
    retainBox(kCFErrorDomainCGImageMetadata as String)
}

@_cdecl("imageio_metadata_register_namespace_for_prefix")
public func imageioMetadataRegisterNamespaceForPrefix(
    _ raw: UnsafeMutableRawPointer?,
    _ xmlns: UnsafePointer<CChar>?,
    _ prefix: UnsafePointer<CChar>?,
    _ errorBuffer: UnsafeMutablePointer<CChar>?,
    _ errorBufferSize: Int
) -> Bool {
    guard let raw, let xmlns, let prefix else {
        writeCString("invalid namespace or prefix", into: errorBuffer, capacity: errorBufferSize)
        return false
    }
    let metadata = unretainedBox(raw, as: CGMutableImageMetadata.self).value
    var error: Unmanaged<CFError>?
    let ok = CGImageMetadataRegisterNamespaceForPrefix(
        metadata,
        String(cString: xmlns) as CFString,
        String(cString: prefix) as CFString,
        &error
    )
    if !ok {
        let message = error?.takeRetainedValue().localizedDescription ?? "CGImageMetadataRegisterNamespaceForPrefix returned false"
        writeCString(message, into: errorBuffer, capacity: errorBufferSize)
    }
    return ok
}

@_cdecl("imageio_metadata_set_tag_with_path")
public func imageioMetadataSetTagWithPath(
    _ raw: UnsafeMutableRawPointer?,
    _ path: UnsafePointer<CChar>?,
    _ tagRaw: UnsafeMutableRawPointer?,
    _ errorBuffer: UnsafeMutablePointer<CChar>?,
    _ errorBufferSize: Int
) -> Bool {
    guard let raw, let path, let tagRaw else {
        writeCString("invalid metadata tag or path", into: errorBuffer, capacity: errorBufferSize)
        return false
    }
    let metadata = unretainedBox(raw, as: CGMutableImageMetadata.self).value
    let tag = unretainedBox(tagRaw, as: CGImageMetadataTag.self).value
    let ok = CGImageMetadataSetTagWithPath(metadata, nil, String(cString: path) as CFString, tag)
    if !ok {
        writeCString("CGImageMetadataSetTagWithPath returned false", into: errorBuffer, capacity: errorBufferSize)
    }
    return ok
}

@_cdecl("imageio_metadata_set_string_value_with_path")
public func imageioMetadataSetStringValueWithPath(
    _ raw: UnsafeMutableRawPointer?,
    _ path: UnsafePointer<CChar>?,
    _ value: UnsafePointer<CChar>?,
    _ errorBuffer: UnsafeMutablePointer<CChar>?,
    _ errorBufferSize: Int
) -> Bool {
    guard let raw, let path, let value else {
        writeCString("invalid metadata value or path", into: errorBuffer, capacity: errorBufferSize)
        return false
    }
    let metadata = unretainedBox(raw, as: CGMutableImageMetadata.self).value
    let ok = CGImageMetadataSetValueWithPath(
        metadata,
        nil,
        String(cString: path) as CFString,
        String(cString: value) as CFString
    )
    if !ok {
        writeCString("CGImageMetadataSetValueWithPath returned false", into: errorBuffer, capacity: errorBufferSize)
    }
    return ok
}

@_cdecl("imageio_metadata_remove_tag_with_path")
public func imageioMetadataRemoveTagWithPath(
    _ raw: UnsafeMutableRawPointer?,
    _ path: UnsafePointer<CChar>?,
    _ errorBuffer: UnsafeMutablePointer<CChar>?,
    _ errorBufferSize: Int
) -> Bool {
    guard let raw, let path else {
        writeCString("invalid metadata path", into: errorBuffer, capacity: errorBufferSize)
        return false
    }
    let metadata = unretainedBox(raw, as: CGMutableImageMetadata.self).value
    let ok = CGImageMetadataRemoveTagWithPath(metadata, nil, String(cString: path) as CFString)
    if !ok {
        writeCString("CGImageMetadataRemoveTagWithPath returned false", into: errorBuffer, capacity: errorBufferSize)
    }
    return ok
}

@_cdecl("imageio_metadata_tag_create_string")
public func imageioMetadataTagCreateString(
    _ xmlns: UnsafePointer<CChar>?,
    _ prefix: UnsafePointer<CChar>?,
    _ name: UnsafePointer<CChar>?,
    _ value: UnsafePointer<CChar>?,
    _ errorBuffer: UnsafeMutablePointer<CChar>?,
    _ errorBufferSize: Int
) -> UnsafeMutableRawPointer? {
    guard let xmlns, let name, let value else {
        writeCString("invalid metadata tag inputs", into: errorBuffer, capacity: errorBufferSize)
        return nil
    }
    let prefixValue = prefix.map { String(cString: $0) as CFString }
    guard let tag = CGImageMetadataTagCreate(
        String(cString: xmlns) as CFString,
        prefixValue,
        String(cString: name) as CFString,
        .string,
        String(cString: value) as CFString
    ) else {
        writeCString("CGImageMetadataTagCreate returned nil", into: errorBuffer, capacity: errorBufferSize)
        return nil
    }
    return retainBox(tag)
}

@_cdecl("imageio_metadata_tag_copy_namespace")
public func imageioMetadataTagCopyNamespace(_ raw: UnsafeMutableRawPointer?) -> UnsafeMutableRawPointer? {
    guard let raw else {
        return nil
    }
    let tag = unretainedBox(raw, as: CGImageMetadataTag.self).value
    guard let value = CGImageMetadataTagCopyNamespace(tag) else {
        return nil
    }
    return retainBox(value as String)
}

@_cdecl("imageio_metadata_tag_copy_prefix")
public func imageioMetadataTagCopyPrefix(_ raw: UnsafeMutableRawPointer?) -> UnsafeMutableRawPointer? {
    guard let raw else {
        return nil
    }
    let tag = unretainedBox(raw, as: CGImageMetadataTag.self).value
    guard let value = CGImageMetadataTagCopyPrefix(tag) else {
        return nil
    }
    return retainBox(value as String)
}

@_cdecl("imageio_metadata_tag_copy_name")
public func imageioMetadataTagCopyName(_ raw: UnsafeMutableRawPointer?) -> UnsafeMutableRawPointer? {
    guard let raw else {
        return nil
    }
    let tag = unretainedBox(raw, as: CGImageMetadataTag.self).value
    guard let value = CGImageMetadataTagCopyName(tag) else {
        return nil
    }
    return retainBox(value as String)
}

@_cdecl("imageio_metadata_tag_copy_string_value")
public func imageioMetadataTagCopyStringValue(_ raw: UnsafeMutableRawPointer?) -> UnsafeMutableRawPointer? {
    guard let raw else {
        return nil
    }
    let tag = unretainedBox(raw, as: CGImageMetadataTag.self).value
    guard let value = CGImageMetadataTagCopyValue(tag) as? String else {
        return nil
    }
    return retainBox(value)
}

@_cdecl("imageio_metadata_tag_get_type")
public func imageioMetadataTagGetType(_ raw: UnsafeMutableRawPointer?) -> Int32 {
    guard let raw else {
        return -1
    }
    let tag = unretainedBox(raw, as: CGImageMetadataTag.self).value
    return Int32(CGImageMetadataTagGetType(tag).rawValue)
}

@_cdecl("imageio_metadata_tag_copy_qualifiers")
public func imageioMetadataTagCopyQualifiers(_ raw: UnsafeMutableRawPointer?) -> UnsafeMutableRawPointer? {
    guard let raw else {
        return nil
    }
    let tag = unretainedBox(raw, as: CGImageMetadataTag.self).value
    let qualifiers = (CGImageMetadataTagCopyQualifiers(tag) as? [CGImageMetadataTag]) ?? []
    return retainBox(qualifiers)
}
