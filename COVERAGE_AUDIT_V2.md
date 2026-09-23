# imageio-rs coverage audit v2 (ImageIO functions, MacOSX26.5.sdk)

SDK_PUBLIC_FUNCTIONS: 58
SAFE_WRAPPER: 51
NOT_WRAPPED: 7
COVERAGE_PCT: 87.9

An earlier revision of this file claimed safe wrappers for 846 of 847 ImageIO symbols. Most of the wrapper names it listed do not exist in the crate (for example `ImageSource::with_url`, `ImageSource::with_data_provider`, `ImageDestination::with_data_consumer`, `ImageDestination::with_io` and an `imageio::keys` module), and it listed a function, `CGImageDestinationCreateWithIO`, that ImageIO does not declare. This revision replaces it.

## Methodology

Every function declared with `IMAGEIO_EXTERN` in the ImageIO headers of the installed MacOSX26.5.sdk is listed below. A function counts as wrapped when a public safe Rust item reaches it through the Swift bridge (traced from the `@_cdecl` export that calls it to the Rust functions that call that export). The `raw-ffi` feature declares the C functions directly in `imageio::ffi`, but those are unsafe and are not counted here.

Property keys and other constants are not wrapped one by one. The safe API takes the key strings (for example `ImageProperties::i64("PixelWidth")`) and offers typed helpers for the APNG, HEIF, ProRAW and color keys; the constants themselves are declared in the unsafe `imageio::ffi` module.

## Functions

