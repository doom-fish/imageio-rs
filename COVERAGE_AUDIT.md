# imageio coverage audit (vs MacOSX26.2.sdk)

SDK_PUBLIC_SYMBOLS: 847
VERIFIED: 844
GAPS: 2
EXEMPT: 1
COVERAGE_PCT: 99.76%

## Audit scope

- SDK root: `/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.2.sdk`
- Public headers with symbols: `CGImageSource.h`, `CGImageDestination.h`, `CGImageAnimation.h`, `CGImageMetadata.h`, `CGImageProperties.h` (`ImageIO.h` and `ImageIOBase.h` only add imports/macros).
- Crate surface considered: the default safe Rust API (`source`, `destination`, `properties`, `metadata`, `auxiliary_data`, `color_sync`, `animated_png`, `heif`, `proraw`, `thumbnail`) plus `imageio::ffi::*` when `raw-ffi` is enabled.
- Deprecated SDK declarations are counted as **EXEMPT** per the audit instructions, even if the crate already exposes them.

## Safe public surface

| Area | Public API |
| --- | --- |
| Source | `ImageSource`, `SourceStatus`, `read_metadata`, `decode_bgra*`, `create_thumbnail` |
| Destination | `ImageDestination`, `encode_bgra_to_bytes`, `convert_format`, `copy_image_source` |
| Properties | `ImageProperties`, `MutableProperties` |
| Metadata | `Metadata`, `MutableMetadata`, `MetadataTag`, `MetadataType` |
| AuxiliaryData | `AuxiliaryDataInfo`, `AuxiliaryDataType` |
| ColorSync | `DecodeRequest`, `EncodeRequest`, `profile_name`, `source_profile_name`, and encode/decode request helpers |
| Animated image | `animate_image`, `animate_image_from_bytes`, `AnimatedPngBuilder` |
| HEIF / HEICS | `HeifBuilder`, `HeifProperties` |
| ProRAW | `ProRawBuilder`, `ProRawProperties` |
| Raw C surface | `imageio::ffi::*` behind the `raw-ffi` feature |

## 🟢 VERIFIED

