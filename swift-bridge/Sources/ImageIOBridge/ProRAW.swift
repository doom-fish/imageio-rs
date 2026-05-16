import Foundation
import ImageIO

private func rawDictionary(from raw: UnsafeMutableRawPointer?) -> NSDictionary? {
    guard let raw else {
        return nil
    }
    let properties = unretainedBox(raw, as: NSDictionary.self).value
    return properties[kCGImagePropertyRawDictionary] as? NSDictionary
}

private func dngDictionary(from raw: UnsafeMutableRawPointer?) -> NSDictionary? {
    guard let raw else {
        return nil
    }
    let properties = unretainedBox(raw, as: NSDictionary.self).value
    return properties[kCGImagePropertyDNGDictionary] as? NSDictionary
}

@_cdecl("imageio_proraw_copy_raw_dictionary")
public func imageioProrawCopyRawDictionary(_ raw: UnsafeMutableRawPointer?) -> UnsafeMutableRawPointer? {
    guard let dictionary = rawDictionary(from: raw) else {
        return nil
    }
    return retainBox(NSDictionary(dictionary: dictionary))
}

@_cdecl("imageio_proraw_copy_dng_dictionary")
public func imageioProrawCopyDngDictionary(_ raw: UnsafeMutableRawPointer?) -> UnsafeMutableRawPointer? {
    guard let dictionary = dngDictionary(from: raw) else {
        return nil
    }
    return retainBox(NSDictionary(dictionary: dictionary))
}

@_cdecl("imageio_proraw_copy_profile_name")
public func imageioProrawCopyProfileName(_ raw: UnsafeMutableRawPointer?) -> UnsafeMutableRawPointer? {
    guard let dictionary = dngDictionary(from: raw), let profileName = dictionary["ProfileName"] as? String else {
        return nil
    }
    return retainBox(profileName)
}

@_cdecl("imageio_proraw_copy_unique_camera_model")
public func imageioProrawCopyUniqueCameraModel(_ raw: UnsafeMutableRawPointer?) -> UnsafeMutableRawPointer? {
    if let rawDictionary = rawDictionary(from: raw), let model = rawDictionary["UniqueCameraModel"] as? String {
        return retainBox(model)
    }
    if let dngDictionary = dngDictionary(from: raw), let model = dngDictionary["UniqueCameraModel"] as? String {
        return retainBox(model)
    }
    return nil
}
