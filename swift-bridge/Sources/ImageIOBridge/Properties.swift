import Foundation
import ImageIO

@_cdecl("imageio_mutable_properties_create")
public func imageioMutablePropertiesCreate() -> UnsafeMutableRawPointer? {
    retainBox(NSMutableDictionary())
}

@_cdecl("imageio_mutable_properties_freeze")
public func imageioMutablePropertiesFreeze(_ raw: UnsafeMutableRawPointer?) -> UnsafeMutableRawPointer? {
    guard let raw else {
        return nil
    }
    let dictionary = unretainedBox(raw, as: NSMutableDictionary.self).value
    return retainBox(NSDictionary(dictionary: dictionary))
}

@_cdecl("imageio_mutable_properties_set_string")
public func imageioMutablePropertiesSetString(
    _ raw: UnsafeMutableRawPointer?,
    _ key: UnsafePointer<CChar>?,
    _ value: UnsafePointer<CChar>?
) {
    guard let raw, let key, let value else {
        return
    }
    let dictionary = unretainedBox(raw, as: NSMutableDictionary.self).value
    dictionary[String(cString: key)] = String(cString: value)
}

@_cdecl("imageio_mutable_properties_set_i64")
public func imageioMutablePropertiesSetI64(
    _ raw: UnsafeMutableRawPointer?,
    _ key: UnsafePointer<CChar>?,
    _ value: Int64
) {
    guard let raw, let key else {
        return
    }
    let dictionary = unretainedBox(raw, as: NSMutableDictionary.self).value
    dictionary[String(cString: key)] = NSNumber(value: value)
}

@_cdecl("imageio_mutable_properties_set_f64")
public func imageioMutablePropertiesSetF64(
    _ raw: UnsafeMutableRawPointer?,
    _ key: UnsafePointer<CChar>?,
    _ value: Double
) {
    guard let raw, let key else {
        return
    }
    let dictionary = unretainedBox(raw, as: NSMutableDictionary.self).value
    dictionary[String(cString: key)] = NSNumber(value: value)
}

@_cdecl("imageio_mutable_properties_set_bool")
public func imageioMutablePropertiesSetBool(
    _ raw: UnsafeMutableRawPointer?,
    _ key: UnsafePointer<CChar>?,
    _ value: Bool
) {
    guard let raw, let key else {
        return
    }
    let dictionary = unretainedBox(raw, as: NSMutableDictionary.self).value
    dictionary[String(cString: key)] = NSNumber(value: value)
}

@_cdecl("imageio_mutable_properties_set_dictionary")
public func imageioMutablePropertiesSetDictionary(
    _ raw: UnsafeMutableRawPointer?,
    _ key: UnsafePointer<CChar>?,
    _ nested: UnsafeMutableRawPointer?
) {
    guard let raw, let key, let nested else {
        return
    }
    let dictionary = unretainedBox(raw, as: NSMutableDictionary.self).value
    let nestedDictionary = unretainedBox(nested, as: NSDictionary.self).value
    dictionary[String(cString: key)] = NSDictionary(dictionary: nestedDictionary)
}

@_cdecl("imageio_properties_copy_keys")
public func imageioPropertiesCopyKeys(_ raw: UnsafeMutableRawPointer?) -> UnsafeMutableRawPointer? {
    guard let raw else {
        return nil
    }
    let dictionary = unretainedBox(raw, as: NSDictionary.self).value
    let keys = dictionary.allKeys.compactMap { $0 as? String }.sorted()
    return retainBox(keys)
}

@_cdecl("imageio_properties_has_key")
public func imageioPropertiesHasKey(
    _ raw: UnsafeMutableRawPointer?,
    _ key: UnsafePointer<CChar>?
) -> Bool {
    guard let raw, let key else {
        return false
    }
    let dictionary = unretainedBox(raw, as: NSDictionary.self).value
    return dictionary[String(cString: key)] != nil
}

@_cdecl("imageio_properties_copy_string")
public func imageioPropertiesCopyString(
    _ raw: UnsafeMutableRawPointer?,
    _ key: UnsafePointer<CChar>?
) -> UnsafeMutableRawPointer? {
    guard let raw, let key else {
        return nil
    }
    let dictionary = unretainedBox(raw, as: NSDictionary.self).value
    guard let value = dictionary[String(cString: key)] as? String else {
        return nil
    }
    return retainBox(value)
}

@_cdecl("imageio_properties_get_i64")
public func imageioPropertiesGetI64(
    _ raw: UnsafeMutableRawPointer?,
    _ key: UnsafePointer<CChar>?,
    _ outValue: UnsafeMutablePointer<Int64>?
) -> Bool {
    guard let raw, let key, let outValue else {
        return false
    }
    let dictionary = unretainedBox(raw, as: NSDictionary.self).value
    guard let number = dictionary[String(cString: key)] as? NSNumber else {
        return false
    }
    outValue.pointee = number.int64Value
    return true
}

@_cdecl("imageio_properties_get_f64")
public func imageioPropertiesGetF64(
    _ raw: UnsafeMutableRawPointer?,
    _ key: UnsafePointer<CChar>?,
    _ outValue: UnsafeMutablePointer<Double>?
) -> Bool {
    guard let raw, let key, let outValue else {
        return false
    }
    let dictionary = unretainedBox(raw, as: NSDictionary.self).value
    guard let number = dictionary[String(cString: key)] as? NSNumber else {
        return false
    }
    outValue.pointee = number.doubleValue
    return true
}

@_cdecl("imageio_properties_get_bool")
public func imageioPropertiesGetBool(
    _ raw: UnsafeMutableRawPointer?,
    _ key: UnsafePointer<CChar>?,
    _ outValue: UnsafeMutablePointer<Bool>?
) -> Bool {
    guard let raw, let key, let outValue else {
        return false
    }
    let dictionary = unretainedBox(raw, as: NSDictionary.self).value
    guard let number = dictionary[String(cString: key)] as? NSNumber else {
        return false
    }
    outValue.pointee = number.boolValue
    return true
}

@_cdecl("imageio_properties_copy_dictionary")
public func imageioPropertiesCopyDictionary(
    _ raw: UnsafeMutableRawPointer?,
    _ key: UnsafePointer<CChar>?
) -> UnsafeMutableRawPointer? {
    guard let raw, let key else {
        return nil
    }
    let dictionary = unretainedBox(raw, as: NSDictionary.self).value
    guard let nested = dictionary[String(cString: key)] as? NSDictionary else {
        return nil
    }
    return retainBox(NSDictionary(dictionary: nested))
}