| Symbol | Kind | Header | Wrapped by |
| --- | --- | --- | --- |
| `CGImageSourceRef` | type | `CGImageSource.h` | ImageSource<br>`ffi::CGImageSourceRef` |
| `CGImageSourceStatus` | type | `CGImageSource.h` | SourceStatus<br>`ffi::CGImageSourceStatus` |
| `CGImageSourceStatus` | enum type | `CGImageSource.h` | SourceStatus<br>`ffi::CGImageSourceStatus` |
| `kCGImageStatusComplete` | enum case | `CGImageSource.h` | source / image / thumbnail helpers<br>`ffi::kCGImageStatusComplete` |
| `kCGImageStatusIncomplete` | enum case | `CGImageSource.h` | source / image / thumbnail helpers<br>`ffi::kCGImageStatusIncomplete` |
| `kCGImageStatusInvalidData` | enum case | `CGImageSource.h` | source / image / thumbnail helpers<br>`ffi::kCGImageStatusInvalidData` |
| `kCGImageStatusReadingHeader` | enum case | `CGImageSource.h` | source / image / thumbnail helpers<br>`ffi::kCGImageStatusReadingHeader` |
| `kCGImageStatusUnexpectedEOF` | enum case | `CGImageSource.h` | source / image / thumbnail helpers<br>`ffi::kCGImageStatusUnexpectedEOF` |
| `kCGImageStatusUnknownType` | enum case | `CGImageSource.h` | source / image / thumbnail helpers<br>`ffi::kCGImageStatusUnknownType` |
| `CGImageSourceCopyAuxiliaryDataInfoAtIndex` | function | `CGImageSource.h` | source / image / thumbnail helpers<br>`ffi::CGImageSourceCopyAuxiliaryDataInfoAtIndex` |
| `CGImageSourceCopyMetadataAtIndex` | function | `CGImageSource.h` | ImageSource::metadata_at_index / read_metadata<br>`ffi::CGImageSourceCopyMetadataAtIndex` |
| `CGImageSourceCopyProperties` | function | `CGImageSource.h` | source / image / thumbnail helpers<br>`ffi::CGImageSourceCopyProperties` |
| `CGImageSourceCopyPropertiesAtIndex` | function | `CGImageSource.h` | source / image / thumbnail helpers<br>`ffi::CGImageSourceCopyPropertiesAtIndex` |
| `CGImageSourceCopyTypeIdentifiers` | function | `CGImageSource.h` | source / image / thumbnail helpers<br>`ffi::CGImageSourceCopyTypeIdentifiers` |
| `CGImageSourceCreateImageAtIndex` | function | `CGImageSource.h` | ImageSource::decode_image_at_index / decode_bgra*<br>`ffi::CGImageSourceCreateImageAtIndex` |
| `CGImageSourceCreateIncremental` | function | `CGImageSource.h` | source / image / thumbnail helpers<br>`ffi::CGImageSourceCreateIncremental` |
| `CGImageSourceCreateThumbnailAtIndex` | function | `CGImageSource.h` | create_thumbnail<br>`ffi::CGImageSourceCreateThumbnailAtIndex` |
| `CGImageSourceCreateWithData` | function | `CGImageSource.h` | source / image / thumbnail helpers<br>`ffi::CGImageSourceCreateWithData` |
| `CGImageSourceCreateWithDataProvider` | function | `CGImageSource.h` | source / image / thumbnail helpers<br>`ffi::CGImageSourceCreateWithDataProvider` |
| `CGImageSourceCreateWithURL` | function | `CGImageSource.h` | source / image / thumbnail helpers<br>`ffi::CGImageSourceCreateWithURL` |
| `CGImageSourceGetCount` | function | `CGImageSource.h` | source / image / thumbnail helpers<br>`ffi::CGImageSourceGetCount` |
| `CGImageSourceGetPrimaryImageIndex` | function | `CGImageSource.h` | source / image / thumbnail helpers<br>`ffi::CGImageSourceGetPrimaryImageIndex` |
| `CGImageSourceGetStatus` | function | `CGImageSource.h` | source / image / thumbnail helpers<br>`ffi::CGImageSourceGetStatus` |
| `CGImageSourceGetStatusAtIndex` | function | `CGImageSource.h` | source / image / thumbnail helpers<br>`ffi::CGImageSourceGetStatusAtIndex` |
| `CGImageSourceGetType` | function | `CGImageSource.h` | source / image / thumbnail helpers<br>`ffi::CGImageSourceGetType` |
| `CGImageSourceGetTypeID` | function | `CGImageSource.h` | source / image / thumbnail helpers<br>`ffi::CGImageSourceGetTypeID` |
| `CGImageSourceRemoveCacheAtIndex` | function | `CGImageSource.h` | source / image / thumbnail helpers<br>`ffi::CGImageSourceRemoveCacheAtIndex` |
| `CGImageSourceSetAllowableTypes` | function | `CGImageSource.h` | source / image / thumbnail helpers<br>`ffi::CGImageSourceSetAllowableTypes` |
| `CGImageSourceUpdateData` | function | `CGImageSource.h` | source / image / thumbnail helpers<br>`ffi::CGImageSourceUpdateData` |
| `CGImageSourceUpdateDataProvider` | function | `CGImageSource.h` | source / image / thumbnail helpers<br>`ffi::CGImageSourceUpdateDataProvider` |
| `kCGComputeHDRStats` | constant | `CGImageSource.h` | source / image / thumbnail helpers<br>`ffi::kCGComputeHDRStats` |
| `kCGImageSourceCreateThumbnailFromImageAlways` | constant | `CGImageSource.h` | source / image / thumbnail helpers<br>`ffi::kCGImageSourceCreateThumbnailFromImageAlways` |
| `kCGImageSourceCreateThumbnailFromImageIfAbsent` | constant | `CGImageSource.h` | source / image / thumbnail helpers<br>`ffi::kCGImageSourceCreateThumbnailFromImageIfAbsent` |
| `kCGImageSourceCreateThumbnailWithTransform` | constant | `CGImageSource.h` | source / image / thumbnail helpers<br>`ffi::kCGImageSourceCreateThumbnailWithTransform` |
| `kCGImageSourceDecodeRequest` | constant | `CGImageSource.h` | source / image / thumbnail helpers<br>`ffi::kCGImageSourceDecodeRequest` |
| `kCGImageSourceDecodeRequestOptions` | constant | `CGImageSource.h` | source / image / thumbnail helpers<br>`ffi::kCGImageSourceDecodeRequestOptions` |
| `kCGImageSourceDecodeToHDR` | constant | `CGImageSource.h` | source / image / thumbnail helpers<br>`ffi::kCGImageSourceDecodeToHDR` |
| `kCGImageSourceDecodeToSDR` | constant | `CGImageSource.h` | source / image / thumbnail helpers<br>`ffi::kCGImageSourceDecodeToSDR` |
| `kCGImageSourceGenerateImageSpecificLumaScaling` | constant | `CGImageSource.h` | source / image / thumbnail helpers<br>`ffi::kCGImageSourceGenerateImageSpecificLumaScaling` |
| `kCGImageSourceShouldAllowFloat` | constant | `CGImageSource.h` | source / image / thumbnail helpers<br>`ffi::kCGImageSourceShouldAllowFloat` |
| `kCGImageSourceShouldCache` | constant | `CGImageSource.h` | source / image / thumbnail helpers<br>`ffi::kCGImageSourceShouldCache` |
| `kCGImageSourceShouldCacheImmediately` | constant | `CGImageSource.h` | source / image / thumbnail helpers<br>`ffi::kCGImageSourceShouldCacheImmediately` |
| `kCGImageSourceSubsampleFactor` | constant | `CGImageSource.h` | source / image / thumbnail helpers<br>`ffi::kCGImageSourceSubsampleFactor` |
| `kCGImageSourceThumbnailMaxPixelSize` | constant | `CGImageSource.h` | source / image / thumbnail helpers<br>`ffi::kCGImageSourceThumbnailMaxPixelSize` |
| `kCGImageSourceTypeIdentifierHint` | constant | `CGImageSource.h` | source / image / thumbnail helpers<br>`ffi::kCGImageSourceTypeIdentifierHint` |
| `CGImageDestinationRef` | type | `CGImageDestination.h` | ImageDestination<br>`ffi::CGImageDestinationRef` |
| `CGImageDestinationAddAuxiliaryDataInfo` | function | `CGImageDestination.h` | destination / animated_png / heif / proraw helpers<br>`ffi::CGImageDestinationAddAuxiliaryDataInfo` |
| `CGImageDestinationAddImage` | function | `CGImageDestination.h` | destination / animated_png / heif / proraw helpers<br>`ffi::CGImageDestinationAddImage` |
| `CGImageDestinationAddImageAndMetadata` | function | `CGImageDestination.h` | ImageDestination::add_image_with_metadata<br>`ffi::CGImageDestinationAddImageAndMetadata` |
| `CGImageDestinationAddImageFromSource` | function | `CGImageDestination.h` | destination / animated_png / heif / proraw helpers<br>`ffi::CGImageDestinationAddImageFromSource` |
| `CGImageDestinationCopyImageSource` | function | `CGImageDestination.h` | ImageDestination::copy_image_source / copy_image_source<br>`ffi::CGImageDestinationCopyImageSource` |
| `CGImageDestinationCopyTypeIdentifiers` | function | `CGImageDestination.h` | destination / animated_png / heif / proraw helpers<br>`ffi::CGImageDestinationCopyTypeIdentifiers` |
| `CGImageDestinationCreateWithData` | function | `CGImageDestination.h` | ImageDestination::to_data / encode_bgra_to_bytes<br>`ffi::CGImageDestinationCreateWithData` |
| `CGImageDestinationCreateWithDataConsumer` | function | `CGImageDestination.h` | destination / animated_png / heif / proraw helpers<br>`ffi::CGImageDestinationCreateWithDataConsumer` |
| `CGImageDestinationCreateWithURL` | function | `CGImageDestination.h` | ImageDestination::to_path / convert_format / copy_image_source<br>`ffi::CGImageDestinationCreateWithURL` |
| `CGImageDestinationFinalize` | function | `CGImageDestination.h` | destination / animated_png / heif / proraw helpers<br>`ffi::CGImageDestinationFinalize` |
| `CGImageDestinationGetTypeID` | function | `CGImageDestination.h` | destination / animated_png / heif / proraw helpers<br>`ffi::CGImageDestinationGetTypeID` |
| `CGImageDestinationSetProperties` | function | `CGImageDestination.h` | destination / animated_png / heif / proraw helpers<br>`ffi::CGImageDestinationSetProperties` |
| `kCGImageDestinationBackgroundColor` | constant | `CGImageDestination.h` | destination / animated_png / heif / proraw helpers<br>`ffi::kCGImageDestinationBackgroundColor` |
| `kCGImageDestinationDateTime` | constant | `CGImageDestination.h` | destination / animated_png / heif / proraw helpers<br>`ffi::kCGImageDestinationDateTime` |
| `kCGImageDestinationEmbedThumbnail` | constant | `CGImageDestination.h` | destination / animated_png / heif / proraw helpers<br>`ffi::kCGImageDestinationEmbedThumbnail` |
| `kCGImageDestinationEncodeAlternateColorSpace` | constant | `CGImageDestination.h` | destination / animated_png / heif / proraw helpers<br>`ffi::kCGImageDestinationEncodeAlternateColorSpace` |
| `kCGImageDestinationEncodeBaseColorSpace` | constant | `CGImageDestination.h` | destination / animated_png / heif / proraw helpers<br>`ffi::kCGImageDestinationEncodeBaseColorSpace` |
| `kCGImageDestinationEncodeBaseIsSDR` | constant | `CGImageDestination.h` | destination / animated_png / heif / proraw helpers<br>`ffi::kCGImageDestinationEncodeBaseIsSDR` |
| `kCGImageDestinationEncodeBasePixelFormatRequest` | constant | `CGImageDestination.h` | destination / animated_png / heif / proraw helpers<br>`ffi::kCGImageDestinationEncodeBasePixelFormatRequest` |
| `kCGImageDestinationEncodeGainMapPixelFormatRequest` | constant | `CGImageDestination.h` | destination / animated_png / heif / proraw helpers<br>`ffi::kCGImageDestinationEncodeGainMapPixelFormatRequest` |
| `kCGImageDestinationEncodeGainMapSubsampleFactor` | constant | `CGImageDestination.h` | destination / animated_png / heif / proraw helpers<br>`ffi::kCGImageDestinationEncodeGainMapSubsampleFactor` |
| `kCGImageDestinationEncodeGenerateGainMapWithBaseImage` | constant | `CGImageDestination.h` | destination / animated_png / heif / proraw helpers<br>`ffi::kCGImageDestinationEncodeGenerateGainMapWithBaseImage` |
| `kCGImageDestinationEncodeIsBaseImage` | constant | `CGImageDestination.h` | destination / animated_png / heif / proraw helpers<br>`ffi::kCGImageDestinationEncodeIsBaseImage` |
| `kCGImageDestinationEncodeRequest` | constant | `CGImageDestination.h` | destination / animated_png / heif / proraw helpers<br>`ffi::kCGImageDestinationEncodeRequest` |
| `kCGImageDestinationEncodeRequestOptions` | constant | `CGImageDestination.h` | destination / animated_png / heif / proraw helpers<br>`ffi::kCGImageDestinationEncodeRequestOptions` |
| `kCGImageDestinationEncodeToISOGainmap` | constant | `CGImageDestination.h` | destination / animated_png / heif / proraw helpers<br>`ffi::kCGImageDestinationEncodeToISOGainmap` |
| `kCGImageDestinationEncodeToISOHDR` | constant | `CGImageDestination.h` | destination / animated_png / heif / proraw helpers<br>`ffi::kCGImageDestinationEncodeToISOHDR` |
| `kCGImageDestinationEncodeTonemapMode` | constant | `CGImageDestination.h` | destination / animated_png / heif / proraw helpers<br>`ffi::kCGImageDestinationEncodeTonemapMode` |
| `kCGImageDestinationEncodeToSDR` | constant | `CGImageDestination.h` | destination / animated_png / heif / proraw helpers<br>`ffi::kCGImageDestinationEncodeToSDR` |
| `kCGImageDestinationImageMaxPixelSize` | constant | `CGImageDestination.h` | destination / animated_png / heif / proraw helpers<br>`ffi::kCGImageDestinationImageMaxPixelSize` |
| `kCGImageDestinationLossyCompressionQuality` | constant | `CGImageDestination.h` | destination / animated_png / heif / proraw helpers<br>`ffi::kCGImageDestinationLossyCompressionQuality` |
| `kCGImageDestinationMergeMetadata` | constant | `CGImageDestination.h` | destination / animated_png / heif / proraw helpers<br>`ffi::kCGImageDestinationMergeMetadata` |
| `kCGImageDestinationMetadata` | constant | `CGImageDestination.h` | destination / animated_png / heif / proraw helpers<br>`ffi::kCGImageDestinationMetadata` |
| `kCGImageDestinationOptimizeColorForSharing` | constant | `CGImageDestination.h` | destination / animated_png / heif / proraw helpers<br>`ffi::kCGImageDestinationOptimizeColorForSharing` |
| `kCGImageDestinationOrientation` | constant | `CGImageDestination.h` | destination / animated_png / heif / proraw helpers<br>`ffi::kCGImageDestinationOrientation` |
| `kCGImageDestinationPreserveGainMap` | constant | `CGImageDestination.h` | destination / animated_png / heif / proraw helpers<br>`ffi::kCGImageDestinationPreserveGainMap` |
| `kCGImageMetadataShouldExcludeGPS` | constant | `CGImageDestination.h` | destination / animated_png / heif / proraw helpers<br>`ffi::kCGImageMetadataShouldExcludeGPS` |
| `kCGImageMetadataShouldExcludeXMP` | constant | `CGImageDestination.h` | destination / animated_png / heif / proraw helpers<br>`ffi::kCGImageMetadataShouldExcludeXMP` |
| `kCGImagePropertyASTCBlockSize` | constant | `CGImageDestination.h` | destination / animated_png / heif / proraw helpers<br>`ffi::kCGImagePropertyASTCBlockSize` |
| `kCGImagePropertyASTCBlockSize4x4` | constant | `CGImageDestination.h` | destination / animated_png / heif / proraw helpers<br>`ffi::kCGImagePropertyASTCBlockSize4x4` |
| `kCGImagePropertyASTCBlockSize8x8` | constant | `CGImageDestination.h` | destination / animated_png / heif / proraw helpers<br>`ffi::kCGImagePropertyASTCBlockSize8x8` |
| `kCGImagePropertyASTCEncoder` | constant | `CGImageDestination.h` | destination / animated_png / heif / proraw helpers<br>`ffi::kCGImagePropertyASTCEncoder` |
| `kCGImagePropertyBCEncoder` | constant | `CGImageDestination.h` | destination / animated_png / heif / proraw helpers<br>`ffi::kCGImagePropertyBCEncoder` |
| `kCGImagePropertyBCFormat` | constant | `CGImageDestination.h` | destination / animated_png / heif / proraw helpers<br>`ffi::kCGImagePropertyBCFormat` |
| `kCGImagePropertyEncoder` | constant | `CGImageDestination.h` | destination / animated_png / heif / proraw helpers<br>`ffi::kCGImagePropertyEncoder` |
| `kCGImagePropertyPVREncoder` | constant | `CGImageDestination.h` | destination / animated_png / heif / proraw helpers<br>`ffi::kCGImagePropertyPVREncoder` |
| `CGImageAnimationStatus` | type | `CGImageAnimation.h` | animate_image / animate_image_from_bytes<br>`ffi::CGImageAnimationStatus` |
| `CGImageSourceAnimationBlock` | type | `CGImageAnimation.h` | animate_image / animate_image_from_bytes<br>`ffi::CGImageSourceAnimationBlock` |
| `CGImageAnimationStatus` | enum type | `CGImageAnimation.h` | animate_image / animate_image_from_bytes<br>`ffi::CGImageAnimationStatus` |
| `kCGImageAnimationStatus_AllocationFailure` | enum case | `CGImageAnimation.h` | animation / animated_png helpers<br>`ffi::kCGImageAnimationStatus_AllocationFailure` |
| `kCGImageAnimationStatus_CorruptInputImage` | enum case | `CGImageAnimation.h` | animation / animated_png helpers<br>`ffi::kCGImageAnimationStatus_CorruptInputImage` |
| `kCGImageAnimationStatus_IncompleteInputImage` | enum case | `CGImageAnimation.h` | animation / animated_png helpers<br>`ffi::kCGImageAnimationStatus_IncompleteInputImage` |
| `kCGImageAnimationStatus_ParameterError` | enum case | `CGImageAnimation.h` | animation / animated_png helpers<br>`ffi::kCGImageAnimationStatus_ParameterError` |
| `kCGImageAnimationStatus_UnsupportedFormat` | enum case | `CGImageAnimation.h` | animation / animated_png helpers<br>`ffi::kCGImageAnimationStatus_UnsupportedFormat` |
| `CGAnimateImageAtURLWithBlock` | function | `CGImageAnimation.h` | animate_image<br>`ffi::CGAnimateImageAtURLWithBlock` |
| `CGAnimateImageDataWithBlock` | function | `CGImageAnimation.h` | animate_image_from_bytes<br>`ffi::CGAnimateImageDataWithBlock` |
| `kCGImageAnimationDelayTime` | constant | `CGImageAnimation.h` | animation / animated_png helpers<br>`ffi::kCGImageAnimationDelayTime` |
| `kCGImageAnimationLoopCount` | constant | `CGImageAnimation.h` | animation / animated_png helpers<br>`ffi::kCGImageAnimationLoopCount` |
| `kCGImageAnimationStartIndex` | constant | `CGImageAnimation.h` | animation / animated_png helpers<br>`ffi::kCGImageAnimationStartIndex` |
| `CGImageMetadataErrors` | type | `CGImageMetadata.h` | metadata helpers<br>`ffi::CGImageMetadataErrors` |
| `CGImageMetadataRef` | type | `CGImageMetadata.h` | Metadata<br>`ffi::CGImageMetadataRef` |
| `CGImageMetadataTagBlock` | type | `CGImageMetadata.h` | Metadata::enumerate_tags<br>`ffi::CGImageMetadataTagBlock` |
| `CGImageMetadataTagRef` | type | `CGImageMetadata.h` | MetadataTag<br>`ffi::CGImageMetadataTagRef` |
| `CGImageMetadataType` | type | `CGImageMetadata.h` | MetadataType<br>`ffi::CGImageMetadataType` |
| `CGMutableImageMetadataRef` | type | `CGImageMetadata.h` | MutableMetadata<br>`ffi::CGMutableImageMetadataRef` |
| `CGImageMetadataErrors` | enum type | `CGImageMetadata.h` | metadata helpers<br>`ffi::CGImageMetadataErrors` |
| `CGImageMetadataType` | enum type | `CGImageMetadata.h` | MetadataType<br>`ffi::CGImageMetadataType` |
| `kCGImageMetadataErrorBadArgument` | enum case | `CGImageMetadata.h` | metadata helpers<br>`ffi::kCGImageMetadataErrorBadArgument` |
| `kCGImageMetadataErrorConflictingArguments` | enum case | `CGImageMetadata.h` | metadata helpers<br>`ffi::kCGImageMetadataErrorConflictingArguments` |
| `kCGImageMetadataErrorPrefixConflict` | enum case | `CGImageMetadata.h` | metadata helpers<br>`ffi::kCGImageMetadataErrorPrefixConflict` |
| `kCGImageMetadataErrorUnknown` | enum case | `CGImageMetadata.h` | metadata helpers<br>`ffi::kCGImageMetadataErrorUnknown` |
| `kCGImageMetadataErrorUnsupportedFormat` | enum case | `CGImageMetadata.h` | metadata helpers<br>`ffi::kCGImageMetadataErrorUnsupportedFormat` |
| `kCGImageMetadataTypeAlternateArray` | enum case | `CGImageMetadata.h` | metadata helpers<br>`ffi::kCGImageMetadataTypeAlternateArray` |
| `kCGImageMetadataTypeAlternateText` | enum case | `CGImageMetadata.h` | metadata helpers<br>`ffi::kCGImageMetadataTypeAlternateText` |
| `kCGImageMetadataTypeArrayOrdered` | enum case | `CGImageMetadata.h` | metadata helpers<br>`ffi::kCGImageMetadataTypeArrayOrdered` |
| `kCGImageMetadataTypeArrayUnordered` | enum case | `CGImageMetadata.h` | metadata helpers<br>`ffi::kCGImageMetadataTypeArrayUnordered` |
| `kCGImageMetadataTypeDefault` | enum case | `CGImageMetadata.h` | metadata helpers<br>`ffi::kCGImageMetadataTypeDefault` |
| `kCGImageMetadataTypeInvalid` | enum case | `CGImageMetadata.h` | metadata helpers<br>`ffi::kCGImageMetadataTypeInvalid` |
| `kCGImageMetadataTypeString` | enum case | `CGImageMetadata.h` | metadata helpers<br>`ffi::kCGImageMetadataTypeString` |
| `kCGImageMetadataTypeStructure` | enum case | `CGImageMetadata.h` | metadata helpers<br>`ffi::kCGImageMetadataTypeStructure` |
| `CGImageMetadataCopyStringValueWithPath` | function | `CGImageMetadata.h` | metadata helpers<br>`ffi::CGImageMetadataCopyStringValueWithPath` |
| `CGImageMetadataCopyTagMatchingImageProperty` | function | `CGImageMetadata.h` | metadata helpers<br>`ffi::CGImageMetadataCopyTagMatchingImageProperty` |
| `CGImageMetadataCopyTags` | function | `CGImageMetadata.h` | metadata helpers<br>`ffi::CGImageMetadataCopyTags` |
| `CGImageMetadataCopyTagWithPath` | function | `CGImageMetadata.h` | metadata helpers<br>`ffi::CGImageMetadataCopyTagWithPath` |
| `CGImageMetadataCreateFromXMPData` | function | `CGImageMetadata.h` | metadata helpers<br>`ffi::CGImageMetadataCreateFromXMPData` |
| `CGImageMetadataCreateMutable` | function | `CGImageMetadata.h` | metadata helpers<br>`ffi::CGImageMetadataCreateMutable` |
| `CGImageMetadataCreateMutableCopy` | function | `CGImageMetadata.h` | metadata helpers<br>`ffi::CGImageMetadataCreateMutableCopy` |
| `CGImageMetadataCreateXMPData` | function | `CGImageMetadata.h` | metadata helpers<br>`ffi::CGImageMetadataCreateXMPData` |
| `CGImageMetadataEnumerateTagsUsingBlock` | function | `CGImageMetadata.h` | metadata helpers<br>`ffi::CGImageMetadataEnumerateTagsUsingBlock` |
| `CGImageMetadataGetTypeID` | function | `CGImageMetadata.h` | metadata helpers<br>`ffi::CGImageMetadataGetTypeID` |
| `CGImageMetadataRegisterNamespaceForPrefix` | function | `CGImageMetadata.h` | metadata helpers<br>`ffi::CGImageMetadataRegisterNamespaceForPrefix` |
| `CGImageMetadataRemoveTagWithPath` | function | `CGImageMetadata.h` | metadata helpers<br>`ffi::CGImageMetadataRemoveTagWithPath` |
| `CGImageMetadataSetTagWithPath` | function | `CGImageMetadata.h` | metadata helpers<br>`ffi::CGImageMetadataSetTagWithPath` |
| `CGImageMetadataSetValueMatchingImageProperty` | function | `CGImageMetadata.h` | metadata helpers<br>`ffi::CGImageMetadataSetValueMatchingImageProperty` |
| `CGImageMetadataSetValueWithPath` | function | `CGImageMetadata.h` | metadata helpers<br>`ffi::CGImageMetadataSetValueWithPath` |
| `CGImageMetadataTagCopyName` | function | `CGImageMetadata.h` | metadata helpers<br>`ffi::CGImageMetadataTagCopyName` |
| `CGImageMetadataTagCopyNamespace` | function | `CGImageMetadata.h` | metadata helpers<br>`ffi::CGImageMetadataTagCopyNamespace` |
| `CGImageMetadataTagCopyPrefix` | function | `CGImageMetadata.h` | metadata helpers<br>`ffi::CGImageMetadataTagCopyPrefix` |
| `CGImageMetadataTagCopyQualifiers` | function | `CGImageMetadata.h` | metadata helpers<br>`ffi::CGImageMetadataTagCopyQualifiers` |
| `CGImageMetadataTagCopyValue` | function | `CGImageMetadata.h` | metadata helpers<br>`ffi::CGImageMetadataTagCopyValue` |
| `CGImageMetadataTagCreate` | function | `CGImageMetadata.h` | metadata helpers<br>`ffi::CGImageMetadataTagCreate` |
| `CGImageMetadataTagGetType` | function | `CGImageMetadata.h` | metadata helpers<br>`ffi::CGImageMetadataTagGetType` |
| `CGImageMetadataTagGetTypeID` | function | `CGImageMetadata.h` | metadata helpers<br>`ffi::CGImageMetadataTagGetTypeID` |
| `kCGImageMetadataNamespaceDublinCore` | constant | `CGImageMetadata.h` | metadata helpers<br>`ffi::kCGImageMetadataNamespaceDublinCore` |
| `kCGImageMetadataNamespaceExif` | constant | `CGImageMetadata.h` | metadata helpers<br>`ffi::kCGImageMetadataNamespaceExif` |
| `kCGImageMetadataNamespaceExifAux` | constant | `CGImageMetadata.h` | metadata helpers<br>`ffi::kCGImageMetadataNamespaceExifAux` |
| `kCGImageMetadataNamespaceExifEX` | constant | `CGImageMetadata.h` | metadata helpers<br>`ffi::kCGImageMetadataNamespaceExifEX` |
| `kCGImageMetadataNamespaceIPTCCore` | constant | `CGImageMetadata.h` | metadata helpers<br>`ffi::kCGImageMetadataNamespaceIPTCCore` |
| `kCGImageMetadataNamespaceIPTCExtension` | constant | `CGImageMetadata.h` | metadata helpers<br>`ffi::kCGImageMetadataNamespaceIPTCExtension` |
| `kCGImageMetadataNamespacePhotoshop` | constant | `CGImageMetadata.h` | metadata helpers<br>`ffi::kCGImageMetadataNamespacePhotoshop` |
| `kCGImageMetadataNamespaceTIFF` | constant | `CGImageMetadata.h` | metadata helpers<br>`ffi::kCGImageMetadataNamespaceTIFF` |
| `kCGImageMetadataNamespaceXMPBasic` | constant | `CGImageMetadata.h` | metadata helpers<br>`ffi::kCGImageMetadataNamespaceXMPBasic` |
| `kCGImageMetadataNamespaceXMPRights` | constant | `CGImageMetadata.h` | metadata helpers<br>`ffi::kCGImageMetadataNamespaceXMPRights` |
| `kCGImageMetadataPrefixDublinCore` | constant | `CGImageMetadata.h` | metadata helpers<br>`ffi::kCGImageMetadataPrefixDublinCore` |
| `kCGImageMetadataPrefixExif` | constant | `CGImageMetadata.h` | metadata helpers<br>`ffi::kCGImageMetadataPrefixExif` |
| `kCGImageMetadataPrefixExifAux` | constant | `CGImageMetadata.h` | metadata helpers<br>`ffi::kCGImageMetadataPrefixExifAux` |
| `kCGImageMetadataPrefixExifEX` | constant | `CGImageMetadata.h` | metadata helpers<br>`ffi::kCGImageMetadataPrefixExifEX` |
| `kCGImageMetadataPrefixIPTCCore` | constant | `CGImageMetadata.h` | metadata helpers<br>`ffi::kCGImageMetadataPrefixIPTCCore` |
| `kCGImageMetadataPrefixIPTCExtension` | constant | `CGImageMetadata.h` | metadata helpers<br>`ffi::kCGImageMetadataPrefixIPTCExtension` |
| `kCGImageMetadataPrefixPhotoshop` | constant | `CGImageMetadata.h` | metadata helpers<br>`ffi::kCGImageMetadataPrefixPhotoshop` |
| `kCGImageMetadataPrefixTIFF` | constant | `CGImageMetadata.h` | metadata helpers<br>`ffi::kCGImageMetadataPrefixTIFF` |
| `kCGImageMetadataPrefixXMPBasic` | constant | `CGImageMetadata.h` | metadata helpers<br>`ffi::kCGImageMetadataPrefixXMPBasic` |
| `kCGImageMetadataPrefixXMPRights` | constant | `CGImageMetadata.h` | metadata helpers<br>`ffi::kCGImageMetadataPrefixXMPRights` |
| `kCGImageAuxiliaryDataInfoColorSpace` | constant | `CGImageProperties.h` | properties / animated_png / heif / proraw / color_sync / thumbnail helpers<br>`ffi::kCGImageAuxiliaryDataInfoColorSpace` |
| `kCGImageAuxiliaryDataInfoData` | constant | `CGImageProperties.h` | properties / animated_png / heif / proraw / color_sync / thumbnail helpers<br>`ffi::kCGImageAuxiliaryDataInfoData` |
| `kCGImageAuxiliaryDataInfoDataDescription` | constant | `CGImageProperties.h` | properties / animated_png / heif / proraw / color_sync / thumbnail helpers<br>`ffi::kCGImageAuxiliaryDataInfoDataDescription` |
| `kCGImageAuxiliaryDataInfoMetadata` | constant | `CGImageProperties.h` | properties / animated_png / heif / proraw / color_sync / thumbnail helpers<br>`ffi::kCGImageAuxiliaryDataInfoMetadata` |
| `kCGImageAuxiliaryDataTypeDepth` | constant | `CGImageProperties.h` | properties / animated_png / heif / proraw / color_sync / thumbnail helpers<br>`ffi::kCGImageAuxiliaryDataTypeDepth` |
| `kCGImageAuxiliaryDataTypeDisparity` | constant | `CGImageProperties.h` | properties / animated_png / heif / proraw / color_sync / thumbnail helpers<br>`ffi::kCGImageAuxiliaryDataTypeDisparity` |
| `kCGImageAuxiliaryDataTypeHDRGainMap` | constant | `CGImageProperties.h` | properties / animated_png / heif / proraw / color_sync / thumbnail helpers<br>`ffi::kCGImageAuxiliaryDataTypeHDRGainMap` |
| `kCGImageAuxiliaryDataTypeISOGainMap` | constant | `CGImageProperties.h` | properties / animated_png / heif / proraw / color_sync / thumbnail helpers<br>`ffi::kCGImageAuxiliaryDataTypeISOGainMap` |
| `kCGImageAuxiliaryDataTypePortraitEffectsMatte` | constant | `CGImageProperties.h` | properties / animated_png / heif / proraw / color_sync / thumbnail helpers<br>`ffi::kCGImageAuxiliaryDataTypePortraitEffectsMatte` |
| `kCGImageAuxiliaryDataTypeSemanticSegmentationGlassesMatte` | constant | `CGImageProperties.h` | properties / animated_png / heif / proraw / color_sync / thumbnail helpers<br>`ffi::kCGImageAuxiliaryDataTypeSemanticSegmentationGlassesMatte` |
| `kCGImageAuxiliaryDataTypeSemanticSegmentationHairMatte` | constant | `CGImageProperties.h` | properties / animated_png / heif / proraw / color_sync / thumbnail helpers<br>`ffi::kCGImageAuxiliaryDataTypeSemanticSegmentationHairMatte` |
| `kCGImageAuxiliaryDataTypeSemanticSegmentationSkinMatte` | constant | `CGImageProperties.h` | properties / animated_png / heif / proraw / color_sync / thumbnail helpers<br>`ffi::kCGImageAuxiliaryDataTypeSemanticSegmentationSkinMatte` |
| `kCGImageAuxiliaryDataTypeSemanticSegmentationSkyMatte` | constant | `CGImageProperties.h` | properties / animated_png / heif / proraw / color_sync / thumbnail helpers<br>`ffi::kCGImageAuxiliaryDataTypeSemanticSegmentationSkyMatte` |
| `kCGImageAuxiliaryDataTypeSemanticSegmentationTeethMatte` | constant | `CGImageProperties.h` | properties / animated_png / heif / proraw / color_sync / thumbnail helpers<br>`ffi::kCGImageAuxiliaryDataTypeSemanticSegmentationTeethMatte` |
| `kCGImageProperty8BIMDictionary` | constant | `CGImageProperties.h` | properties / animated_png / heif / proraw / color_sync / thumbnail helpers<br>`ffi::kCGImageProperty8BIMDictionary` |
| `kCGImageProperty8BIMLayerNames` | constant | `CGImageProperties.h` | properties / animated_png / heif / proraw / color_sync / thumbnail helpers<br>`ffi::kCGImageProperty8BIMLayerNames` |
| `kCGImageProperty8BIMVersion` | constant | `CGImageProperties.h` | properties / animated_png / heif / proraw / color_sync / thumbnail helpers<br>`ffi::kCGImageProperty8BIMVersion` |
| `kCGImagePropertyAPNGCanvasPixelHeight` | constant | `CGImageProperties.h` | properties / animated_png / heif / proraw / color_sync / thumbnail helpers<br>`ffi::kCGImagePropertyAPNGCanvasPixelHeight` |
| `kCGImagePropertyAPNGCanvasPixelWidth` | constant | `CGImageProperties.h` | properties / animated_png / heif / proraw / color_sync / thumbnail helpers<br>`ffi::kCGImagePropertyAPNGCanvasPixelWidth` |
| `kCGImagePropertyAPNGDelayTime` | constant | `CGImageProperties.h` | properties / animated_png / heif / proraw / color_sync / thumbnail helpers<br>`ffi::kCGImagePropertyAPNGDelayTime` |
| `kCGImagePropertyAPNGFrameInfoArray` | constant | `CGImageProperties.h` | properties / animated_png / heif / proraw / color_sync / thumbnail helpers<br>`ffi::kCGImagePropertyAPNGFrameInfoArray` |
| `kCGImagePropertyAPNGLoopCount` | constant | `CGImageProperties.h` | properties / animated_png / heif / proraw / color_sync / thumbnail helpers<br>`ffi::kCGImagePropertyAPNGLoopCount` |
| `kCGImagePropertyAPNGUnclampedDelayTime` | constant | `CGImageProperties.h` | properties / animated_png / heif / proraw / color_sync / thumbnail helpers<br>`ffi::kCGImagePropertyAPNGUnclampedDelayTime` |
| `kCGImagePropertyAuxiliaryData` | constant | `CGImageProperties.h` | properties / animated_png / heif / proraw / color_sync / thumbnail helpers<br>`ffi::kCGImagePropertyAuxiliaryData` |
| `kCGImagePropertyAuxiliaryDataType` | constant | `CGImageProperties.h` | properties / animated_png / heif / proraw / color_sync / thumbnail helpers<br>`ffi::kCGImagePropertyAuxiliaryDataType` |
| `kCGImagePropertyAVISDictionary` | constant | `CGImageProperties.h` | properties / animated_png / heif / proraw / color_sync / thumbnail helpers<br>`ffi::kCGImagePropertyAVISDictionary` |
| `kCGImagePropertyBytesPerRow` | constant | `CGImageProperties.h` | properties / animated_png / heif / proraw / color_sync / thumbnail helpers<br>`ffi::kCGImagePropertyBytesPerRow` |
| `kCGImagePropertyCIFFCameraSerialNumber` | constant | `CGImageProperties.h` | properties / animated_png / heif / proraw / color_sync / thumbnail helpers<br>`ffi::kCGImagePropertyCIFFCameraSerialNumber` |
| `kCGImagePropertyCIFFContinuousDrive` | constant | `CGImageProperties.h` | properties / animated_png / heif / proraw / color_sync / thumbnail helpers<br>`ffi::kCGImagePropertyCIFFContinuousDrive` |
| `kCGImagePropertyCIFFDescription` | constant | `CGImageProperties.h` | properties / animated_png / heif / proraw / color_sync / thumbnail helpers<br>`ffi::kCGImagePropertyCIFFDescription` |
| `kCGImagePropertyCIFFDictionary` | constant | `CGImageProperties.h` | properties / animated_png / heif / proraw / color_sync / thumbnail helpers<br>`ffi::kCGImagePropertyCIFFDictionary` |
| `kCGImagePropertyCIFFFirmware` | constant | `CGImageProperties.h` | properties / animated_png / heif / proraw / color_sync / thumbnail helpers<br>`ffi::kCGImagePropertyCIFFFirmware` |
| `kCGImagePropertyCIFFFlashExposureComp` | constant | `CGImageProperties.h` | properties / animated_png / heif / proraw / color_sync / thumbnail helpers<br>`ffi::kCGImagePropertyCIFFFlashExposureComp` |
| `kCGImagePropertyCIFFFocusMode` | constant | `CGImageProperties.h` | properties / animated_png / heif / proraw / color_sync / thumbnail helpers<br>`ffi::kCGImagePropertyCIFFFocusMode` |
| `kCGImagePropertyCIFFImageFileName` | constant | `CGImageProperties.h` | properties / animated_png / heif / proraw / color_sync / thumbnail helpers<br>`ffi::kCGImagePropertyCIFFImageFileName` |
| `kCGImagePropertyCIFFImageName` | constant | `CGImageProperties.h` | properties / animated_png / heif / proraw / color_sync / thumbnail helpers<br>`ffi::kCGImagePropertyCIFFImageName` |
| `kCGImagePropertyCIFFImageSerialNumber` | constant | `CGImageProperties.h` | properties / animated_png / heif / proraw / color_sync / thumbnail helpers<br>`ffi::kCGImagePropertyCIFFImageSerialNumber` |
| `kCGImagePropertyCIFFLensMaxMM` | constant | `CGImageProperties.h` | properties / animated_png / heif / proraw / color_sync / thumbnail helpers<br>`ffi::kCGImagePropertyCIFFLensMaxMM` |
| `kCGImagePropertyCIFFLensMinMM` | constant | `CGImageProperties.h` | properties / animated_png / heif / proraw / color_sync / thumbnail helpers<br>`ffi::kCGImagePropertyCIFFLensMinMM` |
| `kCGImagePropertyCIFFLensModel` | constant | `CGImageProperties.h` | properties / animated_png / heif / proraw / color_sync / thumbnail helpers<br>`ffi::kCGImagePropertyCIFFLensModel` |
| `kCGImagePropertyCIFFMeasuredEV` | constant | `CGImageProperties.h` | properties / animated_png / heif / proraw / color_sync / thumbnail helpers<br>`ffi::kCGImagePropertyCIFFMeasuredEV` |
| `kCGImagePropertyCIFFMeteringMode` | constant | `CGImageProperties.h` | properties / animated_png / heif / proraw / color_sync / thumbnail helpers<br>`ffi::kCGImagePropertyCIFFMeteringMode` |
| `kCGImagePropertyCIFFOwnerName` | constant | `CGImageProperties.h` | properties / animated_png / heif / proraw / color_sync / thumbnail helpers<br>`ffi::kCGImagePropertyCIFFOwnerName` |
| `kCGImagePropertyCIFFRecordID` | constant | `CGImageProperties.h` | properties / animated_png / heif / proraw / color_sync / thumbnail helpers<br>`ffi::kCGImagePropertyCIFFRecordID` |
| `kCGImagePropertyCIFFReleaseMethod` | constant | `CGImageProperties.h` | properties / animated_png / heif / proraw / color_sync / thumbnail helpers<br>`ffi::kCGImagePropertyCIFFReleaseMethod` |
| `kCGImagePropertyCIFFReleaseTiming` | constant | `CGImageProperties.h` | properties / animated_png / heif / proraw / color_sync / thumbnail helpers<br>`ffi::kCGImagePropertyCIFFReleaseTiming` |
| `kCGImagePropertyCIFFSelfTimingTime` | constant | `CGImageProperties.h` | properties / animated_png / heif / proraw / color_sync / thumbnail helpers<br>`ffi::kCGImagePropertyCIFFSelfTimingTime` |
| `kCGImagePropertyCIFFShootingMode` | constant | `CGImageProperties.h` | properties / animated_png / heif / proraw / color_sync / thumbnail helpers<br>`ffi::kCGImagePropertyCIFFShootingMode` |
| `kCGImagePropertyCIFFWhiteBalanceIndex` | constant | `CGImageProperties.h` | properties / animated_png / heif / proraw / color_sync / thumbnail helpers<br>`ffi::kCGImagePropertyCIFFWhiteBalanceIndex` |
| `kCGImagePropertyColorModel` | constant | `CGImageProperties.h` | properties / animated_png / heif / proraw / color_sync / thumbnail helpers<br>`ffi::kCGImagePropertyColorModel` |
| `kCGImagePropertyColorModelCMYK` | constant | `CGImageProperties.h` | properties / animated_png / heif / proraw / color_sync / thumbnail helpers<br>`ffi::kCGImagePropertyColorModelCMYK` |
| `kCGImagePropertyColorModelGray` | constant | `CGImageProperties.h` | properties / animated_png / heif / proraw / color_sync / thumbnail helpers<br>`ffi::kCGImagePropertyColorModelGray` |
| `kCGImagePropertyColorModelLab` | constant | `CGImageProperties.h` | properties / animated_png / heif / proraw / color_sync / thumbnail helpers<br>`ffi::kCGImagePropertyColorModelLab` |
| `kCGImagePropertyColorModelRGB` | constant | `CGImageProperties.h` | properties / animated_png / heif / proraw / color_sync / thumbnail helpers<br>`ffi::kCGImagePropertyColorModelRGB` |
| `kCGImagePropertyDepth` | constant | `CGImageProperties.h` | properties / animated_png / heif / proraw / color_sync / thumbnail helpers<br>`ffi::kCGImagePropertyDepth` |
| `kCGImagePropertyDNGActiveArea` | constant | `CGImageProperties.h` | properties / animated_png / heif / proraw / color_sync / thumbnail helpers<br>`ffi::kCGImagePropertyDNGActiveArea` |
| `kCGImagePropertyDNGAnalogBalance` | constant | `CGImageProperties.h` | properties / animated_png / heif / proraw / color_sync / thumbnail helpers<br>`ffi::kCGImagePropertyDNGAnalogBalance` |
| `kCGImagePropertyDNGAntiAliasStrength` | constant | `CGImageProperties.h` | properties / animated_png / heif / proraw / color_sync / thumbnail helpers<br>`ffi::kCGImagePropertyDNGAntiAliasStrength` |
| `kCGImagePropertyDNGAsShotICCProfile` | constant | `CGImageProperties.h` | properties / animated_png / heif / proraw / color_sync / thumbnail helpers<br>`ffi::kCGImagePropertyDNGAsShotICCProfile` |
| `kCGImagePropertyDNGAsShotNeutral` | constant | `CGImageProperties.h` | properties / animated_png / heif / proraw / color_sync / thumbnail helpers<br>`ffi::kCGImagePropertyDNGAsShotNeutral` |
| `kCGImagePropertyDNGAsShotPreProfileMatrix` | constant | `CGImageProperties.h` | properties / animated_png / heif / proraw / color_sync / thumbnail helpers<br>`ffi::kCGImagePropertyDNGAsShotPreProfileMatrix` |
| `kCGImagePropertyDNGAsShotProfileName` | constant | `CGImageProperties.h` | properties / animated_png / heif / proraw / color_sync / thumbnail helpers<br>`ffi::kCGImagePropertyDNGAsShotProfileName` |
| `kCGImagePropertyDNGAsShotWhiteXY` | constant | `CGImageProperties.h` | properties / animated_png / heif / proraw / color_sync / thumbnail helpers<br>`ffi::kCGImagePropertyDNGAsShotWhiteXY` |
| `kCGImagePropertyDNGBackwardVersion` | constant | `CGImageProperties.h` | properties / animated_png / heif / proraw / color_sync / thumbnail helpers<br>`ffi::kCGImagePropertyDNGBackwardVersion` |
| `kCGImagePropertyDNGBaselineExposure` | constant | `CGImageProperties.h` | properties / animated_png / heif / proraw / color_sync / thumbnail helpers<br>`ffi::kCGImagePropertyDNGBaselineExposure` |
| `kCGImagePropertyDNGBaselineExposureOffset` | constant | `CGImageProperties.h` | properties / animated_png / heif / proraw / color_sync / thumbnail helpers<br>`ffi::kCGImagePropertyDNGBaselineExposureOffset` |
| `kCGImagePropertyDNGBaselineNoise` | constant | `CGImageProperties.h` | properties / animated_png / heif / proraw / color_sync / thumbnail helpers<br>`ffi::kCGImagePropertyDNGBaselineNoise` |
| `kCGImagePropertyDNGBaselineSharpness` | constant | `CGImageProperties.h` | properties / animated_png / heif / proraw / color_sync / thumbnail helpers<br>`ffi::kCGImagePropertyDNGBaselineSharpness` |
| `kCGImagePropertyDNGBayerGreenSplit` | constant | `CGImageProperties.h` | properties / animated_png / heif / proraw / color_sync / thumbnail helpers<br>`ffi::kCGImagePropertyDNGBayerGreenSplit` |
| `kCGImagePropertyDNGBestQualityScale` | constant | `CGImageProperties.h` | properties / animated_png / heif / proraw / color_sync / thumbnail helpers<br>`ffi::kCGImagePropertyDNGBestQualityScale` |
| `kCGImagePropertyDNGBlackLevel` | constant | `CGImageProperties.h` | properties / animated_png / heif / proraw / color_sync / thumbnail helpers<br>`ffi::kCGImagePropertyDNGBlackLevel` |
| `kCGImagePropertyDNGBlackLevelDeltaH` | constant | `CGImageProperties.h` | properties / animated_png / heif / proraw / color_sync / thumbnail helpers<br>`ffi::kCGImagePropertyDNGBlackLevelDeltaH` |
| `kCGImagePropertyDNGBlackLevelDeltaV` | constant | `CGImageProperties.h` | properties / animated_png / heif / proraw / color_sync / thumbnail helpers<br>`ffi::kCGImagePropertyDNGBlackLevelDeltaV` |
| `kCGImagePropertyDNGBlackLevelRepeatDim` | constant | `CGImageProperties.h` | properties / animated_png / heif / proraw / color_sync / thumbnail helpers<br>`ffi::kCGImagePropertyDNGBlackLevelRepeatDim` |
| `kCGImagePropertyDNGCalibrationIlluminant1` | constant | `CGImageProperties.h` | properties / animated_png / heif / proraw / color_sync / thumbnail helpers<br>`ffi::kCGImagePropertyDNGCalibrationIlluminant1` |
| `kCGImagePropertyDNGCalibrationIlluminant2` | constant | `CGImageProperties.h` | properties / animated_png / heif / proraw / color_sync / thumbnail helpers<br>`ffi::kCGImagePropertyDNGCalibrationIlluminant2` |
| `kCGImagePropertyDNGCameraCalibration1` | constant | `CGImageProperties.h` | properties / animated_png / heif / proraw / color_sync / thumbnail helpers<br>`ffi::kCGImagePropertyDNGCameraCalibration1` |
| `kCGImagePropertyDNGCameraCalibration2` | constant | `CGImageProperties.h` | properties / animated_png / heif / proraw / color_sync / thumbnail helpers<br>`ffi::kCGImagePropertyDNGCameraCalibration2` |
| `kCGImagePropertyDNGCameraCalibrationSignature` | constant | `CGImageProperties.h` | properties / animated_png / heif / proraw / color_sync / thumbnail helpers<br>`ffi::kCGImagePropertyDNGCameraCalibrationSignature` |
| `kCGImagePropertyDNGCameraSerialNumber` | constant | `CGImageProperties.h` | properties / animated_png / heif / proraw / color_sync / thumbnail helpers<br>`ffi::kCGImagePropertyDNGCameraSerialNumber` |
| `kCGImagePropertyDNGCFALayout` | constant | `CGImageProperties.h` | properties / animated_png / heif / proraw / color_sync / thumbnail helpers<br>`ffi::kCGImagePropertyDNGCFALayout` |
| `kCGImagePropertyDNGCFAPlaneColor` | constant | `CGImageProperties.h` | properties / animated_png / heif / proraw / color_sync / thumbnail helpers<br>`ffi::kCGImagePropertyDNGCFAPlaneColor` |
| `kCGImagePropertyDNGChromaBlurRadius` | constant | `CGImageProperties.h` | properties / animated_png / heif / proraw / color_sync / thumbnail helpers<br>`ffi::kCGImagePropertyDNGChromaBlurRadius` |
| `kCGImagePropertyDNGColorimetricReference` | constant | `CGImageProperties.h` | properties / animated_png / heif / proraw / color_sync / thumbnail helpers<br>`ffi::kCGImagePropertyDNGColorimetricReference` |
| `kCGImagePropertyDNGColorMatrix1` | constant | `CGImageProperties.h` | properties / animated_png / heif / proraw / color_sync / thumbnail helpers<br>`ffi::kCGImagePropertyDNGColorMatrix1` |
| `kCGImagePropertyDNGColorMatrix2` | constant | `CGImageProperties.h` | properties / animated_png / heif / proraw / color_sync / thumbnail helpers<br>`ffi::kCGImagePropertyDNGColorMatrix2` |
| `kCGImagePropertyDNGCurrentICCProfile` | constant | `CGImageProperties.h` | properties / animated_png / heif / proraw / color_sync / thumbnail helpers<br>`ffi::kCGImagePropertyDNGCurrentICCProfile` |
| `kCGImagePropertyDNGCurrentPreProfileMatrix` | constant | `CGImageProperties.h` | properties / animated_png / heif / proraw / color_sync / thumbnail helpers<br>`ffi::kCGImagePropertyDNGCurrentPreProfileMatrix` |
| `kCGImagePropertyDNGDefaultBlackRender` | constant | `CGImageProperties.h` | properties / animated_png / heif / proraw / color_sync / thumbnail helpers<br>`ffi::kCGImagePropertyDNGDefaultBlackRender` |
| `kCGImagePropertyDNGDefaultCropOrigin` | constant | `CGImageProperties.h` | properties / animated_png / heif / proraw / color_sync / thumbnail helpers<br>`ffi::kCGImagePropertyDNGDefaultCropOrigin` |
| `kCGImagePropertyDNGDefaultCropSize` | constant | `CGImageProperties.h` | properties / animated_png / heif / proraw / color_sync / thumbnail helpers<br>`ffi::kCGImagePropertyDNGDefaultCropSize` |
| `kCGImagePropertyDNGDefaultScale` | constant | `CGImageProperties.h` | properties / animated_png / heif / proraw / color_sync / thumbnail helpers<br>`ffi::kCGImagePropertyDNGDefaultScale` |
| `kCGImagePropertyDNGDefaultUserCrop` | constant | `CGImageProperties.h` | properties / animated_png / heif / proraw / color_sync / thumbnail helpers<br>`ffi::kCGImagePropertyDNGDefaultUserCrop` |
| `kCGImagePropertyDNGDictionary` | constant | `CGImageProperties.h` | properties / animated_png / heif / proraw / color_sync / thumbnail helpers<br>`ffi::kCGImagePropertyDNGDictionary` |
| `kCGImagePropertyDNGExtraCameraProfiles` | constant | `CGImageProperties.h` | properties / animated_png / heif / proraw / color_sync / thumbnail helpers<br>`ffi::kCGImagePropertyDNGExtraCameraProfiles` |
| `kCGImagePropertyDNGFixVignetteRadial` | constant | `CGImageProperties.h` | properties / animated_png / heif / proraw / color_sync / thumbnail helpers<br>`ffi::kCGImagePropertyDNGFixVignetteRadial` |
| `kCGImagePropertyDNGForwardMatrix1` | constant | `CGImageProperties.h` | properties / animated_png / heif / proraw / color_sync / thumbnail helpers<br>`ffi::kCGImagePropertyDNGForwardMatrix1` |
| `kCGImagePropertyDNGForwardMatrix2` | constant | `CGImageProperties.h` | properties / animated_png / heif / proraw / color_sync / thumbnail helpers<br>`ffi::kCGImagePropertyDNGForwardMatrix2` |
| `kCGImagePropertyDNGLensInfo` | constant | `CGImageProperties.h` | properties / animated_png / heif / proraw / color_sync / thumbnail helpers<br>`ffi::kCGImagePropertyDNGLensInfo` |
| `kCGImagePropertyDNGLinearizationTable` | constant | `CGImageProperties.h` | properties / animated_png / heif / proraw / color_sync / thumbnail helpers<br>`ffi::kCGImagePropertyDNGLinearizationTable` |
| `kCGImagePropertyDNGLinearResponseLimit` | constant | `CGImageProperties.h` | properties / animated_png / heif / proraw / color_sync / thumbnail helpers<br>`ffi::kCGImagePropertyDNGLinearResponseLimit` |
| `kCGImagePropertyDNGLocalizedCameraModel` | constant | `CGImageProperties.h` | properties / animated_png / heif / proraw / color_sync / thumbnail helpers<br>`ffi::kCGImagePropertyDNGLocalizedCameraModel` |
| `kCGImagePropertyDNGMakerNoteSafety` | constant | `CGImageProperties.h` | properties / animated_png / heif / proraw / color_sync / thumbnail helpers<br>`ffi::kCGImagePropertyDNGMakerNoteSafety` |
| `kCGImagePropertyDNGMaskedAreas` | constant | `CGImageProperties.h` | properties / animated_png / heif / proraw / color_sync / thumbnail helpers<br>`ffi::kCGImagePropertyDNGMaskedAreas` |
| `kCGImagePropertyDNGNewRawImageDigest` | constant | `CGImageProperties.h` | properties / animated_png / heif / proraw / color_sync / thumbnail helpers<br>`ffi::kCGImagePropertyDNGNewRawImageDigest` |
| `kCGImagePropertyDNGNoiseProfile` | constant | `CGImageProperties.h` | properties / animated_png / heif / proraw / color_sync / thumbnail helpers<br>`ffi::kCGImagePropertyDNGNoiseProfile` |
| `kCGImagePropertyDNGNoiseReductionApplied` | constant | `CGImageProperties.h` | properties / animated_png / heif / proraw / color_sync / thumbnail helpers<br>`ffi::kCGImagePropertyDNGNoiseReductionApplied` |
| `kCGImagePropertyDNGOpcodeList1` | constant | `CGImageProperties.h` | properties / animated_png / heif / proraw / color_sync / thumbnail helpers<br>`ffi::kCGImagePropertyDNGOpcodeList1` |
| `kCGImagePropertyDNGOpcodeList2` | constant | `CGImageProperties.h` | properties / animated_png / heif / proraw / color_sync / thumbnail helpers<br>`ffi::kCGImagePropertyDNGOpcodeList2` |
| `kCGImagePropertyDNGOpcodeList3` | constant | `CGImageProperties.h` | properties / animated_png / heif / proraw / color_sync / thumbnail helpers<br>`ffi::kCGImagePropertyDNGOpcodeList3` |
| `kCGImagePropertyDNGOriginalBestQualityFinalSize` | constant | `CGImageProperties.h` | properties / animated_png / heif / proraw / color_sync / thumbnail helpers<br>`ffi::kCGImagePropertyDNGOriginalBestQualityFinalSize` |
| `kCGImagePropertyDNGOriginalDefaultCropSize` | constant | `CGImageProperties.h` | properties / animated_png / heif / proraw / color_sync / thumbnail helpers<br>`ffi::kCGImagePropertyDNGOriginalDefaultCropSize` |
| `kCGImagePropertyDNGOriginalDefaultFinalSize` | constant | `CGImageProperties.h` | properties / animated_png / heif / proraw / color_sync / thumbnail helpers<br>`ffi::kCGImagePropertyDNGOriginalDefaultFinalSize` |
| `kCGImagePropertyDNGOriginalRawFileData` | constant | `CGImageProperties.h` | properties / animated_png / heif / proraw / color_sync / thumbnail helpers<br>`ffi::kCGImagePropertyDNGOriginalRawFileData` |
| `kCGImagePropertyDNGOriginalRawFileDigest` | constant | `CGImageProperties.h` | properties / animated_png / heif / proraw / color_sync / thumbnail helpers<br>`ffi::kCGImagePropertyDNGOriginalRawFileDigest` |
| `kCGImagePropertyDNGOriginalRawFileName` | constant | `CGImageProperties.h` | properties / animated_png / heif / proraw / color_sync / thumbnail helpers<br>`ffi::kCGImagePropertyDNGOriginalRawFileName` |
| `kCGImagePropertyDNGPreviewApplicationName` | constant | `CGImageProperties.h` | properties / animated_png / heif / proraw / color_sync / thumbnail helpers<br>`ffi::kCGImagePropertyDNGPreviewApplicationName` |
| `kCGImagePropertyDNGPreviewApplicationVersion` | constant | `CGImageProperties.h` | properties / animated_png / heif / proraw / color_sync / thumbnail helpers<br>`ffi::kCGImagePropertyDNGPreviewApplicationVersion` |
| `kCGImagePropertyDNGPreviewColorSpace` | constant | `CGImageProperties.h` | properties / animated_png / heif / proraw / color_sync / thumbnail helpers<br>`ffi::kCGImagePropertyDNGPreviewColorSpace` |
| `kCGImagePropertyDNGPreviewDateTime` | constant | `CGImageProperties.h` | properties / animated_png / heif / proraw / color_sync / thumbnail helpers<br>`ffi::kCGImagePropertyDNGPreviewDateTime` |
| `kCGImagePropertyDNGPreviewSettingsDigest` | constant | `CGImageProperties.h` | properties / animated_png / heif / proraw / color_sync / thumbnail helpers<br>`ffi::kCGImagePropertyDNGPreviewSettingsDigest` |
| `kCGImagePropertyDNGPreviewSettingsName` | constant | `CGImageProperties.h` | properties / animated_png / heif / proraw / color_sync / thumbnail helpers<br>`ffi::kCGImagePropertyDNGPreviewSettingsName` |
| `kCGImagePropertyDNGPrivateData` | constant | `CGImageProperties.h` | properties / animated_png / heif / proraw / color_sync / thumbnail helpers<br>`ffi::kCGImagePropertyDNGPrivateData` |
| `kCGImagePropertyDNGProfileCalibrationSignature` | constant | `CGImageProperties.h` | properties / animated_png / heif / proraw / color_sync / thumbnail helpers<br>`ffi::kCGImagePropertyDNGProfileCalibrationSignature` |
| `kCGImagePropertyDNGProfileCopyright` | constant | `CGImageProperties.h` | properties / animated_png / heif / proraw / color_sync / thumbnail helpers<br>`ffi::kCGImagePropertyDNGProfileCopyright` |
| `kCGImagePropertyDNGProfileEmbedPolicy` | constant | `CGImageProperties.h` | properties / animated_png / heif / proraw / color_sync / thumbnail helpers<br>`ffi::kCGImagePropertyDNGProfileEmbedPolicy` |
| `kCGImagePropertyDNGProfileHueSatMapData1` | constant | `CGImageProperties.h` | properties / animated_png / heif / proraw / color_sync / thumbnail helpers<br>`ffi::kCGImagePropertyDNGProfileHueSatMapData1` |
| `kCGImagePropertyDNGProfileHueSatMapData2` | constant | `CGImageProperties.h` | properties / animated_png / heif / proraw / color_sync / thumbnail helpers<br>`ffi::kCGImagePropertyDNGProfileHueSatMapData2` |
| `kCGImagePropertyDNGProfileHueSatMapDims` | constant | `CGImageProperties.h` | properties / animated_png / heif / proraw / color_sync / thumbnail helpers<br>`ffi::kCGImagePropertyDNGProfileHueSatMapDims` |
| `kCGImagePropertyDNGProfileHueSatMapEncoding` | constant | `CGImageProperties.h` | properties / animated_png / heif / proraw / color_sync / thumbnail helpers<br>`ffi::kCGImagePropertyDNGProfileHueSatMapEncoding` |
| `kCGImagePropertyDNGProfileLookTableData` | constant | `CGImageProperties.h` | properties / animated_png / heif / proraw / color_sync / thumbnail helpers<br>`ffi::kCGImagePropertyDNGProfileLookTableData` |
| `kCGImagePropertyDNGProfileLookTableDims` | constant | `CGImageProperties.h` | properties / animated_png / heif / proraw / color_sync / thumbnail helpers<br>`ffi::kCGImagePropertyDNGProfileLookTableDims` |
| `kCGImagePropertyDNGProfileLookTableEncoding` | constant | `CGImageProperties.h` | properties / animated_png / heif / proraw / color_sync / thumbnail helpers<br>`ffi::kCGImagePropertyDNGProfileLookTableEncoding` |
| `kCGImagePropertyDNGProfileName` | constant | `CGImageProperties.h` | properties / animated_png / heif / proraw / color_sync / thumbnail helpers<br>`ffi::kCGImagePropertyDNGProfileName` |
| `kCGImagePropertyDNGProfileToneCurve` | constant | `CGImageProperties.h` | properties / animated_png / heif / proraw / color_sync / thumbnail helpers<br>`ffi::kCGImagePropertyDNGProfileToneCurve` |
| `kCGImagePropertyDNGRawDataUniqueID` | constant | `CGImageProperties.h` | properties / animated_png / heif / proraw / color_sync / thumbnail helpers<br>`ffi::kCGImagePropertyDNGRawDataUniqueID` |
| `kCGImagePropertyDNGRawImageDigest` | constant | `CGImageProperties.h` | properties / animated_png / heif / proraw / color_sync / thumbnail helpers<br>`ffi::kCGImagePropertyDNGRawImageDigest` |
| `kCGImagePropertyDNGRawToPreviewGain` | constant | `CGImageProperties.h` | properties / animated_png / heif / proraw / color_sync / thumbnail helpers<br>`ffi::kCGImagePropertyDNGRawToPreviewGain` |
| `kCGImagePropertyDNGReductionMatrix1` | constant | `CGImageProperties.h` | properties / animated_png / heif / proraw / color_sync / thumbnail helpers<br>`ffi::kCGImagePropertyDNGReductionMatrix1` |
| `kCGImagePropertyDNGReductionMatrix2` | constant | `CGImageProperties.h` | properties / animated_png / heif / proraw / color_sync / thumbnail helpers<br>`ffi::kCGImagePropertyDNGReductionMatrix2` |
| `kCGImagePropertyDNGRowInterleaveFactor` | constant | `CGImageProperties.h` | properties / animated_png / heif / proraw / color_sync / thumbnail helpers<br>`ffi::kCGImagePropertyDNGRowInterleaveFactor` |
| `kCGImagePropertyDNGShadowScale` | constant | `CGImageProperties.h` | properties / animated_png / heif / proraw / color_sync / thumbnail helpers<br>`ffi::kCGImagePropertyDNGShadowScale` |
| `kCGImagePropertyDNGSubTileBlockSize` | constant | `CGImageProperties.h` | properties / animated_png / heif / proraw / color_sync / thumbnail helpers<br>`ffi::kCGImagePropertyDNGSubTileBlockSize` |
| `kCGImagePropertyDNGUniqueCameraModel` | constant | `CGImageProperties.h` | properties / animated_png / heif / proraw / color_sync / thumbnail helpers<br>`ffi::kCGImagePropertyDNGUniqueCameraModel` |
| `kCGImagePropertyDNGVersion` | constant | `CGImageProperties.h` | properties / animated_png / heif / proraw / color_sync / thumbnail helpers<br>`ffi::kCGImagePropertyDNGVersion` |
| `kCGImagePropertyDNGWarpFisheye` | constant | `CGImageProperties.h` | properties / animated_png / heif / proraw / color_sync / thumbnail helpers<br>`ffi::kCGImagePropertyDNGWarpFisheye` |
| `kCGImagePropertyDNGWarpRectilinear` | constant | `CGImageProperties.h` | properties / animated_png / heif / proraw / color_sync / thumbnail helpers<br>`ffi::kCGImagePropertyDNGWarpRectilinear` |
| `kCGImagePropertyDNGWhiteLevel` | constant | `CGImageProperties.h` | properties / animated_png / heif / proraw / color_sync / thumbnail helpers<br>`ffi::kCGImagePropertyDNGWhiteLevel` |
| `kCGImagePropertyDPIHeight` | constant | `CGImageProperties.h` | properties / animated_png / heif / proraw / color_sync / thumbnail helpers<br>`ffi::kCGImagePropertyDPIHeight` |
| `kCGImagePropertyDPIWidth` | constant | `CGImageProperties.h` | properties / animated_png / heif / proraw / color_sync / thumbnail helpers<br>`ffi::kCGImagePropertyDPIWidth` |
| `kCGImagePropertyExifApertureValue` | constant | `CGImageProperties.h` | properties / animated_png / heif / proraw / color_sync / thumbnail helpers<br>`ffi::kCGImagePropertyExifApertureValue` |
| `kCGImagePropertyExifAuxDictionary` | constant | `CGImageProperties.h` | properties / animated_png / heif / proraw / color_sync / thumbnail helpers<br>`ffi::kCGImagePropertyExifAuxDictionary` |
| `kCGImagePropertyExifAuxFirmware` | constant | `CGImageProperties.h` | properties / animated_png / heif / proraw / color_sync / thumbnail helpers<br>`ffi::kCGImagePropertyExifAuxFirmware` |
| `kCGImagePropertyExifAuxFlashCompensation` | constant | `CGImageProperties.h` | properties / animated_png / heif / proraw / color_sync / thumbnail helpers<br>`ffi::kCGImagePropertyExifAuxFlashCompensation` |
| `kCGImagePropertyExifAuxImageNumber` | constant | `CGImageProperties.h` | properties / animated_png / heif / proraw / color_sync / thumbnail helpers<br>`ffi::kCGImagePropertyExifAuxImageNumber` |
| `kCGImagePropertyExifAuxLensID` | constant | `CGImageProperties.h` | properties / animated_png / heif / proraw / color_sync / thumbnail helpers<br>`ffi::kCGImagePropertyExifAuxLensID` |
| `kCGImagePropertyExifAuxLensInfo` | constant | `CGImageProperties.h` | properties / animated_png / heif / proraw / color_sync / thumbnail helpers<br>`ffi::kCGImagePropertyExifAuxLensInfo` |
| `kCGImagePropertyExifAuxLensModel` | constant | `CGImageProperties.h` | properties / animated_png / heif / proraw / color_sync / thumbnail helpers<br>`ffi::kCGImagePropertyExifAuxLensModel` |
| `kCGImagePropertyExifAuxLensSerialNumber` | constant | `CGImageProperties.h` | properties / animated_png / heif / proraw / color_sync / thumbnail helpers<br>`ffi::kCGImagePropertyExifAuxLensSerialNumber` |
| `kCGImagePropertyExifAuxOwnerName` | constant | `CGImageProperties.h` | properties / animated_png / heif / proraw / color_sync / thumbnail helpers<br>`ffi::kCGImagePropertyExifAuxOwnerName` |
| `kCGImagePropertyExifAuxSerialNumber` | constant | `CGImageProperties.h` | properties / animated_png / heif / proraw / color_sync / thumbnail helpers<br>`ffi::kCGImagePropertyExifAuxSerialNumber` |
| `kCGImagePropertyExifBodySerialNumber` | constant | `CGImageProperties.h` | properties / animated_png / heif / proraw / color_sync / thumbnail helpers<br>`ffi::kCGImagePropertyExifBodySerialNumber` |
| `kCGImagePropertyExifBrightnessValue` | constant | `CGImageProperties.h` | properties / animated_png / heif / proraw / color_sync / thumbnail helpers<br>`ffi::kCGImagePropertyExifBrightnessValue` |
| `kCGImagePropertyExifCameraOwnerName` | constant | `CGImageProperties.h` | properties / animated_png / heif / proraw / color_sync / thumbnail helpers<br>`ffi::kCGImagePropertyExifCameraOwnerName` |
| `kCGImagePropertyExifCFAPattern` | constant | `CGImageProperties.h` | properties / animated_png / heif / proraw / color_sync / thumbnail helpers<br>`ffi::kCGImagePropertyExifCFAPattern` |
| `kCGImagePropertyExifColorSpace` | constant | `CGImageProperties.h` | properties / animated_png / heif / proraw / color_sync / thumbnail helpers<br>`ffi::kCGImagePropertyExifColorSpace` |
| `kCGImagePropertyExifComponentsConfiguration` | constant | `CGImageProperties.h` | properties / animated_png / heif / proraw / color_sync / thumbnail helpers<br>`ffi::kCGImagePropertyExifComponentsConfiguration` |
| `kCGImagePropertyExifCompositeImage` | constant | `CGImageProperties.h` | properties / animated_png / heif / proraw / color_sync / thumbnail helpers<br>`ffi::kCGImagePropertyExifCompositeImage` |
| `kCGImagePropertyExifCompressedBitsPerPixel` | constant | `CGImageProperties.h` | properties / animated_png / heif / proraw / color_sync / thumbnail helpers<br>`ffi::kCGImagePropertyExifCompressedBitsPerPixel` |
| `kCGImagePropertyExifContrast` | constant | `CGImageProperties.h` | properties / animated_png / heif / proraw / color_sync / thumbnail helpers<br>`ffi::kCGImagePropertyExifContrast` |
| `kCGImagePropertyExifCustomRendered` | constant | `CGImageProperties.h` | properties / animated_png / heif / proraw / color_sync / thumbnail helpers<br>`ffi::kCGImagePropertyExifCustomRendered` |
| `kCGImagePropertyExifDateTimeDigitized` | constant | `CGImageProperties.h` | properties / animated_png / heif / proraw / color_sync / thumbnail helpers<br>`ffi::kCGImagePropertyExifDateTimeDigitized` |
| `kCGImagePropertyExifDateTimeOriginal` | constant | `CGImageProperties.h` | properties / animated_png / heif / proraw / color_sync / thumbnail helpers<br>`ffi::kCGImagePropertyExifDateTimeOriginal` |
| `kCGImagePropertyExifDeviceSettingDescription` | constant | `CGImageProperties.h` | properties / animated_png / heif / proraw / color_sync / thumbnail helpers<br>`ffi::kCGImagePropertyExifDeviceSettingDescription` |
| `kCGImagePropertyExifDictionary` | constant | `CGImageProperties.h` | properties / animated_png / heif / proraw / color_sync / thumbnail helpers<br>`ffi::kCGImagePropertyExifDictionary` |
| `kCGImagePropertyExifDigitalZoomRatio` | constant | `CGImageProperties.h` | properties / animated_png / heif / proraw / color_sync / thumbnail helpers<br>`ffi::kCGImagePropertyExifDigitalZoomRatio` |
| `kCGImagePropertyExifExposureBiasValue` | constant | `CGImageProperties.h` | properties / animated_png / heif / proraw / color_sync / thumbnail helpers<br>`ffi::kCGImagePropertyExifExposureBiasValue` |
| `kCGImagePropertyExifExposureIndex` | constant | `CGImageProperties.h` | properties / animated_png / heif / proraw / color_sync / thumbnail helpers<br>`ffi::kCGImagePropertyExifExposureIndex` |
| `kCGImagePropertyExifExposureMode` | constant | `CGImageProperties.h` | properties / animated_png / heif / proraw / color_sync / thumbnail helpers<br>`ffi::kCGImagePropertyExifExposureMode` |
| `kCGImagePropertyExifExposureProgram` | constant | `CGImageProperties.h` | properties / animated_png / heif / proraw / color_sync / thumbnail helpers<br>`ffi::kCGImagePropertyExifExposureProgram` |
| `kCGImagePropertyExifExposureTime` | constant | `CGImageProperties.h` | properties / animated_png / heif / proraw / color_sync / thumbnail helpers<br>`ffi::kCGImagePropertyExifExposureTime` |
| `kCGImagePropertyExifFileSource` | constant | `CGImageProperties.h` | properties / animated_png / heif / proraw / color_sync / thumbnail helpers<br>`ffi::kCGImagePropertyExifFileSource` |
| `kCGImagePropertyExifFlash` | constant | `CGImageProperties.h` | properties / animated_png / heif / proraw / color_sync / thumbnail helpers<br>`ffi::kCGImagePropertyExifFlash` |
| `kCGImagePropertyExifFlashEnergy` | constant | `CGImageProperties.h` | properties / animated_png / heif / proraw / color_sync / thumbnail helpers<br>`ffi::kCGImagePropertyExifFlashEnergy` |
| `kCGImagePropertyExifFlashPixVersion` | constant | `CGImageProperties.h` | properties / animated_png / heif / proraw / color_sync / thumbnail helpers<br>`ffi::kCGImagePropertyExifFlashPixVersion` |
| `kCGImagePropertyExifFNumber` | constant | `CGImageProperties.h` | properties / animated_png / heif / proraw / color_sync / thumbnail helpers<br>`ffi::kCGImagePropertyExifFNumber` |
| `kCGImagePropertyExifFocalLength` | constant | `CGImageProperties.h` | properties / animated_png / heif / proraw / color_sync / thumbnail helpers<br>`ffi::kCGImagePropertyExifFocalLength` |
| `kCGImagePropertyExifFocalLenIn35mmFilm` | constant | `CGImageProperties.h` | properties / animated_png / heif / proraw / color_sync / thumbnail helpers<br>`ffi::kCGImagePropertyExifFocalLenIn35mmFilm` |
| `kCGImagePropertyExifFocalPlaneResolutionUnit` | constant | `CGImageProperties.h` | properties / animated_png / heif / proraw / color_sync / thumbnail helpers<br>`ffi::kCGImagePropertyExifFocalPlaneResolutionUnit` |
| `kCGImagePropertyExifFocalPlaneXResolution` | constant | `CGImageProperties.h` | properties / animated_png / heif / proraw / color_sync / thumbnail helpers<br>`ffi::kCGImagePropertyExifFocalPlaneXResolution` |
| `kCGImagePropertyExifFocalPlaneYResolution` | constant | `CGImageProperties.h` | properties / animated_png / heif / proraw / color_sync / thumbnail helpers<br>`ffi::kCGImagePropertyExifFocalPlaneYResolution` |
| `kCGImagePropertyExifGainControl` | constant | `CGImageProperties.h` | properties / animated_png / heif / proraw / color_sync / thumbnail helpers<br>`ffi::kCGImagePropertyExifGainControl` |
| `kCGImagePropertyExifGamma` | constant | `CGImageProperties.h` | properties / animated_png / heif / proraw / color_sync / thumbnail helpers<br>`ffi::kCGImagePropertyExifGamma` |
| `kCGImagePropertyExifImageUniqueID` | constant | `CGImageProperties.h` | properties / animated_png / heif / proraw / color_sync / thumbnail helpers<br>`ffi::kCGImagePropertyExifImageUniqueID` |
| `kCGImagePropertyExifISOSpeed` | constant | `CGImageProperties.h` | properties / animated_png / heif / proraw / color_sync / thumbnail helpers<br>`ffi::kCGImagePropertyExifISOSpeed` |
| `kCGImagePropertyExifISOSpeedLatitudeyyy` | constant | `CGImageProperties.h` | properties / animated_png / heif / proraw / color_sync / thumbnail helpers<br>`ffi::kCGImagePropertyExifISOSpeedLatitudeyyy` |
| `kCGImagePropertyExifISOSpeedLatitudezzz` | constant | `CGImageProperties.h` | properties / animated_png / heif / proraw / color_sync / thumbnail helpers<br>`ffi::kCGImagePropertyExifISOSpeedLatitudezzz` |
| `kCGImagePropertyExifISOSpeedRatings` | constant | `CGImageProperties.h` | properties / animated_png / heif / proraw / color_sync / thumbnail helpers<br>`ffi::kCGImagePropertyExifISOSpeedRatings` |
| `kCGImagePropertyExifLensMake` | constant | `CGImageProperties.h` | properties / animated_png / heif / proraw / color_sync / thumbnail helpers<br>`ffi::kCGImagePropertyExifLensMake` |
| `kCGImagePropertyExifLensModel` | constant | `CGImageProperties.h` | properties / animated_png / heif / proraw / color_sync / thumbnail helpers<br>`ffi::kCGImagePropertyExifLensModel` |
| `kCGImagePropertyExifLensSerialNumber` | constant | `CGImageProperties.h` | properties / animated_png / heif / proraw / color_sync / thumbnail helpers<br>`ffi::kCGImagePropertyExifLensSerialNumber` |
| `kCGImagePropertyExifLensSpecification` | constant | `CGImageProperties.h` | properties / animated_png / heif / proraw / color_sync / thumbnail helpers<br>`ffi::kCGImagePropertyExifLensSpecification` |
| `kCGImagePropertyExifLightSource` | constant | `CGImageProperties.h` | properties / animated_png / heif / proraw / color_sync / thumbnail helpers<br>`ffi::kCGImagePropertyExifLightSource` |
| `kCGImagePropertyExifMakerNote` | constant | `CGImageProperties.h` | properties / animated_png / heif / proraw / color_sync / thumbnail helpers<br>`ffi::kCGImagePropertyExifMakerNote` |
| `kCGImagePropertyExifMaxApertureValue` | constant | `CGImageProperties.h` | properties / animated_png / heif / proraw / color_sync / thumbnail helpers<br>`ffi::kCGImagePropertyExifMaxApertureValue` |
| `kCGImagePropertyExifMeteringMode` | constant | `CGImageProperties.h` | properties / animated_png / heif / proraw / color_sync / thumbnail helpers<br>`ffi::kCGImagePropertyExifMeteringMode` |
| `kCGImagePropertyExifOECF` | constant | `CGImageProperties.h` | properties / animated_png / heif / proraw / color_sync / thumbnail helpers<br>`ffi::kCGImagePropertyExifOECF` |
| `kCGImagePropertyExifOffsetTime` | constant | `CGImageProperties.h` | properties / animated_png / heif / proraw / color_sync / thumbnail helpers<br>`ffi::kCGImagePropertyExifOffsetTime` |
| `kCGImagePropertyExifOffsetTimeDigitized` | constant | `CGImageProperties.h` | properties / animated_png / heif / proraw / color_sync / thumbnail helpers<br>`ffi::kCGImagePropertyExifOffsetTimeDigitized` |
| `kCGImagePropertyExifOffsetTimeOriginal` | constant | `CGImageProperties.h` | properties / animated_png / heif / proraw / color_sync / thumbnail helpers<br>`ffi::kCGImagePropertyExifOffsetTimeOriginal` |
| `kCGImagePropertyExifPixelXDimension` | constant | `CGImageProperties.h` | properties / animated_png / heif / proraw / color_sync / thumbnail helpers<br>`ffi::kCGImagePropertyExifPixelXDimension` |
| `kCGImagePropertyExifPixelYDimension` | constant | `CGImageProperties.h` | properties / animated_png / heif / proraw / color_sync / thumbnail helpers<br>`ffi::kCGImagePropertyExifPixelYDimension` |
| `kCGImagePropertyExifRecommendedExposureIndex` | constant | `CGImageProperties.h` | properties / animated_png / heif / proraw / color_sync / thumbnail helpers<br>`ffi::kCGImagePropertyExifRecommendedExposureIndex` |
| `kCGImagePropertyExifRelatedSoundFile` | constant | `CGImageProperties.h` | properties / animated_png / heif / proraw / color_sync / thumbnail helpers<br>`ffi::kCGImagePropertyExifRelatedSoundFile` |
| `kCGImagePropertyExifSaturation` | constant | `CGImageProperties.h` | properties / animated_png / heif / proraw / color_sync / thumbnail helpers<br>`ffi::kCGImagePropertyExifSaturation` |
| `kCGImagePropertyExifSceneCaptureType` | constant | `CGImageProperties.h` | properties / animated_png / heif / proraw / color_sync / thumbnail helpers<br>`ffi::kCGImagePropertyExifSceneCaptureType` |
| `kCGImagePropertyExifSceneType` | constant | `CGImageProperties.h` | properties / animated_png / heif / proraw / color_sync / thumbnail helpers<br>`ffi::kCGImagePropertyExifSceneType` |
| `kCGImagePropertyExifSensingMethod` | constant | `CGImageProperties.h` | properties / animated_png / heif / proraw / color_sync / thumbnail helpers<br>`ffi::kCGImagePropertyExifSensingMethod` |
| `kCGImagePropertyExifSensitivityType` | constant | `CGImageProperties.h` | properties / animated_png / heif / proraw / color_sync / thumbnail helpers<br>`ffi::kCGImagePropertyExifSensitivityType` |
| `kCGImagePropertyExifSharpness` | constant | `CGImageProperties.h` | properties / animated_png / heif / proraw / color_sync / thumbnail helpers<br>`ffi::kCGImagePropertyExifSharpness` |
| `kCGImagePropertyExifShutterSpeedValue` | constant | `CGImageProperties.h` | properties / animated_png / heif / proraw / color_sync / thumbnail helpers<br>`ffi::kCGImagePropertyExifShutterSpeedValue` |
| `kCGImagePropertyExifSourceExposureTimesOfCompositeImage` | constant | `CGImageProperties.h` | properties / animated_png / heif / proraw / color_sync / thumbnail helpers<br>`ffi::kCGImagePropertyExifSourceExposureTimesOfCompositeImage` |
| `kCGImagePropertyExifSourceImageNumberOfCompositeImage` | constant | `CGImageProperties.h` | properties / animated_png / heif / proraw / color_sync / thumbnail helpers<br>`ffi::kCGImagePropertyExifSourceImageNumberOfCompositeImage` |
| `kCGImagePropertyExifSpatialFrequencyResponse` | constant | `CGImageProperties.h` | properties / animated_png / heif / proraw / color_sync / thumbnail helpers<br>`ffi::kCGImagePropertyExifSpatialFrequencyResponse` |
| `kCGImagePropertyExifSpectralSensitivity` | constant | `CGImageProperties.h` | properties / animated_png / heif / proraw / color_sync / thumbnail helpers<br>`ffi::kCGImagePropertyExifSpectralSensitivity` |
| `kCGImagePropertyExifStandardOutputSensitivity` | constant | `CGImageProperties.h` | properties / animated_png / heif / proraw / color_sync / thumbnail helpers<br>`ffi::kCGImagePropertyExifStandardOutputSensitivity` |
| `kCGImagePropertyExifSubjectArea` | constant | `CGImageProperties.h` | properties / animated_png / heif / proraw / color_sync / thumbnail helpers<br>`ffi::kCGImagePropertyExifSubjectArea` |
| `kCGImagePropertyExifSubjectDistance` | constant | `CGImageProperties.h` | properties / animated_png / heif / proraw / color_sync / thumbnail helpers<br>`ffi::kCGImagePropertyExifSubjectDistance` |
| `kCGImagePropertyExifSubjectDistRange` | constant | `CGImageProperties.h` | properties / animated_png / heif / proraw / color_sync / thumbnail helpers<br>`ffi::kCGImagePropertyExifSubjectDistRange` |
| `kCGImagePropertyExifSubjectLocation` | constant | `CGImageProperties.h` | properties / animated_png / heif / proraw / color_sync / thumbnail helpers<br>`ffi::kCGImagePropertyExifSubjectLocation` |
| `kCGImagePropertyExifSubsecTime` | constant | `CGImageProperties.h` | properties / animated_png / heif / proraw / color_sync / thumbnail helpers<br>`ffi::kCGImagePropertyExifSubsecTime` |
| `kCGImagePropertyExifSubsecTimeDigitized` | constant | `CGImageProperties.h` | properties / animated_png / heif / proraw / color_sync / thumbnail helpers<br>`ffi::kCGImagePropertyExifSubsecTimeDigitized` |
| `kCGImagePropertyExifSubsecTimeOriginal` | constant | `CGImageProperties.h` | properties / animated_png / heif / proraw / color_sync / thumbnail helpers<br>`ffi::kCGImagePropertyExifSubsecTimeOriginal` |
| `kCGImagePropertyExifUserComment` | constant | `CGImageProperties.h` | properties / animated_png / heif / proraw / color_sync / thumbnail helpers<br>`ffi::kCGImagePropertyExifUserComment` |
| `kCGImagePropertyExifVersion` | constant | `CGImageProperties.h` | properties / animated_png / heif / proraw / color_sync / thumbnail helpers<br>`ffi::kCGImagePropertyExifVersion` |
| `kCGImagePropertyExifWhiteBalance` | constant | `CGImageProperties.h` | properties / animated_png / heif / proraw / color_sync / thumbnail helpers<br>`ffi::kCGImagePropertyExifWhiteBalance` |
| `kCGImagePropertyFileContentsDictionary` | constant | `CGImageProperties.h` | properties / animated_png / heif / proraw / color_sync / thumbnail helpers<br>`ffi::kCGImagePropertyFileContentsDictionary` |
| `kCGImagePropertyFileSize` | constant | `CGImageProperties.h` | properties / animated_png / heif / proraw / color_sync / thumbnail helpers<br>`ffi::kCGImagePropertyFileSize` |
| `kCGImagePropertyGIFCanvasPixelHeight` | constant | `CGImageProperties.h` | properties / animated_png / heif / proraw / color_sync / thumbnail helpers<br>`ffi::kCGImagePropertyGIFCanvasPixelHeight` |
| `kCGImagePropertyGIFCanvasPixelWidth` | constant | `CGImageProperties.h` | properties / animated_png / heif / proraw / color_sync / thumbnail helpers<br>`ffi::kCGImagePropertyGIFCanvasPixelWidth` |
| `kCGImagePropertyGIFDelayTime` | constant | `CGImageProperties.h` | properties / animated_png / heif / proraw / color_sync / thumbnail helpers<br>`ffi::kCGImagePropertyGIFDelayTime` |
| `kCGImagePropertyGIFDictionary` | constant | `CGImageProperties.h` | properties / animated_png / heif / proraw / color_sync / thumbnail helpers<br>`ffi::kCGImagePropertyGIFDictionary` |
| `kCGImagePropertyGIFFrameInfoArray` | constant | `CGImageProperties.h` | properties / animated_png / heif / proraw / color_sync / thumbnail helpers<br>`ffi::kCGImagePropertyGIFFrameInfoArray` |
| `kCGImagePropertyGIFHasGlobalColorMap` | constant | `CGImageProperties.h` | properties / animated_png / heif / proraw / color_sync / thumbnail helpers<br>`ffi::kCGImagePropertyGIFHasGlobalColorMap` |
| `kCGImagePropertyGIFImageColorMap` | constant | `CGImageProperties.h` | properties / animated_png / heif / proraw / color_sync / thumbnail helpers<br>`ffi::kCGImagePropertyGIFImageColorMap` |
| `kCGImagePropertyGIFLoopCount` | constant | `CGImageProperties.h` | properties / animated_png / heif / proraw / color_sync / thumbnail helpers<br>`ffi::kCGImagePropertyGIFLoopCount` |
| `kCGImagePropertyGIFUnclampedDelayTime` | constant | `CGImageProperties.h` | properties / animated_png / heif / proraw / color_sync / thumbnail helpers<br>`ffi::kCGImagePropertyGIFUnclampedDelayTime` |
| `kCGImagePropertyGPSAltitude` | constant | `CGImageProperties.h` | properties / animated_png / heif / proraw / color_sync / thumbnail helpers<br>`ffi::kCGImagePropertyGPSAltitude` |
| `kCGImagePropertyGPSAltitudeRef` | constant | `CGImageProperties.h` | properties / animated_png / heif / proraw / color_sync / thumbnail helpers<br>`ffi::kCGImagePropertyGPSAltitudeRef` |
| `kCGImagePropertyGPSAreaInformation` | constant | `CGImageProperties.h` | properties / animated_png / heif / proraw / color_sync / thumbnail helpers<br>`ffi::kCGImagePropertyGPSAreaInformation` |
| `kCGImagePropertyGPSDateStamp` | constant | `CGImageProperties.h` | properties / animated_png / heif / proraw / color_sync / thumbnail helpers<br>`ffi::kCGImagePropertyGPSDateStamp` |
| `kCGImagePropertyGPSDestBearing` | constant | `CGImageProperties.h` | properties / animated_png / heif / proraw / color_sync / thumbnail helpers<br>`ffi::kCGImagePropertyGPSDestBearing` |
| `kCGImagePropertyGPSDestBearingRef` | constant | `CGImageProperties.h` | properties / animated_png / heif / proraw / color_sync / thumbnail helpers<br>`ffi::kCGImagePropertyGPSDestBearingRef` |
| `kCGImagePropertyGPSDestDistance` | constant | `CGImageProperties.h` | properties / animated_png / heif / proraw / color_sync / thumbnail helpers<br>`ffi::kCGImagePropertyGPSDestDistance` |
| `kCGImagePropertyGPSDestDistanceRef` | constant | `CGImageProperties.h` | properties / animated_png / heif / proraw / color_sync / thumbnail helpers<br>`ffi::kCGImagePropertyGPSDestDistanceRef` |
| `kCGImagePropertyGPSDestLatitude` | constant | `CGImageProperties.h` | properties / animated_png / heif / proraw / color_sync / thumbnail helpers<br>`ffi::kCGImagePropertyGPSDestLatitude` |
| `kCGImagePropertyGPSDestLatitudeRef` | constant | `CGImageProperties.h` | properties / animated_png / heif / proraw / color_sync / thumbnail helpers<br>`ffi::kCGImagePropertyGPSDestLatitudeRef` |
| `kCGImagePropertyGPSDestLongitude` | constant | `CGImageProperties.h` | properties / animated_png / heif / proraw / color_sync / thumbnail helpers<br>`ffi::kCGImagePropertyGPSDestLongitude` |
| `kCGImagePropertyGPSDestLongitudeRef` | constant | `CGImageProperties.h` | properties / animated_png / heif / proraw / color_sync / thumbnail helpers<br>`ffi::kCGImagePropertyGPSDestLongitudeRef` |
| `kCGImagePropertyGPSDictionary` | constant | `CGImageProperties.h` | properties / animated_png / heif / proraw / color_sync / thumbnail helpers<br>`ffi::kCGImagePropertyGPSDictionary` |
| `kCGImagePropertyGPSDifferental` | constant | `CGImageProperties.h` | properties / animated_png / heif / proraw / color_sync / thumbnail helpers<br>`ffi::kCGImagePropertyGPSDifferental` |
| `kCGImagePropertyGPSDOP` | constant | `CGImageProperties.h` | properties / animated_png / heif / proraw / color_sync / thumbnail helpers<br>`ffi::kCGImagePropertyGPSDOP` |
| `kCGImagePropertyGPSHPositioningError` | constant | `CGImageProperties.h` | properties / animated_png / heif / proraw / color_sync / thumbnail helpers<br>`ffi::kCGImagePropertyGPSHPositioningError` |
| `kCGImagePropertyGPSImgDirection` | constant | `CGImageProperties.h` | properties / animated_png / heif / proraw / color_sync / thumbnail helpers<br>`ffi::kCGImagePropertyGPSImgDirection` |
| `kCGImagePropertyGPSImgDirectionRef` | constant | `CGImageProperties.h` | properties / animated_png / heif / proraw / color_sync / thumbnail helpers<br>`ffi::kCGImagePropertyGPSImgDirectionRef` |
| `kCGImagePropertyGPSLatitude` | constant | `CGImageProperties.h` | properties / animated_png / heif / proraw / color_sync / thumbnail helpers<br>`ffi::kCGImagePropertyGPSLatitude` |
| `kCGImagePropertyGPSLatitudeRef` | constant | `CGImageProperties.h` | properties / animated_png / heif / proraw / color_sync / thumbnail helpers<br>`ffi::kCGImagePropertyGPSLatitudeRef` |
| `kCGImagePropertyGPSLongitude` | constant | `CGImageProperties.h` | properties / animated_png / heif / proraw / color_sync / thumbnail helpers<br>`ffi::kCGImagePropertyGPSLongitude` |
| `kCGImagePropertyGPSLongitudeRef` | constant | `CGImageProperties.h` | properties / animated_png / heif / proraw / color_sync / thumbnail helpers<br>`ffi::kCGImagePropertyGPSLongitudeRef` |
| `kCGImagePropertyGPSMapDatum` | constant | `CGImageProperties.h` | properties / animated_png / heif / proraw / color_sync / thumbnail helpers<br>`ffi::kCGImagePropertyGPSMapDatum` |
| `kCGImagePropertyGPSMeasureMode` | constant | `CGImageProperties.h` | properties / animated_png / heif / proraw / color_sync / thumbnail helpers<br>`ffi::kCGImagePropertyGPSMeasureMode` |
| `kCGImagePropertyGPSProcessingMethod` | constant | `CGImageProperties.h` | properties / animated_png / heif / proraw / color_sync / thumbnail helpers<br>`ffi::kCGImagePropertyGPSProcessingMethod` |
| `kCGImagePropertyGPSSatellites` | constant | `CGImageProperties.h` | properties / animated_png / heif / proraw / color_sync / thumbnail helpers<br>`ffi::kCGImagePropertyGPSSatellites` |
| `kCGImagePropertyGPSSpeed` | constant | `CGImageProperties.h` | properties / animated_png / heif / proraw / color_sync / thumbnail helpers<br>`ffi::kCGImagePropertyGPSSpeed` |
| `kCGImagePropertyGPSSpeedRef` | constant | `CGImageProperties.h` | properties / animated_png / heif / proraw / color_sync / thumbnail helpers<br>`ffi::kCGImagePropertyGPSSpeedRef` |
| `kCGImagePropertyGPSStatus` | constant | `CGImageProperties.h` | properties / animated_png / heif / proraw / color_sync / thumbnail helpers<br>`ffi::kCGImagePropertyGPSStatus` |
| `kCGImagePropertyGPSTimeStamp` | constant | `CGImageProperties.h` | properties / animated_png / heif / proraw / color_sync / thumbnail helpers<br>`ffi::kCGImagePropertyGPSTimeStamp` |
| `kCGImagePropertyGPSTrack` | constant | `CGImageProperties.h` | properties / animated_png / heif / proraw / color_sync / thumbnail helpers<br>`ffi::kCGImagePropertyGPSTrack` |
| `kCGImagePropertyGPSTrackRef` | constant | `CGImageProperties.h` | properties / animated_png / heif / proraw / color_sync / thumbnail helpers<br>`ffi::kCGImagePropertyGPSTrackRef` |
| `kCGImagePropertyGPSVersion` | constant | `CGImageProperties.h` | properties / animated_png / heif / proraw / color_sync / thumbnail helpers<br>`ffi::kCGImagePropertyGPSVersion` |
| `kCGImagePropertyGroupImageBaseline` | constant | `CGImageProperties.h` | properties / animated_png / heif / proraw / color_sync / thumbnail helpers<br>`ffi::kCGImagePropertyGroupImageBaseline` |
| `kCGImagePropertyGroupImageDisparityAdjustment` | constant | `CGImageProperties.h` | properties / animated_png / heif / proraw / color_sync / thumbnail helpers<br>`ffi::kCGImagePropertyGroupImageDisparityAdjustment` |
| `kCGImagePropertyGroupImageIndexLeft` | constant | `CGImageProperties.h` | properties / animated_png / heif / proraw / color_sync / thumbnail helpers<br>`ffi::kCGImagePropertyGroupImageIndexLeft` |
| `kCGImagePropertyGroupImageIndexMonoscopic` | constant | `CGImageProperties.h` | properties / animated_png / heif / proraw / color_sync / thumbnail helpers<br>`ffi::kCGImagePropertyGroupImageIndexMonoscopic` |
| `kCGImagePropertyGroupImageIndexRight` | constant | `CGImageProperties.h` | properties / animated_png / heif / proraw / color_sync / thumbnail helpers<br>`ffi::kCGImagePropertyGroupImageIndexRight` |
| `kCGImagePropertyGroupImageIsAlternateImage` | constant | `CGImageProperties.h` | properties / animated_png / heif / proraw / color_sync / thumbnail helpers<br>`ffi::kCGImagePropertyGroupImageIsAlternateImage` |
| `kCGImagePropertyGroupImageIsLeftImage` | constant | `CGImageProperties.h` | properties / animated_png / heif / proraw / color_sync / thumbnail helpers<br>`ffi::kCGImagePropertyGroupImageIsLeftImage` |
| `kCGImagePropertyGroupImageIsMonoscopicImage` | constant | `CGImageProperties.h` | properties / animated_png / heif / proraw / color_sync / thumbnail helpers<br>`ffi::kCGImagePropertyGroupImageIsMonoscopicImage` |
| `kCGImagePropertyGroupImageIsRightImage` | constant | `CGImageProperties.h` | properties / animated_png / heif / proraw / color_sync / thumbnail helpers<br>`ffi::kCGImagePropertyGroupImageIsRightImage` |
| `kCGImagePropertyGroupImagesAlternate` | constant | `CGImageProperties.h` | properties / animated_png / heif / proraw / color_sync / thumbnail helpers<br>`ffi::kCGImagePropertyGroupImagesAlternate` |
| `kCGImagePropertyGroupImageStereoAggressors` | constant | `CGImageProperties.h` | properties / animated_png / heif / proraw / color_sync / thumbnail helpers<br>`ffi::kCGImagePropertyGroupImageStereoAggressors` |
| `kCGImagePropertyGroupIndex` | constant | `CGImageProperties.h` | properties / animated_png / heif / proraw / color_sync / thumbnail helpers<br>`ffi::kCGImagePropertyGroupIndex` |
| `kCGImagePropertyGroupMonoscopicImageLocation` | constant | `CGImageProperties.h` | properties / animated_png / heif / proraw / color_sync / thumbnail helpers<br>`ffi::kCGImagePropertyGroupMonoscopicImageLocation` |
| `kCGImagePropertyGroups` | constant | `CGImageProperties.h` | properties / animated_png / heif / proraw / color_sync / thumbnail helpers<br>`ffi::kCGImagePropertyGroups` |
| `kCGImagePropertyGroupType` | constant | `CGImageProperties.h` | properties / animated_png / heif / proraw / color_sync / thumbnail helpers<br>`ffi::kCGImagePropertyGroupType` |
| `kCGImagePropertyGroupTypeAlternate` | constant | `CGImageProperties.h` | properties / animated_png / heif / proraw / color_sync / thumbnail helpers<br>`ffi::kCGImagePropertyGroupTypeAlternate` |
| `kCGImagePropertyGroupTypeStereoPair` | constant | `CGImageProperties.h` | properties / animated_png / heif / proraw / color_sync / thumbnail helpers<br>`ffi::kCGImagePropertyGroupTypeStereoPair` |
| `kCGImagePropertyHasAlpha` | constant | `CGImageProperties.h` | properties / animated_png / heif / proraw / color_sync / thumbnail helpers<br>`ffi::kCGImagePropertyHasAlpha` |
| `kCGImagePropertyHEICSCanvasPixelHeight` | constant | `CGImageProperties.h` | properties / animated_png / heif / proraw / color_sync / thumbnail helpers<br>`ffi::kCGImagePropertyHEICSCanvasPixelHeight` |
| `kCGImagePropertyHEICSCanvasPixelWidth` | constant | `CGImageProperties.h` | properties / animated_png / heif / proraw / color_sync / thumbnail helpers<br>`ffi::kCGImagePropertyHEICSCanvasPixelWidth` |
| `kCGImagePropertyHEICSDelayTime` | constant | `CGImageProperties.h` | properties / animated_png / heif / proraw / color_sync / thumbnail helpers<br>`ffi::kCGImagePropertyHEICSDelayTime` |
| `kCGImagePropertyHEICSDictionary` | constant | `CGImageProperties.h` | properties / animated_png / heif / proraw / color_sync / thumbnail helpers<br>`ffi::kCGImagePropertyHEICSDictionary` |
| `kCGImagePropertyHEICSFrameInfoArray` | constant | `CGImageProperties.h` | properties / animated_png / heif / proraw / color_sync / thumbnail helpers<br>`ffi::kCGImagePropertyHEICSFrameInfoArray` |
| `kCGImagePropertyHEICSLoopCount` | constant | `CGImageProperties.h` | properties / animated_png / heif / proraw / color_sync / thumbnail helpers<br>`ffi::kCGImagePropertyHEICSLoopCount` |
| `kCGImagePropertyHEICSUnclampedDelayTime` | constant | `CGImageProperties.h` | properties / animated_png / heif / proraw / color_sync / thumbnail helpers<br>`ffi::kCGImagePropertyHEICSUnclampedDelayTime` |
| `kCGImagePropertyHEIFDictionary` | constant | `CGImageProperties.h` | properties / animated_png / heif / proraw / color_sync / thumbnail helpers<br>`ffi::kCGImagePropertyHEIFDictionary` |
| `kCGImagePropertyHeight` | constant | `CGImageProperties.h` | properties / animated_png / heif / proraw / color_sync / thumbnail helpers<br>`ffi::kCGImagePropertyHeight` |
| `kCGImagePropertyImageCount` | constant | `CGImageProperties.h` | properties / animated_png / heif / proraw / color_sync / thumbnail helpers<br>`ffi::kCGImagePropertyImageCount` |
| `kCGImagePropertyImageIndex` | constant | `CGImageProperties.h` | properties / animated_png / heif / proraw / color_sync / thumbnail helpers<br>`ffi::kCGImagePropertyImageIndex` |
| `kCGImagePropertyImages` | constant | `CGImageProperties.h` | properties / animated_png / heif / proraw / color_sync / thumbnail helpers<br>`ffi::kCGImagePropertyImages` |
| `kCGImagePropertyIPTCActionAdvised` | constant | `CGImageProperties.h` | properties / animated_png / heif / proraw / color_sync / thumbnail helpers<br>`ffi::kCGImagePropertyIPTCActionAdvised` |
| `kCGImagePropertyIPTCByline` | constant | `CGImageProperties.h` | properties / animated_png / heif / proraw / color_sync / thumbnail helpers<br>`ffi::kCGImagePropertyIPTCByline` |
| `kCGImagePropertyIPTCBylineTitle` | constant | `CGImageProperties.h` | properties / animated_png / heif / proraw / color_sync / thumbnail helpers<br>`ffi::kCGImagePropertyIPTCBylineTitle` |
| `kCGImagePropertyIPTCCaptionAbstract` | constant | `CGImageProperties.h` | properties / animated_png / heif / proraw / color_sync / thumbnail helpers<br>`ffi::kCGImagePropertyIPTCCaptionAbstract` |
| `kCGImagePropertyIPTCCategory` | constant | `CGImageProperties.h` | properties / animated_png / heif / proraw / color_sync / thumbnail helpers<br>`ffi::kCGImagePropertyIPTCCategory` |
| `kCGImagePropertyIPTCCity` | constant | `CGImageProperties.h` | properties / animated_png / heif / proraw / color_sync / thumbnail helpers<br>`ffi::kCGImagePropertyIPTCCity` |
| `kCGImagePropertyIPTCContact` | constant | `CGImageProperties.h` | properties / animated_png / heif / proraw / color_sync / thumbnail helpers<br>`ffi::kCGImagePropertyIPTCContact` |
| `kCGImagePropertyIPTCContactInfoAddress` | constant | `CGImageProperties.h` | properties / animated_png / heif / proraw / color_sync / thumbnail helpers<br>`ffi::kCGImagePropertyIPTCContactInfoAddress` |
| `kCGImagePropertyIPTCContactInfoCity` | constant | `CGImageProperties.h` | properties / animated_png / heif / proraw / color_sync / thumbnail helpers<br>`ffi::kCGImagePropertyIPTCContactInfoCity` |
| `kCGImagePropertyIPTCContactInfoCountry` | constant | `CGImageProperties.h` | properties / animated_png / heif / proraw / color_sync / thumbnail helpers<br>`ffi::kCGImagePropertyIPTCContactInfoCountry` |
| `kCGImagePropertyIPTCContactInfoEmails` | constant | `CGImageProperties.h` | properties / animated_png / heif / proraw / color_sync / thumbnail helpers<br>`ffi::kCGImagePropertyIPTCContactInfoEmails` |
| `kCGImagePropertyIPTCContactInfoPhones` | constant | `CGImageProperties.h` | properties / animated_png / heif / proraw / color_sync / thumbnail helpers<br>`ffi::kCGImagePropertyIPTCContactInfoPhones` |
| `kCGImagePropertyIPTCContactInfoPostalCode` | constant | `CGImageProperties.h` | properties / animated_png / heif / proraw / color_sync / thumbnail helpers<br>`ffi::kCGImagePropertyIPTCContactInfoPostalCode` |
| `kCGImagePropertyIPTCContactInfoStateProvince` | constant | `CGImageProperties.h` | properties / animated_png / heif / proraw / color_sync / thumbnail helpers<br>`ffi::kCGImagePropertyIPTCContactInfoStateProvince` |
| `kCGImagePropertyIPTCContactInfoWebURLs` | constant | `CGImageProperties.h` | properties / animated_png / heif / proraw / color_sync / thumbnail helpers<br>`ffi::kCGImagePropertyIPTCContactInfoWebURLs` |
| `kCGImagePropertyIPTCContentLocationCode` | constant | `CGImageProperties.h` | properties / animated_png / heif / proraw / color_sync / thumbnail helpers<br>`ffi::kCGImagePropertyIPTCContentLocationCode` |
| `kCGImagePropertyIPTCContentLocationName` | constant | `CGImageProperties.h` | properties / animated_png / heif / proraw / color_sync / thumbnail helpers<br>`ffi::kCGImagePropertyIPTCContentLocationName` |
| `kCGImagePropertyIPTCCopyrightNotice` | constant | `CGImageProperties.h` | properties / animated_png / heif / proraw / color_sync / thumbnail helpers<br>`ffi::kCGImagePropertyIPTCCopyrightNotice` |
| `kCGImagePropertyIPTCCountryPrimaryLocationCode` | constant | `CGImageProperties.h` | properties / animated_png / heif / proraw / color_sync / thumbnail helpers<br>`ffi::kCGImagePropertyIPTCCountryPrimaryLocationCode` |
| `kCGImagePropertyIPTCCountryPrimaryLocationName` | constant | `CGImageProperties.h` | properties / animated_png / heif / proraw / color_sync / thumbnail helpers<br>`ffi::kCGImagePropertyIPTCCountryPrimaryLocationName` |
| `kCGImagePropertyIPTCCreatorContactInfo` | constant | `CGImageProperties.h` | properties / animated_png / heif / proraw / color_sync / thumbnail helpers<br>`ffi::kCGImagePropertyIPTCCreatorContactInfo` |
| `kCGImagePropertyIPTCCredit` | constant | `CGImageProperties.h` | properties / animated_png / heif / proraw / color_sync / thumbnail helpers<br>`ffi::kCGImagePropertyIPTCCredit` |
| `kCGImagePropertyIPTCDateCreated` | constant | `CGImageProperties.h` | properties / animated_png / heif / proraw / color_sync / thumbnail helpers<br>`ffi::kCGImagePropertyIPTCDateCreated` |
| `kCGImagePropertyIPTCDictionary` | constant | `CGImageProperties.h` | properties / animated_png / heif / proraw / color_sync / thumbnail helpers<br>`ffi::kCGImagePropertyIPTCDictionary` |
| `kCGImagePropertyIPTCDigitalCreationDate` | constant | `CGImageProperties.h` | properties / animated_png / heif / proraw / color_sync / thumbnail helpers<br>`ffi::kCGImagePropertyIPTCDigitalCreationDate` |
| `kCGImagePropertyIPTCDigitalCreationTime` | constant | `CGImageProperties.h` | properties / animated_png / heif / proraw / color_sync / thumbnail helpers<br>`ffi::kCGImagePropertyIPTCDigitalCreationTime` |
| `kCGImagePropertyIPTCEditorialUpdate` | constant | `CGImageProperties.h` | properties / animated_png / heif / proraw / color_sync / thumbnail helpers<br>`ffi::kCGImagePropertyIPTCEditorialUpdate` |
| `kCGImagePropertyIPTCEditStatus` | constant | `CGImageProperties.h` | properties / animated_png / heif / proraw / color_sync / thumbnail helpers<br>`ffi::kCGImagePropertyIPTCEditStatus` |
| `kCGImagePropertyIPTCExpirationDate` | constant | `CGImageProperties.h` | properties / animated_png / heif / proraw / color_sync / thumbnail helpers<br>`ffi::kCGImagePropertyIPTCExpirationDate` |
| `kCGImagePropertyIPTCExpirationTime` | constant | `CGImageProperties.h` | properties / animated_png / heif / proraw / color_sync / thumbnail helpers<br>`ffi::kCGImagePropertyIPTCExpirationTime` |
| `kCGImagePropertyIPTCExtAboutCvTerm` | constant | `CGImageProperties.h` | properties / animated_png / heif / proraw / color_sync / thumbnail helpers<br>`ffi::kCGImagePropertyIPTCExtAboutCvTerm` |
| `kCGImagePropertyIPTCExtAboutCvTermCvId` | constant | `CGImageProperties.h` | properties / animated_png / heif / proraw / color_sync / thumbnail helpers<br>`ffi::kCGImagePropertyIPTCExtAboutCvTermCvId` |
| `kCGImagePropertyIPTCExtAboutCvTermId` | constant | `CGImageProperties.h` | properties / animated_png / heif / proraw / color_sync / thumbnail helpers<br>`ffi::kCGImagePropertyIPTCExtAboutCvTermId` |
| `kCGImagePropertyIPTCExtAboutCvTermName` | constant | `CGImageProperties.h` | properties / animated_png / heif / proraw / color_sync / thumbnail helpers<br>`ffi::kCGImagePropertyIPTCExtAboutCvTermName` |
| `kCGImagePropertyIPTCExtAboutCvTermRefinedAbout` | constant | `CGImageProperties.h` | properties / animated_png / heif / proraw / color_sync / thumbnail helpers<br>`ffi::kCGImagePropertyIPTCExtAboutCvTermRefinedAbout` |
| `kCGImagePropertyIPTCExtAddlModelInfo` | constant | `CGImageProperties.h` | properties / animated_png / heif / proraw / color_sync / thumbnail helpers<br>`ffi::kCGImagePropertyIPTCExtAddlModelInfo` |
| `kCGImagePropertyIPTCExtArtworkCircaDateCreated` | constant | `CGImageProperties.h` | properties / animated_png / heif / proraw / color_sync / thumbnail helpers<br>`ffi::kCGImagePropertyIPTCExtArtworkCircaDateCreated` |
| `kCGImagePropertyIPTCExtArtworkContentDescription` | constant | `CGImageProperties.h` | properties / animated_png / heif / proraw / color_sync / thumbnail helpers<br>`ffi::kCGImagePropertyIPTCExtArtworkContentDescription` |
| `kCGImagePropertyIPTCExtArtworkContributionDescription` | constant | `CGImageProperties.h` | properties / animated_png / heif / proraw / color_sync / thumbnail helpers<br>`ffi::kCGImagePropertyIPTCExtArtworkContributionDescription` |
| `kCGImagePropertyIPTCExtArtworkCopyrightNotice` | constant | `CGImageProperties.h` | properties / animated_png / heif / proraw / color_sync / thumbnail helpers<br>`ffi::kCGImagePropertyIPTCExtArtworkCopyrightNotice` |
| `kCGImagePropertyIPTCExtArtworkCopyrightOwnerID` | constant | `CGImageProperties.h` | properties / animated_png / heif / proraw / color_sync / thumbnail helpers<br>`ffi::kCGImagePropertyIPTCExtArtworkCopyrightOwnerID` |
| `kCGImagePropertyIPTCExtArtworkCopyrightOwnerName` | constant | `CGImageProperties.h` | properties / animated_png / heif / proraw / color_sync / thumbnail helpers<br>`ffi::kCGImagePropertyIPTCExtArtworkCopyrightOwnerName` |
| `kCGImagePropertyIPTCExtArtworkCreator` | constant | `CGImageProperties.h` | properties / animated_png / heif / proraw / color_sync / thumbnail helpers<br>`ffi::kCGImagePropertyIPTCExtArtworkCreator` |
| `kCGImagePropertyIPTCExtArtworkCreatorID` | constant | `CGImageProperties.h` | properties / animated_png / heif / proraw / color_sync / thumbnail helpers<br>`ffi::kCGImagePropertyIPTCExtArtworkCreatorID` |
| `kCGImagePropertyIPTCExtArtworkDateCreated` | constant | `CGImageProperties.h` | properties / animated_png / heif / proraw / color_sync / thumbnail helpers<br>`ffi::kCGImagePropertyIPTCExtArtworkDateCreated` |
| `kCGImagePropertyIPTCExtArtworkLicensorID` | constant | `CGImageProperties.h` | properties / animated_png / heif / proraw / color_sync / thumbnail helpers<br>`ffi::kCGImagePropertyIPTCExtArtworkLicensorID` |
| `kCGImagePropertyIPTCExtArtworkLicensorName` | constant | `CGImageProperties.h` | properties / animated_png / heif / proraw / color_sync / thumbnail helpers<br>`ffi::kCGImagePropertyIPTCExtArtworkLicensorName` |
| `kCGImagePropertyIPTCExtArtworkOrObject` | constant | `CGImageProperties.h` | properties / animated_png / heif / proraw / color_sync / thumbnail helpers<br>`ffi::kCGImagePropertyIPTCExtArtworkOrObject` |
| `kCGImagePropertyIPTCExtArtworkPhysicalDescription` | constant | `CGImageProperties.h` | properties / animated_png / heif / proraw / color_sync / thumbnail helpers<br>`ffi::kCGImagePropertyIPTCExtArtworkPhysicalDescription` |
| `kCGImagePropertyIPTCExtArtworkSource` | constant | `CGImageProperties.h` | properties / animated_png / heif / proraw / color_sync / thumbnail helpers<br>`ffi::kCGImagePropertyIPTCExtArtworkSource` |
| `kCGImagePropertyIPTCExtArtworkSourceInventoryNo` | constant | `CGImageProperties.h` | properties / animated_png / heif / proraw / color_sync / thumbnail helpers<br>`ffi::kCGImagePropertyIPTCExtArtworkSourceInventoryNo` |
| `kCGImagePropertyIPTCExtArtworkSourceInvURL` | constant | `CGImageProperties.h` | properties / animated_png / heif / proraw / color_sync / thumbnail helpers<br>`ffi::kCGImagePropertyIPTCExtArtworkSourceInvURL` |
| `kCGImagePropertyIPTCExtArtworkStylePeriod` | constant | `CGImageProperties.h` | properties / animated_png / heif / proraw / color_sync / thumbnail helpers<br>`ffi::kCGImagePropertyIPTCExtArtworkStylePeriod` |
| `kCGImagePropertyIPTCExtArtworkTitle` | constant | `CGImageProperties.h` | properties / animated_png / heif / proraw / color_sync / thumbnail helpers<br>`ffi::kCGImagePropertyIPTCExtArtworkTitle` |
| `kCGImagePropertyIPTCExtAudioBitrate` | constant | `CGImageProperties.h` | properties / animated_png / heif / proraw / color_sync / thumbnail helpers<br>`ffi::kCGImagePropertyIPTCExtAudioBitrate` |
| `kCGImagePropertyIPTCExtAudioBitrateMode` | constant | `CGImageProperties.h` | properties / animated_png / heif / proraw / color_sync / thumbnail helpers<br>`ffi::kCGImagePropertyIPTCExtAudioBitrateMode` |
| `kCGImagePropertyIPTCExtAudioChannelCount` | constant | `CGImageProperties.h` | properties / animated_png / heif / proraw / color_sync / thumbnail helpers<br>`ffi::kCGImagePropertyIPTCExtAudioChannelCount` |
| `kCGImagePropertyIPTCExtCircaDateCreated` | constant | `CGImageProperties.h` | properties / animated_png / heif / proraw / color_sync / thumbnail helpers<br>`ffi::kCGImagePropertyIPTCExtCircaDateCreated` |
| `kCGImagePropertyIPTCExtContainerFormat` | constant | `CGImageProperties.h` | properties / animated_png / heif / proraw / color_sync / thumbnail helpers<br>`ffi::kCGImagePropertyIPTCExtContainerFormat` |
| `kCGImagePropertyIPTCExtContainerFormatIdentifier` | constant | `CGImageProperties.h` | properties / animated_png / heif / proraw / color_sync / thumbnail helpers<br>`ffi::kCGImagePropertyIPTCExtContainerFormatIdentifier` |
| `kCGImagePropertyIPTCExtContainerFormatName` | constant | `CGImageProperties.h` | properties / animated_png / heif / proraw / color_sync / thumbnail helpers<br>`ffi::kCGImagePropertyIPTCExtContainerFormatName` |
| `kCGImagePropertyIPTCExtContributor` | constant | `CGImageProperties.h` | properties / animated_png / heif / proraw / color_sync / thumbnail helpers<br>`ffi::kCGImagePropertyIPTCExtContributor` |
| `kCGImagePropertyIPTCExtContributorIdentifier` | constant | `CGImageProperties.h` | properties / animated_png / heif / proraw / color_sync / thumbnail helpers<br>`ffi::kCGImagePropertyIPTCExtContributorIdentifier` |
| `kCGImagePropertyIPTCExtContributorName` | constant | `CGImageProperties.h` | properties / animated_png / heif / proraw / color_sync / thumbnail helpers<br>`ffi::kCGImagePropertyIPTCExtContributorName` |
| `kCGImagePropertyIPTCExtContributorRole` | constant | `CGImageProperties.h` | properties / animated_png / heif / proraw / color_sync / thumbnail helpers<br>`ffi::kCGImagePropertyIPTCExtContributorRole` |
| `kCGImagePropertyIPTCExtControlledVocabularyTerm` | constant | `CGImageProperties.h` | properties / animated_png / heif / proraw / color_sync / thumbnail helpers<br>`ffi::kCGImagePropertyIPTCExtControlledVocabularyTerm` |
| `kCGImagePropertyIPTCExtCopyrightYear` | constant | `CGImageProperties.h` | properties / animated_png / heif / proraw / color_sync / thumbnail helpers<br>`ffi::kCGImagePropertyIPTCExtCopyrightYear` |
| `kCGImagePropertyIPTCExtCreator` | constant | `CGImageProperties.h` | properties / animated_png / heif / proraw / color_sync / thumbnail helpers<br>`ffi::kCGImagePropertyIPTCExtCreator` |
| `kCGImagePropertyIPTCExtCreatorIdentifier` | constant | `CGImageProperties.h` | properties / animated_png / heif / proraw / color_sync / thumbnail helpers<br>`ffi::kCGImagePropertyIPTCExtCreatorIdentifier` |
| `kCGImagePropertyIPTCExtCreatorName` | constant | `CGImageProperties.h` | properties / animated_png / heif / proraw / color_sync / thumbnail helpers<br>`ffi::kCGImagePropertyIPTCExtCreatorName` |
| `kCGImagePropertyIPTCExtCreatorRole` | constant | `CGImageProperties.h` | properties / animated_png / heif / proraw / color_sync / thumbnail helpers<br>`ffi::kCGImagePropertyIPTCExtCreatorRole` |
| `kCGImagePropertyIPTCExtDataOnScreen` | constant | `CGImageProperties.h` | properties / animated_png / heif / proraw / color_sync / thumbnail helpers<br>`ffi::kCGImagePropertyIPTCExtDataOnScreen` |
| `kCGImagePropertyIPTCExtDataOnScreenRegion` | constant | `CGImageProperties.h` | properties / animated_png / heif / proraw / color_sync / thumbnail helpers<br>`ffi::kCGImagePropertyIPTCExtDataOnScreenRegion` |
| `kCGImagePropertyIPTCExtDataOnScreenRegionD` | constant | `CGImageProperties.h` | properties / animated_png / heif / proraw / color_sync / thumbnail helpers<br>`ffi::kCGImagePropertyIPTCExtDataOnScreenRegionD` |
| `kCGImagePropertyIPTCExtDataOnScreenRegionH` | constant | `CGImageProperties.h` | properties / animated_png / heif / proraw / color_sync / thumbnail helpers<br>`ffi::kCGImagePropertyIPTCExtDataOnScreenRegionH` |
| `kCGImagePropertyIPTCExtDataOnScreenRegionText` | constant | `CGImageProperties.h` | properties / animated_png / heif / proraw / color_sync / thumbnail helpers<br>`ffi::kCGImagePropertyIPTCExtDataOnScreenRegionText` |
| `kCGImagePropertyIPTCExtDataOnScreenRegionUnit` | constant | `CGImageProperties.h` | properties / animated_png / heif / proraw / color_sync / thumbnail helpers<br>`ffi::kCGImagePropertyIPTCExtDataOnScreenRegionUnit` |
| `kCGImagePropertyIPTCExtDataOnScreenRegionW` | constant | `CGImageProperties.h` | properties / animated_png / heif / proraw / color_sync / thumbnail helpers<br>`ffi::kCGImagePropertyIPTCExtDataOnScreenRegionW` |
| `kCGImagePropertyIPTCExtDataOnScreenRegionX` | constant | `CGImageProperties.h` | properties / animated_png / heif / proraw / color_sync / thumbnail helpers<br>`ffi::kCGImagePropertyIPTCExtDataOnScreenRegionX` |
| `kCGImagePropertyIPTCExtDataOnScreenRegionY` | constant | `CGImageProperties.h` | properties / animated_png / heif / proraw / color_sync / thumbnail helpers<br>`ffi::kCGImagePropertyIPTCExtDataOnScreenRegionY` |
| `kCGImagePropertyIPTCExtDigitalImageGUID` | constant | `CGImageProperties.h` | properties / animated_png / heif / proraw / color_sync / thumbnail helpers<br>`ffi::kCGImagePropertyIPTCExtDigitalImageGUID` |
| `kCGImagePropertyIPTCExtDigitalSourceFileType` | constant | `CGImageProperties.h` | properties / animated_png / heif / proraw / color_sync / thumbnail helpers<br>`ffi::kCGImagePropertyIPTCExtDigitalSourceFileType` |
| `kCGImagePropertyIPTCExtDigitalSourceType` | constant | `CGImageProperties.h` | properties / animated_png / heif / proraw / color_sync / thumbnail helpers<br>`ffi::kCGImagePropertyIPTCExtDigitalSourceType` |
| `kCGImagePropertyIPTCExtDopesheet` | constant | `CGImageProperties.h` | properties / animated_png / heif / proraw / color_sync / thumbnail helpers<br>`ffi::kCGImagePropertyIPTCExtDopesheet` |
| `kCGImagePropertyIPTCExtDopesheetLink` | constant | `CGImageProperties.h` | properties / animated_png / heif / proraw / color_sync / thumbnail helpers<br>`ffi::kCGImagePropertyIPTCExtDopesheetLink` |
| `kCGImagePropertyIPTCExtDopesheetLinkLink` | constant | `CGImageProperties.h` | properties / animated_png / heif / proraw / color_sync / thumbnail helpers<br>`ffi::kCGImagePropertyIPTCExtDopesheetLinkLink` |
| `kCGImagePropertyIPTCExtDopesheetLinkLinkQualifier` | constant | `CGImageProperties.h` | properties / animated_png / heif / proraw / color_sync / thumbnail helpers<br>`ffi::kCGImagePropertyIPTCExtDopesheetLinkLinkQualifier` |
| `kCGImagePropertyIPTCExtEmbdEncRightsExpr` | constant | `CGImageProperties.h` | properties / animated_png / heif / proraw / color_sync / thumbnail helpers<br>`ffi::kCGImagePropertyIPTCExtEmbdEncRightsExpr` |
| `kCGImagePropertyIPTCExtEmbeddedEncodedRightsExpr` | constant | `CGImageProperties.h` | properties / animated_png / heif / proraw / color_sync / thumbnail helpers<br>`ffi::kCGImagePropertyIPTCExtEmbeddedEncodedRightsExpr` |
| `kCGImagePropertyIPTCExtEmbeddedEncodedRightsExprLangID` | constant | `CGImageProperties.h` | properties / animated_png / heif / proraw / color_sync / thumbnail helpers<br>`ffi::kCGImagePropertyIPTCExtEmbeddedEncodedRightsExprLangID` |
| `kCGImagePropertyIPTCExtEmbeddedEncodedRightsExprType` | constant | `CGImageProperties.h` | properties / animated_png / heif / proraw / color_sync / thumbnail helpers<br>`ffi::kCGImagePropertyIPTCExtEmbeddedEncodedRightsExprType` |
| `kCGImagePropertyIPTCExtEpisode` | constant | `CGImageProperties.h` | properties / animated_png / heif / proraw / color_sync / thumbnail helpers<br>`ffi::kCGImagePropertyIPTCExtEpisode` |
| `kCGImagePropertyIPTCExtEpisodeIdentifier` | constant | `CGImageProperties.h` | properties / animated_png / heif / proraw / color_sync / thumbnail helpers<br>`ffi::kCGImagePropertyIPTCExtEpisodeIdentifier` |
| `kCGImagePropertyIPTCExtEpisodeName` | constant | `CGImageProperties.h` | properties / animated_png / heif / proraw / color_sync / thumbnail helpers<br>`ffi::kCGImagePropertyIPTCExtEpisodeName` |
| `kCGImagePropertyIPTCExtEpisodeNumber` | constant | `CGImageProperties.h` | properties / animated_png / heif / proraw / color_sync / thumbnail helpers<br>`ffi::kCGImagePropertyIPTCExtEpisodeNumber` |
| `kCGImagePropertyIPTCExtEvent` | constant | `CGImageProperties.h` | properties / animated_png / heif / proraw / color_sync / thumbnail helpers<br>`ffi::kCGImagePropertyIPTCExtEvent` |
| `kCGImagePropertyIPTCExtExternalMetadataLink` | constant | `CGImageProperties.h` | properties / animated_png / heif / proraw / color_sync / thumbnail helpers<br>`ffi::kCGImagePropertyIPTCExtExternalMetadataLink` |
| `kCGImagePropertyIPTCExtFeedIdentifier` | constant | `CGImageProperties.h` | properties / animated_png / heif / proraw / color_sync / thumbnail helpers<br>`ffi::kCGImagePropertyIPTCExtFeedIdentifier` |
| `kCGImagePropertyIPTCExtGenre` | constant | `CGImageProperties.h` | properties / animated_png / heif / proraw / color_sync / thumbnail helpers<br>`ffi::kCGImagePropertyIPTCExtGenre` |
| `kCGImagePropertyIPTCExtGenreCvId` | constant | `CGImageProperties.h` | properties / animated_png / heif / proraw / color_sync / thumbnail helpers<br>`ffi::kCGImagePropertyIPTCExtGenreCvId` |
| `kCGImagePropertyIPTCExtGenreCvTermId` | constant | `CGImageProperties.h` | properties / animated_png / heif / proraw / color_sync / thumbnail helpers<br>`ffi::kCGImagePropertyIPTCExtGenreCvTermId` |
| `kCGImagePropertyIPTCExtGenreCvTermName` | constant | `CGImageProperties.h` | properties / animated_png / heif / proraw / color_sync / thumbnail helpers<br>`ffi::kCGImagePropertyIPTCExtGenreCvTermName` |
| `kCGImagePropertyIPTCExtGenreCvTermRefinedAbout` | constant | `CGImageProperties.h` | properties / animated_png / heif / proraw / color_sync / thumbnail helpers<br>`ffi::kCGImagePropertyIPTCExtGenreCvTermRefinedAbout` |
| `kCGImagePropertyIPTCExtHeadline` | constant | `CGImageProperties.h` | properties / animated_png / heif / proraw / color_sync / thumbnail helpers<br>`ffi::kCGImagePropertyIPTCExtHeadline` |
| `kCGImagePropertyIPTCExtIPTCLastEdited` | constant | `CGImageProperties.h` | properties / animated_png / heif / proraw / color_sync / thumbnail helpers<br>`ffi::kCGImagePropertyIPTCExtIPTCLastEdited` |
| `kCGImagePropertyIPTCExtLinkedEncodedRightsExpr` | constant | `CGImageProperties.h` | properties / animated_png / heif / proraw / color_sync / thumbnail helpers<br>`ffi::kCGImagePropertyIPTCExtLinkedEncodedRightsExpr` |
| `kCGImagePropertyIPTCExtLinkedEncodedRightsExprLangID` | constant | `CGImageProperties.h` | properties / animated_png / heif / proraw / color_sync / thumbnail helpers<br>`ffi::kCGImagePropertyIPTCExtLinkedEncodedRightsExprLangID` |
| `kCGImagePropertyIPTCExtLinkedEncodedRightsExprType` | constant | `CGImageProperties.h` | properties / animated_png / heif / proraw / color_sync / thumbnail helpers<br>`ffi::kCGImagePropertyIPTCExtLinkedEncodedRightsExprType` |
| `kCGImagePropertyIPTCExtLinkedEncRightsExpr` | constant | `CGImageProperties.h` | properties / animated_png / heif / proraw / color_sync / thumbnail helpers<br>`ffi::kCGImagePropertyIPTCExtLinkedEncRightsExpr` |
| `kCGImagePropertyIPTCExtLocationCity` | constant | `CGImageProperties.h` | properties / animated_png / heif / proraw / color_sync / thumbnail helpers<br>`ffi::kCGImagePropertyIPTCExtLocationCity` |
| `kCGImagePropertyIPTCExtLocationCountryCode` | constant | `CGImageProperties.h` | properties / animated_png / heif / proraw / color_sync / thumbnail helpers<br>`ffi::kCGImagePropertyIPTCExtLocationCountryCode` |
| `kCGImagePropertyIPTCExtLocationCountryName` | constant | `CGImageProperties.h` | properties / animated_png / heif / proraw / color_sync / thumbnail helpers<br>`ffi::kCGImagePropertyIPTCExtLocationCountryName` |
| `kCGImagePropertyIPTCExtLocationCreated` | constant | `CGImageProperties.h` | properties / animated_png / heif / proraw / color_sync / thumbnail helpers<br>`ffi::kCGImagePropertyIPTCExtLocationCreated` |
| `kCGImagePropertyIPTCExtLocationGPSAltitude` | constant | `CGImageProperties.h` | properties / animated_png / heif / proraw / color_sync / thumbnail helpers<br>`ffi::kCGImagePropertyIPTCExtLocationGPSAltitude` |
| `kCGImagePropertyIPTCExtLocationGPSLatitude` | constant | `CGImageProperties.h` | properties / animated_png / heif / proraw / color_sync / thumbnail helpers<br>`ffi::kCGImagePropertyIPTCExtLocationGPSLatitude` |
| `kCGImagePropertyIPTCExtLocationGPSLongitude` | constant | `CGImageProperties.h` | properties / animated_png / heif / proraw / color_sync / thumbnail helpers<br>`ffi::kCGImagePropertyIPTCExtLocationGPSLongitude` |
| `kCGImagePropertyIPTCExtLocationIdentifier` | constant | `CGImageProperties.h` | properties / animated_png / heif / proraw / color_sync / thumbnail helpers<br>`ffi::kCGImagePropertyIPTCExtLocationIdentifier` |
| `kCGImagePropertyIPTCExtLocationLocationId` | constant | `CGImageProperties.h` | properties / animated_png / heif / proraw / color_sync / thumbnail helpers<br>`ffi::kCGImagePropertyIPTCExtLocationLocationId` |
| `kCGImagePropertyIPTCExtLocationLocationName` | constant | `CGImageProperties.h` | properties / animated_png / heif / proraw / color_sync / thumbnail helpers<br>`ffi::kCGImagePropertyIPTCExtLocationLocationName` |
| `kCGImagePropertyIPTCExtLocationProvinceState` | constant | `CGImageProperties.h` | properties / animated_png / heif / proraw / color_sync / thumbnail helpers<br>`ffi::kCGImagePropertyIPTCExtLocationProvinceState` |
| `kCGImagePropertyIPTCExtLocationShown` | constant | `CGImageProperties.h` | properties / animated_png / heif / proraw / color_sync / thumbnail helpers<br>`ffi::kCGImagePropertyIPTCExtLocationShown` |
| `kCGImagePropertyIPTCExtLocationSublocation` | constant | `CGImageProperties.h` | properties / animated_png / heif / proraw / color_sync / thumbnail helpers<br>`ffi::kCGImagePropertyIPTCExtLocationSublocation` |
| `kCGImagePropertyIPTCExtLocationWorldRegion` | constant | `CGImageProperties.h` | properties / animated_png / heif / proraw / color_sync / thumbnail helpers<br>`ffi::kCGImagePropertyIPTCExtLocationWorldRegion` |
| `kCGImagePropertyIPTCExtMaxAvailHeight` | constant | `CGImageProperties.h` | properties / animated_png / heif / proraw / color_sync / thumbnail helpers<br>`ffi::kCGImagePropertyIPTCExtMaxAvailHeight` |
| `kCGImagePropertyIPTCExtMaxAvailWidth` | constant | `CGImageProperties.h` | properties / animated_png / heif / proraw / color_sync / thumbnail helpers<br>`ffi::kCGImagePropertyIPTCExtMaxAvailWidth` |
| `kCGImagePropertyIPTCExtModelAge` | constant | `CGImageProperties.h` | properties / animated_png / heif / proraw / color_sync / thumbnail helpers<br>`ffi::kCGImagePropertyIPTCExtModelAge` |
| `kCGImagePropertyIPTCExtOrganisationInImageCode` | constant | `CGImageProperties.h` | properties / animated_png / heif / proraw / color_sync / thumbnail helpers<br>`ffi::kCGImagePropertyIPTCExtOrganisationInImageCode` |
| `kCGImagePropertyIPTCExtOrganisationInImageName` | constant | `CGImageProperties.h` | properties / animated_png / heif / proraw / color_sync / thumbnail helpers<br>`ffi::kCGImagePropertyIPTCExtOrganisationInImageName` |
| `kCGImagePropertyIPTCExtPersonHeard` | constant | `CGImageProperties.h` | properties / animated_png / heif / proraw / color_sync / thumbnail helpers<br>`ffi::kCGImagePropertyIPTCExtPersonHeard` |
| `kCGImagePropertyIPTCExtPersonHeardIdentifier` | constant | `CGImageProperties.h` | properties / animated_png / heif / proraw / color_sync / thumbnail helpers<br>`ffi::kCGImagePropertyIPTCExtPersonHeardIdentifier` |
| `kCGImagePropertyIPTCExtPersonHeardName` | constant | `CGImageProperties.h` | properties / animated_png / heif / proraw / color_sync / thumbnail helpers<br>`ffi::kCGImagePropertyIPTCExtPersonHeardName` |
| `kCGImagePropertyIPTCExtPersonInImage` | constant | `CGImageProperties.h` | properties / animated_png / heif / proraw / color_sync / thumbnail helpers<br>`ffi::kCGImagePropertyIPTCExtPersonInImage` |
| `kCGImagePropertyIPTCExtPersonInImageCharacteristic` | constant | `CGImageProperties.h` | properties / animated_png / heif / proraw / color_sync / thumbnail helpers<br>`ffi::kCGImagePropertyIPTCExtPersonInImageCharacteristic` |
| `kCGImagePropertyIPTCExtPersonInImageCvTermCvId` | constant | `CGImageProperties.h` | properties / animated_png / heif / proraw / color_sync / thumbnail helpers<br>`ffi::kCGImagePropertyIPTCExtPersonInImageCvTermCvId` |
| `kCGImagePropertyIPTCExtPersonInImageCvTermId` | constant | `CGImageProperties.h` | properties / animated_png / heif / proraw / color_sync / thumbnail helpers<br>`ffi::kCGImagePropertyIPTCExtPersonInImageCvTermId` |
| `kCGImagePropertyIPTCExtPersonInImageCvTermName` | constant | `CGImageProperties.h` | properties / animated_png / heif / proraw / color_sync / thumbnail helpers<br>`ffi::kCGImagePropertyIPTCExtPersonInImageCvTermName` |
| `kCGImagePropertyIPTCExtPersonInImageCvTermRefinedAbout` | constant | `CGImageProperties.h` | properties / animated_png / heif / proraw / color_sync / thumbnail helpers<br>`ffi::kCGImagePropertyIPTCExtPersonInImageCvTermRefinedAbout` |
| `kCGImagePropertyIPTCExtPersonInImageDescription` | constant | `CGImageProperties.h` | properties / animated_png / heif / proraw / color_sync / thumbnail helpers<br>`ffi::kCGImagePropertyIPTCExtPersonInImageDescription` |
| `kCGImagePropertyIPTCExtPersonInImageId` | constant | `CGImageProperties.h` | properties / animated_png / heif / proraw / color_sync / thumbnail helpers<br>`ffi::kCGImagePropertyIPTCExtPersonInImageId` |
| `kCGImagePropertyIPTCExtPersonInImageName` | constant | `CGImageProperties.h` | properties / animated_png / heif / proraw / color_sync / thumbnail helpers<br>`ffi::kCGImagePropertyIPTCExtPersonInImageName` |
| `kCGImagePropertyIPTCExtPersonInImageWDetails` | constant | `CGImageProperties.h` | properties / animated_png / heif / proraw / color_sync / thumbnail helpers<br>`ffi::kCGImagePropertyIPTCExtPersonInImageWDetails` |
| `kCGImagePropertyIPTCExtProductInImage` | constant | `CGImageProperties.h` | properties / animated_png / heif / proraw / color_sync / thumbnail helpers<br>`ffi::kCGImagePropertyIPTCExtProductInImage` |
| `kCGImagePropertyIPTCExtProductInImageDescription` | constant | `CGImageProperties.h` | properties / animated_png / heif / proraw / color_sync / thumbnail helpers<br>`ffi::kCGImagePropertyIPTCExtProductInImageDescription` |
| `kCGImagePropertyIPTCExtProductInImageGTIN` | constant | `CGImageProperties.h` | properties / animated_png / heif / proraw / color_sync / thumbnail helpers<br>`ffi::kCGImagePropertyIPTCExtProductInImageGTIN` |
| `kCGImagePropertyIPTCExtProductInImageName` | constant | `CGImageProperties.h` | properties / animated_png / heif / proraw / color_sync / thumbnail helpers<br>`ffi::kCGImagePropertyIPTCExtProductInImageName` |
| `kCGImagePropertyIPTCExtPublicationEvent` | constant | `CGImageProperties.h` | properties / animated_png / heif / proraw / color_sync / thumbnail helpers<br>`ffi::kCGImagePropertyIPTCExtPublicationEvent` |
| `kCGImagePropertyIPTCExtPublicationEventDate` | constant | `CGImageProperties.h` | properties / animated_png / heif / proraw / color_sync / thumbnail helpers<br>`ffi::kCGImagePropertyIPTCExtPublicationEventDate` |
| `kCGImagePropertyIPTCExtPublicationEventIdentifier` | constant | `CGImageProperties.h` | properties / animated_png / heif / proraw / color_sync / thumbnail helpers<br>`ffi::kCGImagePropertyIPTCExtPublicationEventIdentifier` |
| `kCGImagePropertyIPTCExtPublicationEventName` | constant | `CGImageProperties.h` | properties / animated_png / heif / proraw / color_sync / thumbnail helpers<br>`ffi::kCGImagePropertyIPTCExtPublicationEventName` |
| `kCGImagePropertyIPTCExtRating` | constant | `CGImageProperties.h` | properties / animated_png / heif / proraw / color_sync / thumbnail helpers<br>`ffi::kCGImagePropertyIPTCExtRating` |
| `kCGImagePropertyIPTCExtRatingRatingRegion` | constant | `CGImageProperties.h` | properties / animated_png / heif / proraw / color_sync / thumbnail helpers<br>`ffi::kCGImagePropertyIPTCExtRatingRatingRegion` |
| `kCGImagePropertyIPTCExtRatingRegionCity` | constant | `CGImageProperties.h` | properties / animated_png / heif / proraw / color_sync / thumbnail helpers<br>`ffi::kCGImagePropertyIPTCExtRatingRegionCity` |
| `kCGImagePropertyIPTCExtRatingRegionCountryCode` | constant | `CGImageProperties.h` | properties / animated_png / heif / proraw / color_sync / thumbnail helpers<br>`ffi::kCGImagePropertyIPTCExtRatingRegionCountryCode` |
| `kCGImagePropertyIPTCExtRatingRegionCountryName` | constant | `CGImageProperties.h` | properties / animated_png / heif / proraw / color_sync / thumbnail helpers<br>`ffi::kCGImagePropertyIPTCExtRatingRegionCountryName` |
| `kCGImagePropertyIPTCExtRatingRegionGPSAltitude` | constant | `CGImageProperties.h` | properties / animated_png / heif / proraw / color_sync / thumbnail helpers<br>`ffi::kCGImagePropertyIPTCExtRatingRegionGPSAltitude` |
| `kCGImagePropertyIPTCExtRatingRegionGPSLatitude` | constant | `CGImageProperties.h` | properties / animated_png / heif / proraw / color_sync / thumbnail helpers<br>`ffi::kCGImagePropertyIPTCExtRatingRegionGPSLatitude` |
| `kCGImagePropertyIPTCExtRatingRegionGPSLongitude` | constant | `CGImageProperties.h` | properties / animated_png / heif / proraw / color_sync / thumbnail helpers<br>`ffi::kCGImagePropertyIPTCExtRatingRegionGPSLongitude` |
| `kCGImagePropertyIPTCExtRatingRegionIdentifier` | constant | `CGImageProperties.h` | properties / animated_png / heif / proraw / color_sync / thumbnail helpers<br>`ffi::kCGImagePropertyIPTCExtRatingRegionIdentifier` |
| `kCGImagePropertyIPTCExtRatingRegionLocationId` | constant | `CGImageProperties.h` | properties / animated_png / heif / proraw / color_sync / thumbnail helpers<br>`ffi::kCGImagePropertyIPTCExtRatingRegionLocationId` |
| `kCGImagePropertyIPTCExtRatingRegionLocationName` | constant | `CGImageProperties.h` | properties / animated_png / heif / proraw / color_sync / thumbnail helpers<br>`ffi::kCGImagePropertyIPTCExtRatingRegionLocationName` |
| `kCGImagePropertyIPTCExtRatingRegionProvinceState` | constant | `CGImageProperties.h` | properties / animated_png / heif / proraw / color_sync / thumbnail helpers<br>`ffi::kCGImagePropertyIPTCExtRatingRegionProvinceState` |
| `kCGImagePropertyIPTCExtRatingRegionSublocation` | constant | `CGImageProperties.h` | properties / animated_png / heif / proraw / color_sync / thumbnail helpers<br>`ffi::kCGImagePropertyIPTCExtRatingRegionSublocation` |
| `kCGImagePropertyIPTCExtRatingRegionWorldRegion` | constant | `CGImageProperties.h` | properties / animated_png / heif / proraw / color_sync / thumbnail helpers<br>`ffi::kCGImagePropertyIPTCExtRatingRegionWorldRegion` |
| `kCGImagePropertyIPTCExtRatingScaleMaxValue` | constant | `CGImageProperties.h` | properties / animated_png / heif / proraw / color_sync / thumbnail helpers<br>`ffi::kCGImagePropertyIPTCExtRatingScaleMaxValue` |
| `kCGImagePropertyIPTCExtRatingScaleMinValue` | constant | `CGImageProperties.h` | properties / animated_png / heif / proraw / color_sync / thumbnail helpers<br>`ffi::kCGImagePropertyIPTCExtRatingScaleMinValue` |
| `kCGImagePropertyIPTCExtRatingSourceLink` | constant | `CGImageProperties.h` | properties / animated_png / heif / proraw / color_sync / thumbnail helpers<br>`ffi::kCGImagePropertyIPTCExtRatingSourceLink` |
| `kCGImagePropertyIPTCExtRatingValue` | constant | `CGImageProperties.h` | properties / animated_png / heif / proraw / color_sync / thumbnail helpers<br>`ffi::kCGImagePropertyIPTCExtRatingValue` |
| `kCGImagePropertyIPTCExtRatingValueLogoLink` | constant | `CGImageProperties.h` | properties / animated_png / heif / proraw / color_sync / thumbnail helpers<br>`ffi::kCGImagePropertyIPTCExtRatingValueLogoLink` |
| `kCGImagePropertyIPTCExtRegistryEntryRole` | constant | `CGImageProperties.h` | properties / animated_png / heif / proraw / color_sync / thumbnail helpers<br>`ffi::kCGImagePropertyIPTCExtRegistryEntryRole` |
| `kCGImagePropertyIPTCExtRegistryID` | constant | `CGImageProperties.h` | properties / animated_png / heif / proraw / color_sync / thumbnail helpers<br>`ffi::kCGImagePropertyIPTCExtRegistryID` |
| `kCGImagePropertyIPTCExtRegistryItemID` | constant | `CGImageProperties.h` | properties / animated_png / heif / proraw / color_sync / thumbnail helpers<br>`ffi::kCGImagePropertyIPTCExtRegistryItemID` |
| `kCGImagePropertyIPTCExtRegistryOrganisationID` | constant | `CGImageProperties.h` | properties / animated_png / heif / proraw / color_sync / thumbnail helpers<br>`ffi::kCGImagePropertyIPTCExtRegistryOrganisationID` |
| `kCGImagePropertyIPTCExtReleaseReady` | constant | `CGImageProperties.h` | properties / animated_png / heif / proraw / color_sync / thumbnail helpers<br>`ffi::kCGImagePropertyIPTCExtReleaseReady` |
| `kCGImagePropertyIPTCExtSeason` | constant | `CGImageProperties.h` | properties / animated_png / heif / proraw / color_sync / thumbnail helpers<br>`ffi::kCGImagePropertyIPTCExtSeason` |
| `kCGImagePropertyIPTCExtSeasonIdentifier` | constant | `CGImageProperties.h` | properties / animated_png / heif / proraw / color_sync / thumbnail helpers<br>`ffi::kCGImagePropertyIPTCExtSeasonIdentifier` |
| `kCGImagePropertyIPTCExtSeasonName` | constant | `CGImageProperties.h` | properties / animated_png / heif / proraw / color_sync / thumbnail helpers<br>`ffi::kCGImagePropertyIPTCExtSeasonName` |
| `kCGImagePropertyIPTCExtSeasonNumber` | constant | `CGImageProperties.h` | properties / animated_png / heif / proraw / color_sync / thumbnail helpers<br>`ffi::kCGImagePropertyIPTCExtSeasonNumber` |
| `kCGImagePropertyIPTCExtSeries` | constant | `CGImageProperties.h` | properties / animated_png / heif / proraw / color_sync / thumbnail helpers<br>`ffi::kCGImagePropertyIPTCExtSeries` |
| `kCGImagePropertyIPTCExtSeriesIdentifier` | constant | `CGImageProperties.h` | properties / animated_png / heif / proraw / color_sync / thumbnail helpers<br>`ffi::kCGImagePropertyIPTCExtSeriesIdentifier` |
| `kCGImagePropertyIPTCExtSeriesName` | constant | `CGImageProperties.h` | properties / animated_png / heif / proraw / color_sync / thumbnail helpers<br>`ffi::kCGImagePropertyIPTCExtSeriesName` |
| `kCGImagePropertyIPTCExtShownEvent` | constant | `CGImageProperties.h` | properties / animated_png / heif / proraw / color_sync / thumbnail helpers<br>`ffi::kCGImagePropertyIPTCExtShownEvent` |
| `kCGImagePropertyIPTCExtShownEventIdentifier` | constant | `CGImageProperties.h` | properties / animated_png / heif / proraw / color_sync / thumbnail helpers<br>`ffi::kCGImagePropertyIPTCExtShownEventIdentifier` |
| `kCGImagePropertyIPTCExtShownEventName` | constant | `CGImageProperties.h` | properties / animated_png / heif / proraw / color_sync / thumbnail helpers<br>`ffi::kCGImagePropertyIPTCExtShownEventName` |
| `kCGImagePropertyIPTCExtStorylineIdentifier` | constant | `CGImageProperties.h` | properties / animated_png / heif / proraw / color_sync / thumbnail helpers<br>`ffi::kCGImagePropertyIPTCExtStorylineIdentifier` |
| `kCGImagePropertyIPTCExtStreamReady` | constant | `CGImageProperties.h` | properties / animated_png / heif / proraw / color_sync / thumbnail helpers<br>`ffi::kCGImagePropertyIPTCExtStreamReady` |
| `kCGImagePropertyIPTCExtStylePeriod` | constant | `CGImageProperties.h` | properties / animated_png / heif / proraw / color_sync / thumbnail helpers<br>`ffi::kCGImagePropertyIPTCExtStylePeriod` |
| `kCGImagePropertyIPTCExtSupplyChainSource` | constant | `CGImageProperties.h` | properties / animated_png / heif / proraw / color_sync / thumbnail helpers<br>`ffi::kCGImagePropertyIPTCExtSupplyChainSource` |
| `kCGImagePropertyIPTCExtSupplyChainSourceIdentifier` | constant | `CGImageProperties.h` | properties / animated_png / heif / proraw / color_sync / thumbnail helpers<br>`ffi::kCGImagePropertyIPTCExtSupplyChainSourceIdentifier` |
| `kCGImagePropertyIPTCExtSupplyChainSourceName` | constant | `CGImageProperties.h` | properties / animated_png / heif / proraw / color_sync / thumbnail helpers<br>`ffi::kCGImagePropertyIPTCExtSupplyChainSourceName` |
| `kCGImagePropertyIPTCExtTemporalCoverage` | constant | `CGImageProperties.h` | properties / animated_png / heif / proraw / color_sync / thumbnail helpers<br>`ffi::kCGImagePropertyIPTCExtTemporalCoverage` |
| `kCGImagePropertyIPTCExtTemporalCoverageFrom` | constant | `CGImageProperties.h` | properties / animated_png / heif / proraw / color_sync / thumbnail helpers<br>`ffi::kCGImagePropertyIPTCExtTemporalCoverageFrom` |
| `kCGImagePropertyIPTCExtTemporalCoverageTo` | constant | `CGImageProperties.h` | properties / animated_png / heif / proraw / color_sync / thumbnail helpers<br>`ffi::kCGImagePropertyIPTCExtTemporalCoverageTo` |
| `kCGImagePropertyIPTCExtTranscript` | constant | `CGImageProperties.h` | properties / animated_png / heif / proraw / color_sync / thumbnail helpers<br>`ffi::kCGImagePropertyIPTCExtTranscript` |
| `kCGImagePropertyIPTCExtTranscriptLink` | constant | `CGImageProperties.h` | properties / animated_png / heif / proraw / color_sync / thumbnail helpers<br>`ffi::kCGImagePropertyIPTCExtTranscriptLink` |
| `kCGImagePropertyIPTCExtTranscriptLinkLink` | constant | `CGImageProperties.h` | properties / animated_png / heif / proraw / color_sync / thumbnail helpers<br>`ffi::kCGImagePropertyIPTCExtTranscriptLinkLink` |
| `kCGImagePropertyIPTCExtTranscriptLinkLinkQualifier` | constant | `CGImageProperties.h` | properties / animated_png / heif / proraw / color_sync / thumbnail helpers<br>`ffi::kCGImagePropertyIPTCExtTranscriptLinkLinkQualifier` |
| `kCGImagePropertyIPTCExtVideoBitrate` | constant | `CGImageProperties.h` | properties / animated_png / heif / proraw / color_sync / thumbnail helpers<br>`ffi::kCGImagePropertyIPTCExtVideoBitrate` |
| `kCGImagePropertyIPTCExtVideoBitrateMode` | constant | `CGImageProperties.h` | properties / animated_png / heif / proraw / color_sync / thumbnail helpers<br>`ffi::kCGImagePropertyIPTCExtVideoBitrateMode` |
| `kCGImagePropertyIPTCExtVideoDisplayAspectRatio` | constant | `CGImageProperties.h` | properties / animated_png / heif / proraw / color_sync / thumbnail helpers<br>`ffi::kCGImagePropertyIPTCExtVideoDisplayAspectRatio` |
| `kCGImagePropertyIPTCExtVideoEncodingProfile` | constant | `CGImageProperties.h` | properties / animated_png / heif / proraw / color_sync / thumbnail helpers<br>`ffi::kCGImagePropertyIPTCExtVideoEncodingProfile` |
| `kCGImagePropertyIPTCExtVideoShotType` | constant | `CGImageProperties.h` | properties / animated_png / heif / proraw / color_sync / thumbnail helpers<br>`ffi::kCGImagePropertyIPTCExtVideoShotType` |
| `kCGImagePropertyIPTCExtVideoShotTypeIdentifier` | constant | `CGImageProperties.h` | properties / animated_png / heif / proraw / color_sync / thumbnail helpers<br>`ffi::kCGImagePropertyIPTCExtVideoShotTypeIdentifier` |
| `kCGImagePropertyIPTCExtVideoShotTypeName` | constant | `CGImageProperties.h` | properties / animated_png / heif / proraw / color_sync / thumbnail helpers<br>`ffi::kCGImagePropertyIPTCExtVideoShotTypeName` |
| `kCGImagePropertyIPTCExtVideoStreamsCount` | constant | `CGImageProperties.h` | properties / animated_png / heif / proraw / color_sync / thumbnail helpers<br>`ffi::kCGImagePropertyIPTCExtVideoStreamsCount` |
| `kCGImagePropertyIPTCExtVisualColor` | constant | `CGImageProperties.h` | properties / animated_png / heif / proraw / color_sync / thumbnail helpers<br>`ffi::kCGImagePropertyIPTCExtVisualColor` |
| `kCGImagePropertyIPTCExtWorkflowTag` | constant | `CGImageProperties.h` | properties / animated_png / heif / proraw / color_sync / thumbnail helpers<br>`ffi::kCGImagePropertyIPTCExtWorkflowTag` |
| `kCGImagePropertyIPTCExtWorkflowTagCvId` | constant | `CGImageProperties.h` | properties / animated_png / heif / proraw / color_sync / thumbnail helpers<br>`ffi::kCGImagePropertyIPTCExtWorkflowTagCvId` |
| `kCGImagePropertyIPTCExtWorkflowTagCvTermId` | constant | `CGImageProperties.h` | properties / animated_png / heif / proraw / color_sync / thumbnail helpers<br>`ffi::kCGImagePropertyIPTCExtWorkflowTagCvTermId` |
| `kCGImagePropertyIPTCExtWorkflowTagCvTermName` | constant | `CGImageProperties.h` | properties / animated_png / heif / proraw / color_sync / thumbnail helpers<br>`ffi::kCGImagePropertyIPTCExtWorkflowTagCvTermName` |
| `kCGImagePropertyIPTCExtWorkflowTagCvTermRefinedAbout` | constant | `CGImageProperties.h` | properties / animated_png / heif / proraw / color_sync / thumbnail helpers<br>`ffi::kCGImagePropertyIPTCExtWorkflowTagCvTermRefinedAbout` |
| `kCGImagePropertyIPTCFixtureIdentifier` | constant | `CGImageProperties.h` | properties / animated_png / heif / proraw / color_sync / thumbnail helpers<br>`ffi::kCGImagePropertyIPTCFixtureIdentifier` |
| `kCGImagePropertyIPTCHeadline` | constant | `CGImageProperties.h` | properties / animated_png / heif / proraw / color_sync / thumbnail helpers<br>`ffi::kCGImagePropertyIPTCHeadline` |
| `kCGImagePropertyIPTCImageOrientation` | constant | `CGImageProperties.h` | properties / animated_png / heif / proraw / color_sync / thumbnail helpers<br>`ffi::kCGImagePropertyIPTCImageOrientation` |
| `kCGImagePropertyIPTCImageType` | constant | `CGImageProperties.h` | properties / animated_png / heif / proraw / color_sync / thumbnail helpers<br>`ffi::kCGImagePropertyIPTCImageType` |
| `kCGImagePropertyIPTCKeywords` | constant | `CGImageProperties.h` | properties / animated_png / heif / proraw / color_sync / thumbnail helpers<br>`ffi::kCGImagePropertyIPTCKeywords` |
| `kCGImagePropertyIPTCLanguageIdentifier` | constant | `CGImageProperties.h` | properties / animated_png / heif / proraw / color_sync / thumbnail helpers<br>`ffi::kCGImagePropertyIPTCLanguageIdentifier` |
| `kCGImagePropertyIPTCObjectAttributeReference` | constant | `CGImageProperties.h` | properties / animated_png / heif / proraw / color_sync / thumbnail helpers<br>`ffi::kCGImagePropertyIPTCObjectAttributeReference` |
| `kCGImagePropertyIPTCObjectCycle` | constant | `CGImageProperties.h` | properties / animated_png / heif / proraw / color_sync / thumbnail helpers<br>`ffi::kCGImagePropertyIPTCObjectCycle` |
| `kCGImagePropertyIPTCObjectName` | constant | `CGImageProperties.h` | properties / animated_png / heif / proraw / color_sync / thumbnail helpers<br>`ffi::kCGImagePropertyIPTCObjectName` |
| `kCGImagePropertyIPTCObjectTypeReference` | constant | `CGImageProperties.h` | properties / animated_png / heif / proraw / color_sync / thumbnail helpers<br>`ffi::kCGImagePropertyIPTCObjectTypeReference` |
| `kCGImagePropertyIPTCOriginalTransmissionReference` | constant | `CGImageProperties.h` | properties / animated_png / heif / proraw / color_sync / thumbnail helpers<br>`ffi::kCGImagePropertyIPTCOriginalTransmissionReference` |
| `kCGImagePropertyIPTCOriginatingProgram` | constant | `CGImageProperties.h` | properties / animated_png / heif / proraw / color_sync / thumbnail helpers<br>`ffi::kCGImagePropertyIPTCOriginatingProgram` |
| `kCGImagePropertyIPTCProgramVersion` | constant | `CGImageProperties.h` | properties / animated_png / heif / proraw / color_sync / thumbnail helpers<br>`ffi::kCGImagePropertyIPTCProgramVersion` |
| `kCGImagePropertyIPTCProvinceState` | constant | `CGImageProperties.h` | properties / animated_png / heif / proraw / color_sync / thumbnail helpers<br>`ffi::kCGImagePropertyIPTCProvinceState` |
| `kCGImagePropertyIPTCReferenceDate` | constant | `CGImageProperties.h` | properties / animated_png / heif / proraw / color_sync / thumbnail helpers<br>`ffi::kCGImagePropertyIPTCReferenceDate` |
| `kCGImagePropertyIPTCReferenceNumber` | constant | `CGImageProperties.h` | properties / animated_png / heif / proraw / color_sync / thumbnail helpers<br>`ffi::kCGImagePropertyIPTCReferenceNumber` |
| `kCGImagePropertyIPTCReferenceService` | constant | `CGImageProperties.h` | properties / animated_png / heif / proraw / color_sync / thumbnail helpers<br>`ffi::kCGImagePropertyIPTCReferenceService` |
| `kCGImagePropertyIPTCReleaseDate` | constant | `CGImageProperties.h` | properties / animated_png / heif / proraw / color_sync / thumbnail helpers<br>`ffi::kCGImagePropertyIPTCReleaseDate` |
| `kCGImagePropertyIPTCReleaseTime` | constant | `CGImageProperties.h` | properties / animated_png / heif / proraw / color_sync / thumbnail helpers<br>`ffi::kCGImagePropertyIPTCReleaseTime` |
| `kCGImagePropertyIPTCRightsUsageTerms` | constant | `CGImageProperties.h` | properties / animated_png / heif / proraw / color_sync / thumbnail helpers<br>`ffi::kCGImagePropertyIPTCRightsUsageTerms` |
| `kCGImagePropertyIPTCScene` | constant | `CGImageProperties.h` | properties / animated_png / heif / proraw / color_sync / thumbnail helpers<br>`ffi::kCGImagePropertyIPTCScene` |
| `kCGImagePropertyIPTCSource` | constant | `CGImageProperties.h` | properties / animated_png / heif / proraw / color_sync / thumbnail helpers<br>`ffi::kCGImagePropertyIPTCSource` |
| `kCGImagePropertyIPTCSpecialInstructions` | constant | `CGImageProperties.h` | properties / animated_png / heif / proraw / color_sync / thumbnail helpers<br>`ffi::kCGImagePropertyIPTCSpecialInstructions` |
| `kCGImagePropertyIPTCStarRating` | constant | `CGImageProperties.h` | properties / animated_png / heif / proraw / color_sync / thumbnail helpers<br>`ffi::kCGImagePropertyIPTCStarRating` |
| `kCGImagePropertyIPTCSubjectReference` | constant | `CGImageProperties.h` | properties / animated_png / heif / proraw / color_sync / thumbnail helpers<br>`ffi::kCGImagePropertyIPTCSubjectReference` |
| `kCGImagePropertyIPTCSubLocation` | constant | `CGImageProperties.h` | properties / animated_png / heif / proraw / color_sync / thumbnail helpers<br>`ffi::kCGImagePropertyIPTCSubLocation` |
| `kCGImagePropertyIPTCSupplementalCategory` | constant | `CGImageProperties.h` | properties / animated_png / heif / proraw / color_sync / thumbnail helpers<br>`ffi::kCGImagePropertyIPTCSupplementalCategory` |
| `kCGImagePropertyIPTCTimeCreated` | constant | `CGImageProperties.h` | properties / animated_png / heif / proraw / color_sync / thumbnail helpers<br>`ffi::kCGImagePropertyIPTCTimeCreated` |
| `kCGImagePropertyIPTCUrgency` | constant | `CGImageProperties.h` | properties / animated_png / heif / proraw / color_sync / thumbnail helpers<br>`ffi::kCGImagePropertyIPTCUrgency` |
| `kCGImagePropertyIPTCWriterEditor` | constant | `CGImageProperties.h` | properties / animated_png / heif / proraw / color_sync / thumbnail helpers<br>`ffi::kCGImagePropertyIPTCWriterEditor` |
| `kCGImagePropertyIsFloat` | constant | `CGImageProperties.h` | properties / animated_png / heif / proraw / color_sync / thumbnail helpers<br>`ffi::kCGImagePropertyIsFloat` |
| `kCGImagePropertyIsIndexed` | constant | `CGImageProperties.h` | properties / animated_png / heif / proraw / color_sync / thumbnail helpers<br>`ffi::kCGImagePropertyIsIndexed` |
| `kCGImagePropertyJFIFDensityUnit` | constant | `CGImageProperties.h` | properties / animated_png / heif / proraw / color_sync / thumbnail helpers<br>`ffi::kCGImagePropertyJFIFDensityUnit` |
| `kCGImagePropertyJFIFDictionary` | constant | `CGImageProperties.h` | properties / animated_png / heif / proraw / color_sync / thumbnail helpers<br>`ffi::kCGImagePropertyJFIFDictionary` |
| `kCGImagePropertyJFIFIsProgressive` | constant | `CGImageProperties.h` | properties / animated_png / heif / proraw / color_sync / thumbnail helpers<br>`ffi::kCGImagePropertyJFIFIsProgressive` |
| `kCGImagePropertyJFIFVersion` | constant | `CGImageProperties.h` | properties / animated_png / heif / proraw / color_sync / thumbnail helpers<br>`ffi::kCGImagePropertyJFIFVersion` |
| `kCGImagePropertyJFIFXDensity` | constant | `CGImageProperties.h` | properties / animated_png / heif / proraw / color_sync / thumbnail helpers<br>`ffi::kCGImagePropertyJFIFXDensity` |
| `kCGImagePropertyJFIFYDensity` | constant | `CGImageProperties.h` | properties / animated_png / heif / proraw / color_sync / thumbnail helpers<br>`ffi::kCGImagePropertyJFIFYDensity` |
| `kCGImagePropertyMakerAppleDictionary` | constant | `CGImageProperties.h` | properties / animated_png / heif / proraw / color_sync / thumbnail helpers<br>`ffi::kCGImagePropertyMakerAppleDictionary` |
| `kCGImagePropertyMakerCanonAspectRatioInfo` | constant | `CGImageProperties.h` | properties / animated_png / heif / proraw / color_sync / thumbnail helpers<br>`ffi::kCGImagePropertyMakerCanonAspectRatioInfo` |
| `kCGImagePropertyMakerCanonCameraSerialNumber` | constant | `CGImageProperties.h` | properties / animated_png / heif / proraw / color_sync / thumbnail helpers<br>`ffi::kCGImagePropertyMakerCanonCameraSerialNumber` |
| `kCGImagePropertyMakerCanonContinuousDrive` | constant | `CGImageProperties.h` | properties / animated_png / heif / proraw / color_sync / thumbnail helpers<br>`ffi::kCGImagePropertyMakerCanonContinuousDrive` |
| `kCGImagePropertyMakerCanonDictionary` | constant | `CGImageProperties.h` | properties / animated_png / heif / proraw / color_sync / thumbnail helpers<br>`ffi::kCGImagePropertyMakerCanonDictionary` |
| `kCGImagePropertyMakerCanonFirmware` | constant | `CGImageProperties.h` | properties / animated_png / heif / proraw / color_sync / thumbnail helpers<br>`ffi::kCGImagePropertyMakerCanonFirmware` |
| `kCGImagePropertyMakerCanonFlashExposureComp` | constant | `CGImageProperties.h` | properties / animated_png / heif / proraw / color_sync / thumbnail helpers<br>`ffi::kCGImagePropertyMakerCanonFlashExposureComp` |
| `kCGImagePropertyMakerCanonImageSerialNumber` | constant | `CGImageProperties.h` | properties / animated_png / heif / proraw / color_sync / thumbnail helpers<br>`ffi::kCGImagePropertyMakerCanonImageSerialNumber` |
| `kCGImagePropertyMakerCanonLensModel` | constant | `CGImageProperties.h` | properties / animated_png / heif / proraw / color_sync / thumbnail helpers<br>`ffi::kCGImagePropertyMakerCanonLensModel` |
| `kCGImagePropertyMakerCanonOwnerName` | constant | `CGImageProperties.h` | properties / animated_png / heif / proraw / color_sync / thumbnail helpers<br>`ffi::kCGImagePropertyMakerCanonOwnerName` |
| `kCGImagePropertyMakerFujiDictionary` | constant | `CGImageProperties.h` | properties / animated_png / heif / proraw / color_sync / thumbnail helpers<br>`ffi::kCGImagePropertyMakerFujiDictionary` |
| `kCGImagePropertyMakerMinoltaDictionary` | constant | `CGImageProperties.h` | properties / animated_png / heif / proraw / color_sync / thumbnail helpers<br>`ffi::kCGImagePropertyMakerMinoltaDictionary` |
| `kCGImagePropertyMakerNikonCameraSerialNumber` | constant | `CGImageProperties.h` | properties / animated_png / heif / proraw / color_sync / thumbnail helpers<br>`ffi::kCGImagePropertyMakerNikonCameraSerialNumber` |
| `kCGImagePropertyMakerNikonColorMode` | constant | `CGImageProperties.h` | properties / animated_png / heif / proraw / color_sync / thumbnail helpers<br>`ffi::kCGImagePropertyMakerNikonColorMode` |
| `kCGImagePropertyMakerNikonDictionary` | constant | `CGImageProperties.h` | properties / animated_png / heif / proraw / color_sync / thumbnail helpers<br>`ffi::kCGImagePropertyMakerNikonDictionary` |
| `kCGImagePropertyMakerNikonDigitalZoom` | constant | `CGImageProperties.h` | properties / animated_png / heif / proraw / color_sync / thumbnail helpers<br>`ffi::kCGImagePropertyMakerNikonDigitalZoom` |
| `kCGImagePropertyMakerNikonFlashExposureComp` | constant | `CGImageProperties.h` | properties / animated_png / heif / proraw / color_sync / thumbnail helpers<br>`ffi::kCGImagePropertyMakerNikonFlashExposureComp` |
| `kCGImagePropertyMakerNikonFlashSetting` | constant | `CGImageProperties.h` | properties / animated_png / heif / proraw / color_sync / thumbnail helpers<br>`ffi::kCGImagePropertyMakerNikonFlashSetting` |
| `kCGImagePropertyMakerNikonFocusDistance` | constant | `CGImageProperties.h` | properties / animated_png / heif / proraw / color_sync / thumbnail helpers<br>`ffi::kCGImagePropertyMakerNikonFocusDistance` |
| `kCGImagePropertyMakerNikonFocusMode` | constant | `CGImageProperties.h` | properties / animated_png / heif / proraw / color_sync / thumbnail helpers<br>`ffi::kCGImagePropertyMakerNikonFocusMode` |
| `kCGImagePropertyMakerNikonImageAdjustment` | constant | `CGImageProperties.h` | properties / animated_png / heif / proraw / color_sync / thumbnail helpers<br>`ffi::kCGImagePropertyMakerNikonImageAdjustment` |
| `kCGImagePropertyMakerNikonISOSelection` | constant | `CGImageProperties.h` | properties / animated_png / heif / proraw / color_sync / thumbnail helpers<br>`ffi::kCGImagePropertyMakerNikonISOSelection` |
| `kCGImagePropertyMakerNikonISOSetting` | constant | `CGImageProperties.h` | properties / animated_png / heif / proraw / color_sync / thumbnail helpers<br>`ffi::kCGImagePropertyMakerNikonISOSetting` |
| `kCGImagePropertyMakerNikonLensAdapter` | constant | `CGImageProperties.h` | properties / animated_png / heif / proraw / color_sync / thumbnail helpers<br>`ffi::kCGImagePropertyMakerNikonLensAdapter` |
| `kCGImagePropertyMakerNikonLensInfo` | constant | `CGImageProperties.h` | properties / animated_png / heif / proraw / color_sync / thumbnail helpers<br>`ffi::kCGImagePropertyMakerNikonLensInfo` |
| `kCGImagePropertyMakerNikonLensType` | constant | `CGImageProperties.h` | properties / animated_png / heif / proraw / color_sync / thumbnail helpers<br>`ffi::kCGImagePropertyMakerNikonLensType` |
| `kCGImagePropertyMakerNikonQuality` | constant | `CGImageProperties.h` | properties / animated_png / heif / proraw / color_sync / thumbnail helpers<br>`ffi::kCGImagePropertyMakerNikonQuality` |
| `kCGImagePropertyMakerNikonSharpenMode` | constant | `CGImageProperties.h` | properties / animated_png / heif / proraw / color_sync / thumbnail helpers<br>`ffi::kCGImagePropertyMakerNikonSharpenMode` |
| `kCGImagePropertyMakerNikonShootingMode` | constant | `CGImageProperties.h` | properties / animated_png / heif / proraw / color_sync / thumbnail helpers<br>`ffi::kCGImagePropertyMakerNikonShootingMode` |
| `kCGImagePropertyMakerNikonShutterCount` | constant | `CGImageProperties.h` | properties / animated_png / heif / proraw / color_sync / thumbnail helpers<br>`ffi::kCGImagePropertyMakerNikonShutterCount` |
| `kCGImagePropertyMakerNikonWhiteBalanceMode` | constant | `CGImageProperties.h` | properties / animated_png / heif / proraw / color_sync / thumbnail helpers<br>`ffi::kCGImagePropertyMakerNikonWhiteBalanceMode` |
| `kCGImagePropertyMakerOlympusDictionary` | constant | `CGImageProperties.h` | properties / animated_png / heif / proraw / color_sync / thumbnail helpers<br>`ffi::kCGImagePropertyMakerOlympusDictionary` |
| `kCGImagePropertyMakerPentaxDictionary` | constant | `CGImageProperties.h` | properties / animated_png / heif / proraw / color_sync / thumbnail helpers<br>`ffi::kCGImagePropertyMakerPentaxDictionary` |
| `kCGImagePropertyNamedColorSpace` | constant | `CGImageProperties.h` | properties / animated_png / heif / proraw / color_sync / thumbnail helpers<br>`ffi::kCGImagePropertyNamedColorSpace` |
| `kCGImagePropertyOpenEXRAspectRatio` | constant | `CGImageProperties.h` | properties / animated_png / heif / proraw / color_sync / thumbnail helpers<br>`ffi::kCGImagePropertyOpenEXRAspectRatio` |
| `kCGImagePropertyOpenEXRCompression` | constant | `CGImageProperties.h` | properties / animated_png / heif / proraw / color_sync / thumbnail helpers<br>`ffi::kCGImagePropertyOpenEXRCompression` |
| `kCGImagePropertyOpenEXRDictionary` | constant | `CGImageProperties.h` | properties / animated_png / heif / proraw / color_sync / thumbnail helpers<br>`ffi::kCGImagePropertyOpenEXRDictionary` |
| `kCGImagePropertyOrientation` | constant | `CGImageProperties.h` | properties / animated_png / heif / proraw / color_sync / thumbnail helpers<br>`ffi::kCGImagePropertyOrientation` |
| `kCGImagePropertyPixelFormat` | constant | `CGImageProperties.h` | properties / animated_png / heif / proraw / color_sync / thumbnail helpers<br>`ffi::kCGImagePropertyPixelFormat` |
| `kCGImagePropertyPixelHeight` | constant | `CGImageProperties.h` | properties / animated_png / heif / proraw / color_sync / thumbnail helpers<br>`ffi::kCGImagePropertyPixelHeight` |
| `kCGImagePropertyPixelWidth` | constant | `CGImageProperties.h` | properties / animated_png / heif / proraw / color_sync / thumbnail helpers<br>`ffi::kCGImagePropertyPixelWidth` |
| `kCGImagePropertyPNGAuthor` | constant | `CGImageProperties.h` | properties / animated_png / heif / proraw / color_sync / thumbnail helpers<br>`ffi::kCGImagePropertyPNGAuthor` |
| `kCGImagePropertyPNGChromaticities` | constant | `CGImageProperties.h` | properties / animated_png / heif / proraw / color_sync / thumbnail helpers<br>`ffi::kCGImagePropertyPNGChromaticities` |
| `kCGImagePropertyPNGComment` | constant | `CGImageProperties.h` | properties / animated_png / heif / proraw / color_sync / thumbnail helpers<br>`ffi::kCGImagePropertyPNGComment` |
| `kCGImagePropertyPNGCompressionFilter` | constant | `CGImageProperties.h` | properties / animated_png / heif / proraw / color_sync / thumbnail helpers<br>`ffi::kCGImagePropertyPNGCompressionFilter` |
| `kCGImagePropertyPNGCopyright` | constant | `CGImageProperties.h` | properties / animated_png / heif / proraw / color_sync / thumbnail helpers<br>`ffi::kCGImagePropertyPNGCopyright` |
| `kCGImagePropertyPNGCreationTime` | constant | `CGImageProperties.h` | properties / animated_png / heif / proraw / color_sync / thumbnail helpers<br>`ffi::kCGImagePropertyPNGCreationTime` |
| `kCGImagePropertyPNGDescription` | constant | `CGImageProperties.h` | properties / animated_png / heif / proraw / color_sync / thumbnail helpers<br>`ffi::kCGImagePropertyPNGDescription` |
| `kCGImagePropertyPNGDictionary` | constant | `CGImageProperties.h` | properties / animated_png / heif / proraw / color_sync / thumbnail helpers<br>`ffi::kCGImagePropertyPNGDictionary` |
| `kCGImagePropertyPNGDisclaimer` | constant | `CGImageProperties.h` | properties / animated_png / heif / proraw / color_sync / thumbnail helpers<br>`ffi::kCGImagePropertyPNGDisclaimer` |
| `kCGImagePropertyPNGGamma` | constant | `CGImageProperties.h` | properties / animated_png / heif / proraw / color_sync / thumbnail helpers<br>`ffi::kCGImagePropertyPNGGamma` |
| `kCGImagePropertyPNGInterlaceType` | constant | `CGImageProperties.h` | properties / animated_png / heif / proraw / color_sync / thumbnail helpers<br>`ffi::kCGImagePropertyPNGInterlaceType` |
| `kCGImagePropertyPNGModificationTime` | constant | `CGImageProperties.h` | properties / animated_png / heif / proraw / color_sync / thumbnail helpers<br>`ffi::kCGImagePropertyPNGModificationTime` |
| `kCGImagePropertyPNGPixelsAspectRatio` | constant | `CGImageProperties.h` | properties / animated_png / heif / proraw / color_sync / thumbnail helpers<br>`ffi::kCGImagePropertyPNGPixelsAspectRatio` |
| `kCGImagePropertyPNGSoftware` | constant | `CGImageProperties.h` | properties / animated_png / heif / proraw / color_sync / thumbnail helpers<br>`ffi::kCGImagePropertyPNGSoftware` |
| `kCGImagePropertyPNGSource` | constant | `CGImageProperties.h` | properties / animated_png / heif / proraw / color_sync / thumbnail helpers<br>`ffi::kCGImagePropertyPNGSource` |
| `kCGImagePropertyPNGsRGBIntent` | constant | `CGImageProperties.h` | properties / animated_png / heif / proraw / color_sync / thumbnail helpers<br>`ffi::kCGImagePropertyPNGsRGBIntent` |
| `kCGImagePropertyPNGTitle` | constant | `CGImageProperties.h` | properties / animated_png / heif / proraw / color_sync / thumbnail helpers<br>`ffi::kCGImagePropertyPNGTitle` |
| `kCGImagePropertyPNGTransparency` | constant | `CGImageProperties.h` | properties / animated_png / heif / proraw / color_sync / thumbnail helpers<br>`ffi::kCGImagePropertyPNGTransparency` |
| `kCGImagePropertyPNGWarning` | constant | `CGImageProperties.h` | properties / animated_png / heif / proraw / color_sync / thumbnail helpers<br>`ffi::kCGImagePropertyPNGWarning` |
| `kCGImagePropertyPNGXPixelsPerMeter` | constant | `CGImageProperties.h` | properties / animated_png / heif / proraw / color_sync / thumbnail helpers<br>`ffi::kCGImagePropertyPNGXPixelsPerMeter` |
| `kCGImagePropertyPNGYPixelsPerMeter` | constant | `CGImageProperties.h` | properties / animated_png / heif / proraw / color_sync / thumbnail helpers<br>`ffi::kCGImagePropertyPNGYPixelsPerMeter` |
| `kCGImagePropertyPrimaryImage` | constant | `CGImageProperties.h` | properties / animated_png / heif / proraw / color_sync / thumbnail helpers<br>`ffi::kCGImagePropertyPrimaryImage` |
| `kCGImagePropertyProfileName` | constant | `CGImageProperties.h` | properties / animated_png / heif / proraw / color_sync / thumbnail helpers<br>`ffi::kCGImagePropertyProfileName` |
| `kCGImagePropertyRawDictionary` | constant | `CGImageProperties.h` | properties / animated_png / heif / proraw / color_sync / thumbnail helpers<br>`ffi::kCGImagePropertyRawDictionary` |
| `kCGImagePropertyTGACompression` | constant | `CGImageProperties.h` | properties / animated_png / heif / proraw / color_sync / thumbnail helpers<br>`ffi::kCGImagePropertyTGACompression` |
| `kCGImagePropertyTGADictionary` | constant | `CGImageProperties.h` | properties / animated_png / heif / proraw / color_sync / thumbnail helpers<br>`ffi::kCGImagePropertyTGADictionary` |
| `kCGImagePropertyThumbnailImages` | constant | `CGImageProperties.h` | properties / animated_png / heif / proraw / color_sync / thumbnail helpers<br>`ffi::kCGImagePropertyThumbnailImages` |
| `kCGImagePropertyTIFFArtist` | constant | `CGImageProperties.h` | properties / animated_png / heif / proraw / color_sync / thumbnail helpers<br>`ffi::kCGImagePropertyTIFFArtist` |
| `kCGImagePropertyTIFFCompression` | constant | `CGImageProperties.h` | properties / animated_png / heif / proraw / color_sync / thumbnail helpers<br>`ffi::kCGImagePropertyTIFFCompression` |
| `kCGImagePropertyTIFFCopyright` | constant | `CGImageProperties.h` | properties / animated_png / heif / proraw / color_sync / thumbnail helpers<br>`ffi::kCGImagePropertyTIFFCopyright` |
| `kCGImagePropertyTIFFDateTime` | constant | `CGImageProperties.h` | properties / animated_png / heif / proraw / color_sync / thumbnail helpers<br>`ffi::kCGImagePropertyTIFFDateTime` |
| `kCGImagePropertyTIFFDictionary` | constant | `CGImageProperties.h` | properties / animated_png / heif / proraw / color_sync / thumbnail helpers<br>`ffi::kCGImagePropertyTIFFDictionary` |
| `kCGImagePropertyTIFFDocumentName` | constant | `CGImageProperties.h` | properties / animated_png / heif / proraw / color_sync / thumbnail helpers<br>`ffi::kCGImagePropertyTIFFDocumentName` |
| `kCGImagePropertyTIFFHostComputer` | constant | `CGImageProperties.h` | properties / animated_png / heif / proraw / color_sync / thumbnail helpers<br>`ffi::kCGImagePropertyTIFFHostComputer` |
| `kCGImagePropertyTIFFImageDescription` | constant | `CGImageProperties.h` | properties / animated_png / heif / proraw / color_sync / thumbnail helpers<br>`ffi::kCGImagePropertyTIFFImageDescription` |
| `kCGImagePropertyTIFFMake` | constant | `CGImageProperties.h` | properties / animated_png / heif / proraw / color_sync / thumbnail helpers<br>`ffi::kCGImagePropertyTIFFMake` |
| `kCGImagePropertyTIFFModel` | constant | `CGImageProperties.h` | properties / animated_png / heif / proraw / color_sync / thumbnail helpers<br>`ffi::kCGImagePropertyTIFFModel` |
| `kCGImagePropertyTIFFOrientation` | constant | `CGImageProperties.h` | properties / animated_png / heif / proraw / color_sync / thumbnail helpers<br>`ffi::kCGImagePropertyTIFFOrientation` |
| `kCGImagePropertyTIFFPhotometricInterpretation` | constant | `CGImageProperties.h` | properties / animated_png / heif / proraw / color_sync / thumbnail helpers<br>`ffi::kCGImagePropertyTIFFPhotometricInterpretation` |
| `kCGImagePropertyTIFFPrimaryChromaticities` | constant | `CGImageProperties.h` | properties / animated_png / heif / proraw / color_sync / thumbnail helpers<br>`ffi::kCGImagePropertyTIFFPrimaryChromaticities` |
| `kCGImagePropertyTIFFResolutionUnit` | constant | `CGImageProperties.h` | properties / animated_png / heif / proraw / color_sync / thumbnail helpers<br>`ffi::kCGImagePropertyTIFFResolutionUnit` |
| `kCGImagePropertyTIFFSoftware` | constant | `CGImageProperties.h` | properties / animated_png / heif / proraw / color_sync / thumbnail helpers<br>`ffi::kCGImagePropertyTIFFSoftware` |
| `kCGImagePropertyTIFFTileLength` | constant | `CGImageProperties.h` | properties / animated_png / heif / proraw / color_sync / thumbnail helpers<br>`ffi::kCGImagePropertyTIFFTileLength` |
| `kCGImagePropertyTIFFTileWidth` | constant | `CGImageProperties.h` | properties / animated_png / heif / proraw / color_sync / thumbnail helpers<br>`ffi::kCGImagePropertyTIFFTileWidth` |
| `kCGImagePropertyTIFFTransferFunction` | constant | `CGImageProperties.h` | properties / animated_png / heif / proraw / color_sync / thumbnail helpers<br>`ffi::kCGImagePropertyTIFFTransferFunction` |
| `kCGImagePropertyTIFFWhitePoint` | constant | `CGImageProperties.h` | properties / animated_png / heif / proraw / color_sync / thumbnail helpers<br>`ffi::kCGImagePropertyTIFFWhitePoint` |
| `kCGImagePropertyTIFFXPosition` | constant | `CGImageProperties.h` | properties / animated_png / heif / proraw / color_sync / thumbnail helpers<br>`ffi::kCGImagePropertyTIFFXPosition` |
| `kCGImagePropertyTIFFXResolution` | constant | `CGImageProperties.h` | properties / animated_png / heif / proraw / color_sync / thumbnail helpers<br>`ffi::kCGImagePropertyTIFFXResolution` |
| `kCGImagePropertyTIFFYPosition` | constant | `CGImageProperties.h` | properties / animated_png / heif / proraw / color_sync / thumbnail helpers<br>`ffi::kCGImagePropertyTIFFYPosition` |
| `kCGImagePropertyTIFFYResolution` | constant | `CGImageProperties.h` | properties / animated_png / heif / proraw / color_sync / thumbnail helpers<br>`ffi::kCGImagePropertyTIFFYResolution` |
| `kCGImagePropertyWebPCanvasPixelHeight` | constant | `CGImageProperties.h` | properties / animated_png / heif / proraw / color_sync / thumbnail helpers<br>`ffi::kCGImagePropertyWebPCanvasPixelHeight` |
| `kCGImagePropertyWebPCanvasPixelWidth` | constant | `CGImageProperties.h` | properties / animated_png / heif / proraw / color_sync / thumbnail helpers<br>`ffi::kCGImagePropertyWebPCanvasPixelWidth` |
| `kCGImagePropertyWebPDelayTime` | constant | `CGImageProperties.h` | properties / animated_png / heif / proraw / color_sync / thumbnail helpers<br>`ffi::kCGImagePropertyWebPDelayTime` |
| `kCGImagePropertyWebPDictionary` | constant | `CGImageProperties.h` | properties / animated_png / heif / proraw / color_sync / thumbnail helpers<br>`ffi::kCGImagePropertyWebPDictionary` |
| `kCGImagePropertyWebPFrameInfoArray` | constant | `CGImageProperties.h` | properties / animated_png / heif / proraw / color_sync / thumbnail helpers<br>`ffi::kCGImagePropertyWebPFrameInfoArray` |
| `kCGImagePropertyWebPLoopCount` | constant | `CGImageProperties.h` | properties / animated_png / heif / proraw / color_sync / thumbnail helpers<br>`ffi::kCGImagePropertyWebPLoopCount` |
| `kCGImagePropertyWebPUnclampedDelayTime` | constant | `CGImageProperties.h` | properties / animated_png / heif / proraw / color_sync / thumbnail helpers<br>`ffi::kCGImagePropertyWebPUnclampedDelayTime` |
| `kCGImagePropertyWidth` | constant | `CGImageProperties.h` | properties / animated_png / heif / proraw / color_sync / thumbnail helpers<br>`ffi::kCGImagePropertyWidth` |
| `kCGImageProviderPreferredTileHeight` | constant | `CGImageProperties.h` | properties / animated_png / heif / proraw / color_sync / thumbnail helpers<br>`ffi::kCGImageProviderPreferredTileHeight` |
| `kCGImageProviderPreferredTileWidth` | constant | `CGImageProperties.h` | properties / animated_png / heif / proraw / color_sync / thumbnail helpers<br>`ffi::kCGImageProviderPreferredTileWidth` |
| `kIIOCameraExtrinsics_CoordinateSystemID` | constant | `CGImageProperties.h` | properties / animated_png / heif / proraw / color_sync / thumbnail helpers<br>`ffi::kIIOCameraExtrinsics_CoordinateSystemID` |
| `kIIOCameraExtrinsics_Position` | constant | `CGImageProperties.h` | properties / animated_png / heif / proraw / color_sync / thumbnail helpers<br>`ffi::kIIOCameraExtrinsics_Position` |
| `kIIOCameraExtrinsics_Rotation` | constant | `CGImageProperties.h` | properties / animated_png / heif / proraw / color_sync / thumbnail helpers<br>`ffi::kIIOCameraExtrinsics_Rotation` |
| `kIIOCameraModel_Intrinsics` | constant | `CGImageProperties.h` | properties / animated_png / heif / proraw / color_sync / thumbnail helpers<br>`ffi::kIIOCameraModel_Intrinsics` |
| `kIIOCameraModel_ModelType` | constant | `CGImageProperties.h` | properties / animated_png / heif / proraw / color_sync / thumbnail helpers<br>`ffi::kIIOCameraModel_ModelType` |
| `kIIOCameraModelType_GenericPinhole` | constant | `CGImageProperties.h` | properties / animated_png / heif / proraw / color_sync / thumbnail helpers<br>`ffi::kIIOCameraModelType_GenericPinhole` |
| `kIIOCameraModelType_SimplifiedPinhole` | constant | `CGImageProperties.h` | properties / animated_png / heif / proraw / color_sync / thumbnail helpers<br>`ffi::kIIOCameraModelType_SimplifiedPinhole` |
| `kIIOMetadata_CameraExtrinsicsKey` | constant | `CGImageProperties.h` | properties / animated_png / heif / proraw / color_sync / thumbnail helpers<br>`ffi::kIIOMetadata_CameraExtrinsicsKey` |
| `kIIOMetadata_CameraModelKey` | constant | `CGImageProperties.h` | properties / animated_png / heif / proraw / color_sync / thumbnail helpers<br>`ffi::kIIOMetadata_CameraModelKey` |
| `kIIOMonoscopicImageLocation_Center` | constant | `CGImageProperties.h` | properties / animated_png / heif / proraw / color_sync / thumbnail helpers<br>`ffi::kIIOMonoscopicImageLocation_Center` |
| `kIIOMonoscopicImageLocation_Left` | constant | `CGImageProperties.h` | properties / animated_png / heif / proraw / color_sync / thumbnail helpers<br>`ffi::kIIOMonoscopicImageLocation_Left` |
| `kIIOMonoscopicImageLocation_Right` | constant | `CGImageProperties.h` | properties / animated_png / heif / proraw / color_sync / thumbnail helpers<br>`ffi::kIIOMonoscopicImageLocation_Right` |
| `kIIOMonoscopicImageLocation_Unspecified` | constant | `CGImageProperties.h` | properties / animated_png / heif / proraw / color_sync / thumbnail helpers<br>`ffi::kIIOMonoscopicImageLocation_Unspecified` |
| `kIIOStereoAggressors_Severity` | constant | `CGImageProperties.h` | properties / animated_png / heif / proraw / color_sync / thumbnail helpers<br>`ffi::kIIOStereoAggressors_Severity` |
| `kIIOStereoAggressors_SubTypeURI` | constant | `CGImageProperties.h` | properties / animated_png / heif / proraw / color_sync / thumbnail helpers<br>`ffi::kIIOStereoAggressors_SubTypeURI` |
| `kIIOStereoAggressors_Type` | constant | `CGImageProperties.h` | properties / animated_png / heif / proraw / color_sync / thumbnail helpers<br>`ffi::kIIOStereoAggressors_Type` |

