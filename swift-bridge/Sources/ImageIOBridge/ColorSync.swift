import Foundation
import ImageIO

@_cdecl("imageio_properties_copy_profile_name")
public func imageioPropertiesCopyProfileName(_ raw: UnsafeMutableRawPointer?) -> UnsafeMutableRawPointer? {
    guard let raw else {
        return nil
    }
    let properties = unretainedBox(raw, as: NSDictionary.self).value
    guard let profileName = properties[kCGImagePropertyProfileName] as? String else {
        return nil
    }
    return retainBox(profileName)
}

@_cdecl("imageio_source_copy_profile_name_at_index")
public func imageioSourceCopyProfileNameAtIndex(
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
    guard let profileName = properties[kCGImagePropertyProfileName] as? String else {
        return nil
    }
    return retainBox(profileName)
}
