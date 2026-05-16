import Foundation
import ImageIO

final class AuxiliaryDataInfoBox {
    var data: Data = Data()
    var description: NSDictionary?
    var metadata: CGImageMetadata?
    var hasColorSpace = false

    init() {}

    init(dictionary: NSDictionary) {
        if let value = dictionary[kCGImageAuxiliaryDataInfoData] as? Data {
            data = value
        }
        if let value = dictionary[kCGImageAuxiliaryDataInfoDataDescription] as? NSDictionary {
            description = NSDictionary(dictionary: value)
        }
        if dictionary[kCGImageAuxiliaryDataInfoMetadata] != nil {
            metadata = dictionary[kCGImageAuxiliaryDataInfoMetadata] as! CGImageMetadata
        }
        if #available(macOS 15.0, *) {
            hasColorSpace = dictionary[kCGImageAuxiliaryDataInfoColorSpace] != nil
        }
    }

    func dictionaryValue() -> NSDictionary {
        let dictionary = NSMutableDictionary()
        dictionary[kCGImageAuxiliaryDataInfoData] = data as CFData
        if let description {
            dictionary[kCGImageAuxiliaryDataInfoDataDescription] = description
        }
        if let metadata {
            dictionary[kCGImageAuxiliaryDataInfoMetadata] = metadata
        }
        if #available(macOS 15.0, *), hasColorSpace {
            dictionary[kCGImageAuxiliaryDataInfoColorSpace] = CGColorSpaceCreateDeviceRGB()
        }
        return NSDictionary(dictionary: dictionary)
    }
}

@_cdecl("imageio_auxiliary_data_info_create")
public func imageioAuxiliaryDataInfoCreate() -> UnsafeMutableRawPointer? {
    retainBox(AuxiliaryDataInfoBox())
}

@_cdecl("imageio_auxiliary_data_info_set_data")
public func imageioAuxiliaryDataInfoSetData(
    _ raw: UnsafeMutableRawPointer?,
    _ bytes: UnsafePointer<UInt8>?,
    _ length: Int
) {
    guard let raw, let bytes, length >= 0 else {
        return
    }
    let info = unretainedBox(raw, as: AuxiliaryDataInfoBox.self).value
    info.data = Data(bytes: bytes, count: length)
}

@_cdecl("imageio_auxiliary_data_info_set_description")
public func imageioAuxiliaryDataInfoSetDescription(
    _ raw: UnsafeMutableRawPointer?,
    _ propertiesRaw: UnsafeMutableRawPointer?
) {
    guard let raw, let propertiesRaw else {
        return
    }
    let info = unretainedBox(raw, as: AuxiliaryDataInfoBox.self).value
    let properties = unretainedBox(propertiesRaw, as: NSDictionary.self).value
    info.description = NSDictionary(dictionary: properties)
}

@_cdecl("imageio_auxiliary_data_info_set_metadata")
public func imageioAuxiliaryDataInfoSetMetadata(
    _ raw: UnsafeMutableRawPointer?,
    _ metadataRaw: UnsafeMutableRawPointer?
) {
    guard let raw, let metadataRaw else {
        return
    }
    let info = unretainedBox(raw, as: AuxiliaryDataInfoBox.self).value
    info.metadata = unretainedBox(metadataRaw, as: CGImageMetadata.self).value
}

@_cdecl("imageio_auxiliary_data_info_copy_data")
public func imageioAuxiliaryDataInfoCopyData(_ raw: UnsafeMutableRawPointer?) -> UnsafeMutableRawPointer? {
    guard let raw else {
        return nil
    }
    let info = unretainedBox(raw, as: AuxiliaryDataInfoBox.self).value
    return retainBox(info.data)
}

@_cdecl("imageio_auxiliary_data_info_copy_description")
public func imageioAuxiliaryDataInfoCopyDescription(_ raw: UnsafeMutableRawPointer?) -> UnsafeMutableRawPointer? {
    guard let raw else {
        return nil
    }
    let info = unretainedBox(raw, as: AuxiliaryDataInfoBox.self).value
    guard let description = info.description else {
        return nil
    }
    return retainBox(NSDictionary(dictionary: description))
}

@_cdecl("imageio_auxiliary_data_info_copy_metadata")
public func imageioAuxiliaryDataInfoCopyMetadata(_ raw: UnsafeMutableRawPointer?) -> UnsafeMutableRawPointer? {
    guard let raw else {
        return nil
    }
    let info = unretainedBox(raw, as: AuxiliaryDataInfoBox.self).value
    guard let metadata = info.metadata else {
        return nil
    }
    return retainBox(metadata)
}

@_cdecl("imageio_auxiliary_data_info_has_color_space")
public func imageioAuxiliaryDataInfoHasColorSpace(_ raw: UnsafeMutableRawPointer?) -> Bool {
    guard let raw else {
        return false
    }
    let info = unretainedBox(raw, as: AuxiliaryDataInfoBox.self).value
    return info.hasColorSpace
}