## 🔴 GAPS

| Symbol | Kind | Header | Notes |
| --- | --- | --- | --- |
| `kCFErrorDomainCGImageMetadata` | constant | `CGImageMetadata.h` | Missing from `src/ffi/generated_constants.rs`; no safe equivalent is re-exported. |
| `kCGImageMetadataEnumerateRecursively` | constant | `CGImageMetadata.h` | Missing from `src/ffi/generated_constants.rs`; safe `Metadata::enumerate_tags` does not expose the options constant either. |

## ⏭️ EXEMPT

| Symbol | Kind | Header | Reason | SDK attribute |
| --- | --- | --- | --- | --- |
| `kCGImagePropertyExifSubsecTimeOrginal` | constant | `CGImageProperties.h` | 10.x-deprecated ImageIO property constant; skipped per audit policy. | `IMAGEIO_AVAILABLE_BUT_DEPRECATED(10.4, 10.11, 4.0, 10.0)` |

## Notes

- `cargo test --features raw-ffi` currently stays green despite the two metadata gaps because `tests/api_coverage.rs` only matches constants shaped like `IMAGEIO_EXTERN const CFStringRef k...`; it misses the nullability-annotated `kCGImageMetadataEnumerateRecursively` declaration and the `kCFErrorDomainCGImageMetadata` error-domain constant.
- The lone exempt declaration (`kCGImagePropertyExifSubsecTimeOrginal`) is already present in `imageio::ffi`, but it remains excluded from the score because the SDK marks it deprecated on macOS 10.x.