| Function | Header | Safe wrapper |
| --- | --- | --- |
| `CGAnimateImageAtURLWithBlock` | `CGImageAnimation.h` | `animate_image` |
| `CGAnimateImageDataWithBlock` | `CGImageAnimation.h` | `animate_image_from_bytes` |
| `CGImageDestinationAddAuxiliaryDataInfo` | `CGImageDestination.h` | `ImageDestination::add_auxiliary_data_info` |
| `CGImageDestinationAddImage` | `CGImageDestination.h` | `ImageDestination::add_cg_image`, `ImageDestination::add_image` |
| `CGImageDestinationAddImageAndMetadata` | `CGImageDestination.h` | `ImageDestination::add_image_with_metadata` |
| `CGImageDestinationAddImageFromSource` | `CGImageDestination.h` | `ImageDestination::add_image_from_source`, `convert_format` |
| `CGImageDestinationCopyImageSource` | `CGImageDestination.h` | `ImageDestination::copy_image_source`, `copy_image_source` |
| `CGImageDestinationCopyTypeIdentifiers` | `CGImageDestination.h` | `ImageDestination::type_identifiers` |
| `CGImageDestinationCreateWithData` | `CGImageDestination.h` | `ImageDestination::to_data`, `encode_bgra_to_bytes` |
| `CGImageDestinationCreateWithDataConsumer` | `CGImageDestination.h` | `ImageDestination::to_writer` |
| `CGImageDestinationCreateWithURL` | `CGImageDestination.h` | `ImageDestination::to_path` |
| `CGImageDestinationFinalize` | `CGImageDestination.h` | `ImageDestination::finalize` |
| `CGImageDestinationGetTypeID` | `CGImageDestination.h` | not wrapped (raw `ffi` only) |
| `CGImageDestinationSetProperties` | `CGImageDestination.h` | `ImageDestination::set_properties` |
| `CGImageMetadataCopyStringValueWithPath` | `CGImageMetadata.h` | `Metadata::string_value_with_path` |
| `CGImageMetadataCopyTagMatchingImageProperty` | `CGImageMetadata.h` | not wrapped (raw `ffi` only) |
| `CGImageMetadataCopyTagWithPath` | `CGImageMetadata.h` | `Metadata::tag_with_path` |
| `CGImageMetadataCopyTags` | `CGImageMetadata.h` | `Metadata::tags` |
| `CGImageMetadataCreateFromXMPData` | `CGImageMetadata.h` | `Metadata::from_xmp_data` |
| `CGImageMetadataCreateMutable` | `CGImageMetadata.h` | `MutableMetadata::new` |
| `CGImageMetadataCreateMutableCopy` | `CGImageMetadata.h` | `MutableMetadata::clone`, `MutableMetadata::into_metadata`, `Metadata::enumerate_tags_with_options` |
| `CGImageMetadataCreateXMPData` | `CGImageMetadata.h` | `Metadata::create_xmp_data` |
| `CGImageMetadataEnumerateTagsUsingBlock` | `CGImageMetadata.h` | `Metadata::enumerate_tags_with_options` |
| `CGImageMetadataGetTypeID` | `CGImageMetadata.h` | not wrapped (raw `ffi` only) |
| `CGImageMetadataRegisterNamespaceForPrefix` | `CGImageMetadata.h` | `MutableMetadata::register_namespace_for_prefix` |
| `CGImageMetadataRemoveTagWithPath` | `CGImageMetadata.h` | `MutableMetadata::remove_tag_with_path` |
| `CGImageMetadataSetTagWithPath` | `CGImageMetadata.h` | `MutableMetadata::set_tag_with_path` |
| `CGImageMetadataSetValueMatchingImageProperty` | `CGImageMetadata.h` | not wrapped (raw `ffi` only) |
| `CGImageMetadataSetValueWithPath` | `CGImageMetadata.h` | `MutableMetadata::set_string_value_with_path` |
| `CGImageMetadataTagCopyName` | `CGImageMetadata.h` | `MetadataTag::name` |
| `CGImageMetadataTagCopyNamespace` | `CGImageMetadata.h` | `MetadataTag::namespace` |
| `CGImageMetadataTagCopyPrefix` | `CGImageMetadata.h` | `MetadataTag::prefix` |
| `CGImageMetadataTagCopyQualifiers` | `CGImageMetadata.h` | `MetadataTag::qualifiers` |
| `CGImageMetadataTagCopyValue` | `CGImageMetadata.h` | `MetadataTag::string_value` |
| `CGImageMetadataTagCreate` | `CGImageMetadata.h` | `MetadataTag::new_string` |
| `CGImageMetadataTagGetType` | `CGImageMetadata.h` | `MetadataTag::tag_type` |
| `CGImageMetadataTagGetTypeID` | `CGImageMetadata.h` | not wrapped (raw `ffi` only) |
| `CGImageSourceCopyAuxiliaryDataInfoAtIndex` | `CGImageSource.h` | `ImageSource::auxiliary_data_at_index` |
| `CGImageSourceCopyMetadataAtIndex` | `CGImageSource.h` | `ImageSource::metadata_at_index` |
| `CGImageSourceCopyProperties` | `CGImageSource.h` | `ImageSource::copy_properties` |
| `CGImageSourceCopyPropertiesAtIndex` | `CGImageSource.h` | `ImageSource::properties_at_index`, `read_metadata` |
| `CGImageSourceCopyTypeIdentifiers` | `CGImageSource.h` | `ImageSource::type_identifiers` |
| `CGImageSourceCreateImageAtIndex` | `CGImageSource.h` | `ImageSource::decode_image_at_index`, `decode_bgra`, `decode_bgra_from_bytes` |
| `CGImageSourceCreateIncremental` | `CGImageSource.h` | `ImageSource::incremental`, `ImageSource::incremental_with_options` |
| `CGImageSourceCreateThumbnailAtIndex` | `CGImageSource.h` | `create_thumbnail` |
| `CGImageSourceCreateWithData` | `CGImageSource.h` | `ImageSource::from_bytes`, `ImageSource::from_bytes_with_options`, `animate_image_from_bytes` |
| `CGImageSourceCreateWithDataProvider` | `CGImageSource.h` | `ImageSource::from_data_provider` |
| `CGImageSourceCreateWithURL` | `CGImageSource.h` | `ImageSource::from_path`, `ImageSource::from_path_with_options`, `animate_image` |
| `CGImageSourceGetCount` | `CGImageSource.h` | `ImageSource::frame_count` |
| `CGImageSourceGetPrimaryImageIndex` | `CGImageSource.h` | `ImageSource::primary_image_index` |
| `CGImageSourceGetStatus` | `CGImageSource.h` | `ImageSource::status` |
| `CGImageSourceGetStatusAtIndex` | `CGImageSource.h` | `ImageSource::status_at_index` |
| `CGImageSourceGetType` | `CGImageSource.h` | `ImageSource::source_type` |
| `CGImageSourceGetTypeID` | `CGImageSource.h` | not wrapped (raw `ffi` only) |
| `CGImageSourceRemoveCacheAtIndex` | `CGImageSource.h` | `ImageSource::remove_cache_at_index` |
| `CGImageSourceSetAllowableTypes` | `CGImageSource.h` | not wrapped (raw `ffi` only) |
| `CGImageSourceUpdateData` | `CGImageSource.h` | `ImageSource::update_data` |
| `CGImageSourceUpdateDataProvider` | `CGImageSource.h` | `ImageSource::update_data_provider` |

## Not wrapped

The `*GetTypeID` functions, `CGImageSourceSetAllowableTypes` (a process-wide switch that restricts which formats ImageIO may decode), and `CGImageMetadataCopyTagMatchingImageProperty` / `CGImageMetadataSetValueMatchingImageProperty` have no safe wrapper; they are declared in `imageio::ffi` behind the `raw-ffi` feature.
