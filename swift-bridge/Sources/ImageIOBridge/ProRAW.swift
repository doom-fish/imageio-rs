import Foundation
import ImageIO
import UniformTypeIdentifiers

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
    guard let dictionary = dngDictionary(from: raw),
          let profileName = dictionary[kCGImagePropertyDNGProfileName] as? String
    else {
        return nil
    }
    return retainBox(profileName)
}

@_cdecl("imageio_proraw_copy_unique_camera_model")
public func imageioProrawCopyUniqueCameraModel(_ raw: UnsafeMutableRawPointer?) -> UnsafeMutableRawPointer? {
    if let rawDictionary = rawDictionary(from: raw),
       let model = rawDictionary[kCGImagePropertyDNGUniqueCameraModel] as? String
    {
        return retainBox(model)
    }
    if let dngDictionary = dngDictionary(from: raw),
       let model = dngDictionary[kCGImagePropertyDNGUniqueCameraModel] as? String
    {
        return retainBox(model)
    }
    return nil
}

@_cdecl("imageio_type_identifier_conforms_to_dng")
public func imageioTypeIdentifierConformsToDng(_ identifier: UnsafePointer<CChar>?) -> Bool {
    guard let identifier,
          let candidate = UTType(String(cString: identifier)),
          let dng = UTType("com.adobe.raw-image")
    else {
        return false
    }
    return candidate == dng || candidate.conforms(to: dng)
}
