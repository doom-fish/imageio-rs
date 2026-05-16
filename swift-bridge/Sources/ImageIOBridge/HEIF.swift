import Foundation
import ImageIO

private func heifDictionary(from raw: UnsafeMutableRawPointer?) -> NSDictionary? {
    guard let raw else {
        return nil
    }
    let properties = unretainedBox(raw, as: NSDictionary.self).value
    return properties[kCGImagePropertyHEIFDictionary] as? NSDictionary
}

private func heicsDictionary(from raw: UnsafeMutableRawPointer?) -> NSDictionary? {
    guard let raw else {
        return nil
    }
    let properties = unretainedBox(raw, as: NSDictionary.self).value
    return properties[kCGImagePropertyHEICSDictionary] as? NSDictionary
}

@_cdecl("imageio_heif_copy_dictionary")
public func imageioHeifCopyDictionary(_ raw: UnsafeMutableRawPointer?) -> UnsafeMutableRawPointer? {
    guard let dictionary = heifDictionary(from: raw) else {
        return nil
    }
    return retainBox(NSDictionary(dictionary: dictionary))
}

@_cdecl("imageio_heics_copy_dictionary")
public func imageioHeicsCopyDictionary(_ raw: UnsafeMutableRawPointer?) -> UnsafeMutableRawPointer? {
    guard let dictionary = heicsDictionary(from: raw) else {
        return nil
    }
    return retainBox(NSDictionary(dictionary: dictionary))
}

@_cdecl("imageio_heif_get_primary")
public func imageioHeifGetPrimary(_ raw: UnsafeMutableRawPointer?, _ outValue: UnsafeMutablePointer<Bool>?) -> Bool {
    guard let raw, let outValue else {
        return false
    }
    let properties = unretainedBox(raw, as: NSDictionary.self).value
    guard let number = properties[kCGImagePropertyPrimaryImage] as? NSNumber else {
        return false
    }
    outValue.pointee = number.boolValue
    return true
}

@_cdecl("imageio_heics_get_loop_count")
public func imageioHeicsGetLoopCount(_ raw: UnsafeMutableRawPointer?, _ outValue: UnsafeMutablePointer<Int64>?) -> Bool {
    guard let dictionary = heicsDictionary(from: raw), let outValue, let number = dictionary[kCGImagePropertyHEICSLoopCount] as? NSNumber else {
        return false
    }
    outValue.pointee = number.int64Value
    return true
}

@_cdecl("imageio_heics_get_delay_time")
public func imageioHeicsGetDelayTime(_ raw: UnsafeMutableRawPointer?, _ outValue: UnsafeMutablePointer<Double>?) -> Bool {
    guard let dictionary = heicsDictionary(from: raw), let outValue, let number = dictionary[kCGImagePropertyHEICSDelayTime] as? NSNumber else {
        return false
    }
    outValue.pointee = number.doubleValue
    return true
}

@_cdecl("imageio_heics_get_unclamped_delay_time")
public func imageioHeicsGetUnclampedDelayTime(_ raw: UnsafeMutableRawPointer?, _ outValue: UnsafeMutablePointer<Double>?) -> Bool {
    guard let dictionary = heicsDictionary(from: raw), let outValue, let number = dictionary[kCGImagePropertyHEICSUnclampedDelayTime] as? NSNumber else {
        return false
    }
    outValue.pointee = number.doubleValue
    return true
}

@_cdecl("imageio_heics_get_canvas_width")
public func imageioHeicsGetCanvasWidth(_ raw: UnsafeMutableRawPointer?, _ outValue: UnsafeMutablePointer<Int64>?) -> Bool {
    guard let dictionary = heicsDictionary(from: raw), let outValue, let number = dictionary[kCGImagePropertyHEICSCanvasPixelWidth] as? NSNumber else {
        return false
    }
    outValue.pointee = number.int64Value
    return true
}

@_cdecl("imageio_heics_get_canvas_height")
public func imageioHeicsGetCanvasHeight(_ raw: UnsafeMutableRawPointer?, _ outValue: UnsafeMutablePointer<Int64>?) -> Bool {
    guard let dictionary = heicsDictionary(from: raw), let outValue, let number = dictionary[kCGImagePropertyHEICSCanvasPixelHeight] as? NSNumber else {
        return false
    }
    outValue.pointee = number.int64Value
    return true
}

@_cdecl("imageio_heics_get_frame_info_count")
public func imageioHeicsGetFrameInfoCount(_ raw: UnsafeMutableRawPointer?) -> Int {
    guard let dictionary = heicsDictionary(from: raw), let frames = dictionary[kCGImagePropertyHEICSFrameInfoArray] as? [Any] else {
        return 0
    }
    return frames.count
}
