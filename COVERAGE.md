# COVERAGE

Audited against `/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.2.sdk/System/Library/Frameworks/ImageIO.framework/Headers` for `imageio` `0.4.1`.

Legend: `✅ implemented`, `🟡 partial`, `⏭️ skipped`.

The default crate surface goes through the Swift bridge and area-focused safe Rust modules. The full audited C row set remains available behind the `raw-ffi` feature as `imageio::ffi`. This macOS SDK audit produced no skipped rows.

## Logical-area summary

| Area | Status | Rust modules | Swift bridge | Example | Test |
| --- | --- | --- | --- | --- | --- |
| Source | ✅ implemented | `source`, `image` | `Source.swift` | `01_source_overview` | `source_tests.rs` |
| Destination | ✅ implemented | `destination`, `image` | `Destination.swift` | `02_destination_roundtrip` | `destination_tests.rs` |
| Properties | ✅ implemented | `properties` | `Properties.swift` | `03_properties_view` | `properties_tests.rs` |
| Metadata | ✅ implemented | `metadata` | `Metadata.swift` | `04_metadata_roundtrip` | `metadata_tests.rs` |
| AuxiliaryData | ✅ implemented | `auxiliary_data` | `AuxiliaryData.swift` | `05_auxiliary_data` | `auxiliary_data_tests.rs` |
| ColorSync | ✅ implemented | `color_sync` | `ColorSync.swift` | `06_color_sync` | `color_sync_tests.rs` |
| AnimatedPNG | ✅ implemented | `animated_png`, `animation` | `AnimatedPNG.swift` | `07_animated_png` | `animated_png_tests.rs` |
| HEIF | ✅ implemented | `heif` | `HEIF.swift` | `08_heif` | `heif_tests.rs` |
| ProRAW | ✅ implemented | `proraw` | `ProRAW.swift` | `09_proraw` | `proraw_tests.rs` |
| Thumbnail | ✅ implemented | `thumbnail` | `Thumbnail.swift` | `10_thumbnail` | `thumbnail_tests.rs` |

## Audited SDK rows

## `CGImageSource.h`

Safe coverage for this header lives in `source`, `image`, `thumbnail`, `auxiliary_data`, and `color_sync`; every row below is also available in `imageio::ffi` with `raw-ffi`.

| Category | Count |
| --- | ---: |
| Types | 2 |
| Functions | 21 |
| Constants | 15 |
| Enum types | 1 |
| Enum cases | 6 |

### Types

| API | Status |
| --- | --- |
| `CGImageSourceRef` | ✅ implemented |
| `CGImageSourceStatus` | ✅ implemented |

### Functions

| API | Status |
| --- | --- |
| `CGImageSourceCopyAuxiliaryDataInfoAtIndex` | ✅ implemented |
| `CGImageSourceCopyMetadataAtIndex` | ✅ implemented |
| `CGImageSourceCopyProperties` | ✅ implemented |
| `CGImageSourceCopyPropertiesAtIndex` | ✅ implemented |
| `CGImageSourceCopyTypeIdentifiers` | ✅ implemented |
| `CGImageSourceCreateImageAtIndex` | ✅ implemented |
| `CGImageSourceCreateIncremental` | ✅ implemented |
| `CGImageSourceCreateThumbnailAtIndex` | ✅ implemented |
| `CGImageSourceCreateWithData` | ✅ implemented |
| `CGImageSourceCreateWithDataProvider` | ✅ implemented |
| `CGImageSourceCreateWithURL` | ✅ implemented |
| `CGImageSourceGetCount` | ✅ implemented |
| `CGImageSourceGetPrimaryImageIndex` | ✅ implemented |
| `CGImageSourceGetStatus` | ✅ implemented |
| `CGImageSourceGetStatusAtIndex` | ✅ implemented |
| `CGImageSourceGetType` | ✅ implemented |
| `CGImageSourceGetTypeID` | ✅ implemented |
| `CGImageSourceRemoveCacheAtIndex` | ✅ implemented |
| `CGImageSourceSetAllowableTypes` | ✅ implemented |
| `CGImageSourceUpdateData` | ✅ implemented |
| `CGImageSourceUpdateDataProvider` | ✅ implemented |

### Enum types

| API | Status |
| --- | --- |
| `CGImageSourceStatus` | ✅ implemented |

### Enum cases

| API | Status |
| --- | --- |
| `kCGImageStatusUnexpectedEOF` | ✅ implemented |
| `kCGImageStatusInvalidData` | ✅ implemented |
| `kCGImageStatusUnknownType` | ✅ implemented |
| `kCGImageStatusReadingHeader` | ✅ implemented |
| `kCGImageStatusIncomplete` | ✅ implemented |
| `kCGImageStatusComplete` | ✅ implemented |

### Constants

| API | Status |
| --- | --- |
| `kCGComputeHDRStats` | ✅ implemented |
| `kCGImageSourceCreateThumbnailFromImageAlways` | ✅ implemented |
| `kCGImageSourceCreateThumbnailFromImageIfAbsent` | ✅ implemented |
| `kCGImageSourceCreateThumbnailWithTransform` | ✅ implemented |
| `kCGImageSourceDecodeRequest` | ✅ implemented |
| `kCGImageSourceDecodeRequestOptions` | ✅ implemented |
| `kCGImageSourceDecodeToHDR` | ✅ implemented |
| `kCGImageSourceDecodeToSDR` | ✅ implemented |
| `kCGImageSourceGenerateImageSpecificLumaScaling` | ✅ implemented |
| `kCGImageSourceShouldAllowFloat` | ✅ implemented |
| `kCGImageSourceShouldCache` | ✅ implemented |
| `kCGImageSourceShouldCacheImmediately` | ✅ implemented |
| `kCGImageSourceSubsampleFactor` | ✅ implemented |
| `kCGImageSourceThumbnailMaxPixelSize` | ✅ implemented |
| `kCGImageSourceTypeIdentifierHint` | ✅ implemented |

## `CGImageDestination.h`

Safe coverage for this header lives in `destination`, `image`, `auxiliary_data`, `heif`, `animated_png`, `proraw`, and `color_sync`; every row below is also available in `imageio::ffi` with `raw-ffi`.

| Category | Count |
| --- | ---: |
| Types | 1 |
| Functions | 12 |
| Constants | 34 |
| Enum types | 0 |
| Enum cases | 0 |

### Types

| API | Status |
| --- | --- |
| `CGImageDestinationRef` | ✅ implemented |

### Functions

| API | Status |
| --- | --- |
| `CGImageDestinationAddAuxiliaryDataInfo` | ✅ implemented |
| `CGImageDestinationAddImage` | ✅ implemented |
| `CGImageDestinationAddImageAndMetadata` | ✅ implemented |
| `CGImageDestinationAddImageFromSource` | ✅ implemented |
| `CGImageDestinationCopyImageSource` | ✅ implemented |
| `CGImageDestinationCopyTypeIdentifiers` | ✅ implemented |
| `CGImageDestinationCreateWithData` | ✅ implemented |
| `CGImageDestinationCreateWithDataConsumer` | ✅ implemented |
| `CGImageDestinationCreateWithURL` | ✅ implemented |
| `CGImageDestinationFinalize` | ✅ implemented |
| `CGImageDestinationGetTypeID` | ✅ implemented |
| `CGImageDestinationSetProperties` | ✅ implemented |

### Constants

| API | Status |
| --- | --- |
| `kCGImageDestinationBackgroundColor` | ✅ implemented |
| `kCGImageDestinationDateTime` | ✅ implemented |
| `kCGImageDestinationEmbedThumbnail` | ✅ implemented |
| `kCGImageDestinationEncodeAlternateColorSpace` | ✅ implemented |
| `kCGImageDestinationEncodeBaseColorSpace` | ✅ implemented |
| `kCGImageDestinationEncodeBaseIsSDR` | ✅ implemented |
| `kCGImageDestinationEncodeBasePixelFormatRequest` | ✅ implemented |
| `kCGImageDestinationEncodeGainMapPixelFormatRequest` | ✅ implemented |
| `kCGImageDestinationEncodeGainMapSubsampleFactor` | ✅ implemented |
| `kCGImageDestinationEncodeGenerateGainMapWithBaseImage` | ✅ implemented |
| `kCGImageDestinationEncodeIsBaseImage` | ✅ implemented |
| `kCGImageDestinationEncodeRequest` | ✅ implemented |
| `kCGImageDestinationEncodeRequestOptions` | ✅ implemented |
| `kCGImageDestinationEncodeToISOGainmap` | ✅ implemented |
| `kCGImageDestinationEncodeToISOHDR` | ✅ implemented |
| `kCGImageDestinationEncodeToSDR` | ✅ implemented |
| `kCGImageDestinationEncodeTonemapMode` | ✅ implemented |
| `kCGImageDestinationImageMaxPixelSize` | ✅ implemented |
| `kCGImageDestinationLossyCompressionQuality` | ✅ implemented |
| `kCGImageDestinationMergeMetadata` | ✅ implemented |
| `kCGImageDestinationMetadata` | ✅ implemented |
| `kCGImageDestinationOptimizeColorForSharing` | ✅ implemented |
| `kCGImageDestinationOrientation` | ✅ implemented |
| `kCGImageDestinationPreserveGainMap` | ✅ implemented |
| `kCGImageMetadataShouldExcludeGPS` | ✅ implemented |
| `kCGImageMetadataShouldExcludeXMP` | ✅ implemented |
| `kCGImagePropertyASTCBlockSize` | ✅ implemented |
| `kCGImagePropertyASTCBlockSize4x4` | ✅ implemented |
| `kCGImagePropertyASTCBlockSize8x8` | ✅ implemented |
| `kCGImagePropertyASTCEncoder` | ✅ implemented |
| `kCGImagePropertyBCEncoder` | ✅ implemented |
| `kCGImagePropertyBCFormat` | ✅ implemented |
| `kCGImagePropertyEncoder` | ✅ implemented |
| `kCGImagePropertyPVREncoder` | ✅ implemented |

## `CGImageAnimation.h`

Safe coverage for this header lives in `animation` and `animated_png`; every row below is also available in `imageio::ffi` with `raw-ffi`.

| Category | Count |
| --- | ---: |
| Types | 2 |
| Functions | 2 |
| Constants | 3 |
| Enum types | 1 |
| Enum cases | 5 |

### Types

| API | Status |
| --- | --- |
| `CGImageAnimationStatus` | ✅ implemented |
| `CGImageSourceAnimationBlock` | ✅ implemented |

### Functions

| API | Status |
| --- | --- |
| `CGAnimateImageAtURLWithBlock` | ✅ implemented |
| `CGAnimateImageDataWithBlock` | ✅ implemented |

### Enum types

| API | Status |
| --- | --- |
| `CGImageAnimationStatus` | ✅ implemented |

### Enum cases

| API | Status |
| --- | --- |
| `kCGImageAnimationStatus_ParameterError` | ✅ implemented |
| `kCGImageAnimationStatus_CorruptInputImage` | ✅ implemented |
| `kCGImageAnimationStatus_UnsupportedFormat` | ✅ implemented |
| `kCGImageAnimationStatus_IncompleteInputImage` | ✅ implemented |
| `kCGImageAnimationStatus_AllocationFailure` | ✅ implemented |

### Constants

| API | Status |
| --- | --- |
| `kCGImageAnimationDelayTime` | ✅ implemented |
| `kCGImageAnimationLoopCount` | ✅ implemented |
| `kCGImageAnimationStartIndex` | ✅ implemented |

## `CGImageMetadata.h`

Safe coverage for this header lives in `metadata`; every row below is also available in `imageio::ffi` with `raw-ffi`.

| Category | Count |
| --- | ---: |
| Types | 6 |
| Functions | 23 |
| Constants | 20 |
| Enum types | 2 |
| Enum cases | 13 |

### Types

| API | Status |
| --- | --- |
| `CGImageMetadataRef` | ✅ implemented |
| `CGMutableImageMetadataRef` | ✅ implemented |
| `CGImageMetadataTagRef` | ✅ implemented |
| `CGImageMetadataType` | ✅ implemented |
| `CGImageMetadataErrors` | ✅ implemented |
| `CGImageMetadataTagBlock` | ✅ implemented |

### Functions

| API | Status |
| --- | --- |
| `CGImageMetadataCopyStringValueWithPath` | ✅ implemented |
| `CGImageMetadataCopyTagMatchingImageProperty` | ✅ implemented |
| `CGImageMetadataCopyTagWithPath` | ✅ implemented |
| `CGImageMetadataCopyTags` | ✅ implemented |
| `CGImageMetadataCreateFromXMPData` | ✅ implemented |
| `CGImageMetadataCreateMutable` | ✅ implemented |
| `CGImageMetadataCreateMutableCopy` | ✅ implemented |
| `CGImageMetadataCreateXMPData` | ✅ implemented |
| `CGImageMetadataEnumerateTagsUsingBlock` | ✅ implemented |
| `CGImageMetadataGetTypeID` | ✅ implemented |
| `CGImageMetadataRegisterNamespaceForPrefix` | ✅ implemented |
| `CGImageMetadataRemoveTagWithPath` | ✅ implemented |
| `CGImageMetadataSetTagWithPath` | ✅ implemented |
| `CGImageMetadataSetValueMatchingImageProperty` | ✅ implemented |
| `CGImageMetadataSetValueWithPath` | ✅ implemented |
| `CGImageMetadataTagCopyName` | ✅ implemented |
| `CGImageMetadataTagCopyNamespace` | ✅ implemented |
| `CGImageMetadataTagCopyPrefix` | ✅ implemented |
| `CGImageMetadataTagCopyQualifiers` | ✅ implemented |
| `CGImageMetadataTagCopyValue` | ✅ implemented |
| `CGImageMetadataTagCreate` | ✅ implemented |
| `CGImageMetadataTagGetType` | ✅ implemented |
| `CGImageMetadataTagGetTypeID` | ✅ implemented |

### Enum types

| API | Status |
| --- | --- |
| `CGImageMetadataErrors` | ✅ implemented |
| `CGImageMetadataType` | ✅ implemented |

### Enum cases

| API | Status |
| --- | --- |
| `kCGImageMetadataErrorUnknown` | ✅ implemented |
| `kCGImageMetadataErrorUnsupportedFormat` | ✅ implemented |
| `kCGImageMetadataErrorBadArgument` | ✅ implemented |
| `kCGImageMetadataErrorConflictingArguments` | ✅ implemented |
| `kCGImageMetadataErrorPrefixConflict` | ✅ implemented |
| `kCGImageMetadataTypeInvalid` | ✅ implemented |
| `kCGImageMetadataTypeDefault` | ✅ implemented |
| `kCGImageMetadataTypeString` | ✅ implemented |
| `kCGImageMetadataTypeArrayUnordered` | ✅ implemented |
| `kCGImageMetadataTypeArrayOrdered` | ✅ implemented |
| `kCGImageMetadataTypeAlternateArray` | ✅ implemented |
| `kCGImageMetadataTypeAlternateText` | ✅ implemented |
| `kCGImageMetadataTypeStructure` | ✅ implemented |

### Constants

| API | Status |
| --- | --- |
| `kCGImageMetadataNamespaceDublinCore` | ✅ implemented |
| `kCGImageMetadataNamespaceExif` | ✅ implemented |
| `kCGImageMetadataNamespaceExifAux` | ✅ implemented |
| `kCGImageMetadataNamespaceExifEX` | ✅ implemented |
| `kCGImageMetadataNamespaceIPTCCore` | ✅ implemented |
| `kCGImageMetadataNamespaceIPTCExtension` | ✅ implemented |
| `kCGImageMetadataNamespacePhotoshop` | ✅ implemented |
| `kCGImageMetadataNamespaceTIFF` | ✅ implemented |
| `kCGImageMetadataNamespaceXMPBasic` | ✅ implemented |
| `kCGImageMetadataNamespaceXMPRights` | ✅ implemented |
| `kCGImageMetadataPrefixDublinCore` | ✅ implemented |
| `kCGImageMetadataPrefixExif` | ✅ implemented |
| `kCGImageMetadataPrefixExifAux` | ✅ implemented |
| `kCGImageMetadataPrefixExifEX` | ✅ implemented |
| `kCGImageMetadataPrefixIPTCCore` | ✅ implemented |
| `kCGImageMetadataPrefixIPTCExtension` | ✅ implemented |
| `kCGImageMetadataPrefixPhotoshop` | ✅ implemented |
| `kCGImageMetadataPrefixTIFF` | ✅ implemented |
| `kCGImageMetadataPrefixXMPBasic` | ✅ implemented |
| `kCGImageMetadataPrefixXMPRights` | ✅ implemented |

## `CGImageProperties.h`

Safe coverage for this header lives in `properties`, `animated_png`, `heif`, `proraw`, `color_sync`, and `thumbnail`; every row below is also available in `imageio::ffi` with `raw-ffi`.

| Category | Count |
| --- | ---: |
| Types | 0 |
| Functions | 0 |
| Constants | 676 |
| Enum types | 0 |
| Enum cases | 0 |

### Constants

| API | Status |
| --- | --- |
| `kCGImageAuxiliaryDataInfoColorSpace` | ✅ implemented |
| `kCGImageAuxiliaryDataInfoData` | ✅ implemented |
| `kCGImageAuxiliaryDataInfoDataDescription` | ✅ implemented |
| `kCGImageAuxiliaryDataInfoMetadata` | ✅ implemented |
| `kCGImageAuxiliaryDataTypeDepth` | ✅ implemented |
| `kCGImageAuxiliaryDataTypeDisparity` | ✅ implemented |
| `kCGImageAuxiliaryDataTypeHDRGainMap` | ✅ implemented |
| `kCGImageAuxiliaryDataTypeISOGainMap` | ✅ implemented |
| `kCGImageAuxiliaryDataTypePortraitEffectsMatte` | ✅ implemented |
| `kCGImageAuxiliaryDataTypeSemanticSegmentationGlassesMatte` | ✅ implemented |
| `kCGImageAuxiliaryDataTypeSemanticSegmentationHairMatte` | ✅ implemented |
| `kCGImageAuxiliaryDataTypeSemanticSegmentationSkinMatte` | ✅ implemented |
| `kCGImageAuxiliaryDataTypeSemanticSegmentationSkyMatte` | ✅ implemented |
| `kCGImageAuxiliaryDataTypeSemanticSegmentationTeethMatte` | ✅ implemented |
| `kCGImageProperty8BIMDictionary` | ✅ implemented |
| `kCGImageProperty8BIMLayerNames` | ✅ implemented |
| `kCGImageProperty8BIMVersion` | ✅ implemented |
| `kCGImagePropertyAPNGCanvasPixelHeight` | ✅ implemented |
| `kCGImagePropertyAPNGCanvasPixelWidth` | ✅ implemented |
| `kCGImagePropertyAPNGDelayTime` | ✅ implemented |
| `kCGImagePropertyAPNGFrameInfoArray` | ✅ implemented |
| `kCGImagePropertyAPNGLoopCount` | ✅ implemented |
| `kCGImagePropertyAPNGUnclampedDelayTime` | ✅ implemented |
| `kCGImagePropertyAVISDictionary` | ✅ implemented |
| `kCGImagePropertyAuxiliaryData` | ✅ implemented |
| `kCGImagePropertyAuxiliaryDataType` | ✅ implemented |
| `kCGImagePropertyBytesPerRow` | ✅ implemented |
| `kCGImagePropertyCIFFCameraSerialNumber` | ✅ implemented |
| `kCGImagePropertyCIFFContinuousDrive` | ✅ implemented |
| `kCGImagePropertyCIFFDescription` | ✅ implemented |
| `kCGImagePropertyCIFFDictionary` | ✅ implemented |
| `kCGImagePropertyCIFFFirmware` | ✅ implemented |
| `kCGImagePropertyCIFFFlashExposureComp` | ✅ implemented |
| `kCGImagePropertyCIFFFocusMode` | ✅ implemented |
| `kCGImagePropertyCIFFImageFileName` | ✅ implemented |
| `kCGImagePropertyCIFFImageName` | ✅ implemented |
| `kCGImagePropertyCIFFImageSerialNumber` | ✅ implemented |
| `kCGImagePropertyCIFFLensMaxMM` | ✅ implemented |
| `kCGImagePropertyCIFFLensMinMM` | ✅ implemented |
| `kCGImagePropertyCIFFLensModel` | ✅ implemented |
| `kCGImagePropertyCIFFMeasuredEV` | ✅ implemented |
| `kCGImagePropertyCIFFMeteringMode` | ✅ implemented |
| `kCGImagePropertyCIFFOwnerName` | ✅ implemented |
| `kCGImagePropertyCIFFRecordID` | ✅ implemented |
| `kCGImagePropertyCIFFReleaseMethod` | ✅ implemented |
| `kCGImagePropertyCIFFReleaseTiming` | ✅ implemented |
| `kCGImagePropertyCIFFSelfTimingTime` | ✅ implemented |
| `kCGImagePropertyCIFFShootingMode` | ✅ implemented |
| `kCGImagePropertyCIFFWhiteBalanceIndex` | ✅ implemented |
| `kCGImagePropertyColorModel` | ✅ implemented |
| `kCGImagePropertyColorModelCMYK` | ✅ implemented |
| `kCGImagePropertyColorModelGray` | ✅ implemented |
| `kCGImagePropertyColorModelLab` | ✅ implemented |
| `kCGImagePropertyColorModelRGB` | ✅ implemented |
| `kCGImagePropertyDNGActiveArea` | ✅ implemented |
| `kCGImagePropertyDNGAnalogBalance` | ✅ implemented |
| `kCGImagePropertyDNGAntiAliasStrength` | ✅ implemented |
| `kCGImagePropertyDNGAsShotICCProfile` | ✅ implemented |
| `kCGImagePropertyDNGAsShotNeutral` | ✅ implemented |
| `kCGImagePropertyDNGAsShotPreProfileMatrix` | ✅ implemented |
| `kCGImagePropertyDNGAsShotProfileName` | ✅ implemented |
| `kCGImagePropertyDNGAsShotWhiteXY` | ✅ implemented |
| `kCGImagePropertyDNGBackwardVersion` | ✅ implemented |
| `kCGImagePropertyDNGBaselineExposure` | ✅ implemented |
| `kCGImagePropertyDNGBaselineExposureOffset` | ✅ implemented |
| `kCGImagePropertyDNGBaselineNoise` | ✅ implemented |
| `kCGImagePropertyDNGBaselineSharpness` | ✅ implemented |
| `kCGImagePropertyDNGBayerGreenSplit` | ✅ implemented |
| `kCGImagePropertyDNGBestQualityScale` | ✅ implemented |
| `kCGImagePropertyDNGBlackLevel` | ✅ implemented |
| `kCGImagePropertyDNGBlackLevelDeltaH` | ✅ implemented |
| `kCGImagePropertyDNGBlackLevelDeltaV` | ✅ implemented |
| `kCGImagePropertyDNGBlackLevelRepeatDim` | ✅ implemented |
| `kCGImagePropertyDNGCFALayout` | ✅ implemented |
| `kCGImagePropertyDNGCFAPlaneColor` | ✅ implemented |
| `kCGImagePropertyDNGCalibrationIlluminant1` | ✅ implemented |
| `kCGImagePropertyDNGCalibrationIlluminant2` | ✅ implemented |
| `kCGImagePropertyDNGCameraCalibration1` | ✅ implemented |
| `kCGImagePropertyDNGCameraCalibration2` | ✅ implemented |
| `kCGImagePropertyDNGCameraCalibrationSignature` | ✅ implemented |
| `kCGImagePropertyDNGCameraSerialNumber` | ✅ implemented |
| `kCGImagePropertyDNGChromaBlurRadius` | ✅ implemented |
| `kCGImagePropertyDNGColorMatrix1` | ✅ implemented |
| `kCGImagePropertyDNGColorMatrix2` | ✅ implemented |
| `kCGImagePropertyDNGColorimetricReference` | ✅ implemented |
| `kCGImagePropertyDNGCurrentICCProfile` | ✅ implemented |
| `kCGImagePropertyDNGCurrentPreProfileMatrix` | ✅ implemented |
| `kCGImagePropertyDNGDefaultBlackRender` | ✅ implemented |
| `kCGImagePropertyDNGDefaultCropOrigin` | ✅ implemented |
| `kCGImagePropertyDNGDefaultCropSize` | ✅ implemented |
| `kCGImagePropertyDNGDefaultScale` | ✅ implemented |
| `kCGImagePropertyDNGDefaultUserCrop` | ✅ implemented |
| `kCGImagePropertyDNGDictionary` | ✅ implemented |
| `kCGImagePropertyDNGExtraCameraProfiles` | ✅ implemented |
| `kCGImagePropertyDNGFixVignetteRadial` | ✅ implemented |
| `kCGImagePropertyDNGForwardMatrix1` | ✅ implemented |
| `kCGImagePropertyDNGForwardMatrix2` | ✅ implemented |
| `kCGImagePropertyDNGLensInfo` | ✅ implemented |
| `kCGImagePropertyDNGLinearResponseLimit` | ✅ implemented |
| `kCGImagePropertyDNGLinearizationTable` | ✅ implemented |
| `kCGImagePropertyDNGLocalizedCameraModel` | ✅ implemented |
| `kCGImagePropertyDNGMakerNoteSafety` | ✅ implemented |
| `kCGImagePropertyDNGMaskedAreas` | ✅ implemented |
| `kCGImagePropertyDNGNewRawImageDigest` | ✅ implemented |
| `kCGImagePropertyDNGNoiseProfile` | ✅ implemented |
| `kCGImagePropertyDNGNoiseReductionApplied` | ✅ implemented |
| `kCGImagePropertyDNGOpcodeList1` | ✅ implemented |
| `kCGImagePropertyDNGOpcodeList2` | ✅ implemented |
| `kCGImagePropertyDNGOpcodeList3` | ✅ implemented |
| `kCGImagePropertyDNGOriginalBestQualityFinalSize` | ✅ implemented |
| `kCGImagePropertyDNGOriginalDefaultCropSize` | ✅ implemented |
| `kCGImagePropertyDNGOriginalDefaultFinalSize` | ✅ implemented |
| `kCGImagePropertyDNGOriginalRawFileData` | ✅ implemented |
| `kCGImagePropertyDNGOriginalRawFileDigest` | ✅ implemented |
| `kCGImagePropertyDNGOriginalRawFileName` | ✅ implemented |
| `kCGImagePropertyDNGPreviewApplicationName` | ✅ implemented |
| `kCGImagePropertyDNGPreviewApplicationVersion` | ✅ implemented |
| `kCGImagePropertyDNGPreviewColorSpace` | ✅ implemented |
| `kCGImagePropertyDNGPreviewDateTime` | ✅ implemented |
| `kCGImagePropertyDNGPreviewSettingsDigest` | ✅ implemented |
| `kCGImagePropertyDNGPreviewSettingsName` | ✅ implemented |
| `kCGImagePropertyDNGPrivateData` | ✅ implemented |
| `kCGImagePropertyDNGProfileCalibrationSignature` | ✅ implemented |
| `kCGImagePropertyDNGProfileCopyright` | ✅ implemented |
| `kCGImagePropertyDNGProfileEmbedPolicy` | ✅ implemented |
| `kCGImagePropertyDNGProfileHueSatMapData1` | ✅ implemented |
| `kCGImagePropertyDNGProfileHueSatMapData2` | ✅ implemented |
| `kCGImagePropertyDNGProfileHueSatMapDims` | ✅ implemented |
| `kCGImagePropertyDNGProfileHueSatMapEncoding` | ✅ implemented |
| `kCGImagePropertyDNGProfileLookTableData` | ✅ implemented |
| `kCGImagePropertyDNGProfileLookTableDims` | ✅ implemented |
| `kCGImagePropertyDNGProfileLookTableEncoding` | ✅ implemented |
| `kCGImagePropertyDNGProfileName` | ✅ implemented |
| `kCGImagePropertyDNGProfileToneCurve` | ✅ implemented |
| `kCGImagePropertyDNGRawDataUniqueID` | ✅ implemented |
| `kCGImagePropertyDNGRawImageDigest` | ✅ implemented |
| `kCGImagePropertyDNGRawToPreviewGain` | ✅ implemented |
| `kCGImagePropertyDNGReductionMatrix1` | ✅ implemented |
| `kCGImagePropertyDNGReductionMatrix2` | ✅ implemented |
| `kCGImagePropertyDNGRowInterleaveFactor` | ✅ implemented |
| `kCGImagePropertyDNGShadowScale` | ✅ implemented |
| `kCGImagePropertyDNGSubTileBlockSize` | ✅ implemented |
| `kCGImagePropertyDNGUniqueCameraModel` | ✅ implemented |
| `kCGImagePropertyDNGVersion` | ✅ implemented |
| `kCGImagePropertyDNGWarpFisheye` | ✅ implemented |
| `kCGImagePropertyDNGWarpRectilinear` | ✅ implemented |
| `kCGImagePropertyDNGWhiteLevel` | ✅ implemented |
| `kCGImagePropertyDPIHeight` | ✅ implemented |
| `kCGImagePropertyDPIWidth` | ✅ implemented |
| `kCGImagePropertyDepth` | ✅ implemented |
| `kCGImagePropertyExifApertureValue` | ✅ implemented |
| `kCGImagePropertyExifAuxDictionary` | ✅ implemented |
| `kCGImagePropertyExifAuxFirmware` | ✅ implemented |
| `kCGImagePropertyExifAuxFlashCompensation` | ✅ implemented |
| `kCGImagePropertyExifAuxImageNumber` | ✅ implemented |
| `kCGImagePropertyExifAuxLensID` | ✅ implemented |
| `kCGImagePropertyExifAuxLensInfo` | ✅ implemented |
| `kCGImagePropertyExifAuxLensModel` | ✅ implemented |
| `kCGImagePropertyExifAuxLensSerialNumber` | ✅ implemented |
| `kCGImagePropertyExifAuxOwnerName` | ✅ implemented |
| `kCGImagePropertyExifAuxSerialNumber` | ✅ implemented |
| `kCGImagePropertyExifBodySerialNumber` | ✅ implemented |
| `kCGImagePropertyExifBrightnessValue` | ✅ implemented |
| `kCGImagePropertyExifCFAPattern` | ✅ implemented |
| `kCGImagePropertyExifCameraOwnerName` | ✅ implemented |
| `kCGImagePropertyExifColorSpace` | ✅ implemented |
| `kCGImagePropertyExifComponentsConfiguration` | ✅ implemented |
| `kCGImagePropertyExifCompositeImage` | ✅ implemented |
| `kCGImagePropertyExifCompressedBitsPerPixel` | ✅ implemented |
| `kCGImagePropertyExifContrast` | ✅ implemented |
| `kCGImagePropertyExifCustomRendered` | ✅ implemented |
| `kCGImagePropertyExifDateTimeDigitized` | ✅ implemented |
| `kCGImagePropertyExifDateTimeOriginal` | ✅ implemented |
| `kCGImagePropertyExifDeviceSettingDescription` | ✅ implemented |
| `kCGImagePropertyExifDictionary` | ✅ implemented |
| `kCGImagePropertyExifDigitalZoomRatio` | ✅ implemented |
| `kCGImagePropertyExifExposureBiasValue` | ✅ implemented |
| `kCGImagePropertyExifExposureIndex` | ✅ implemented |
| `kCGImagePropertyExifExposureMode` | ✅ implemented |
| `kCGImagePropertyExifExposureProgram` | ✅ implemented |
| `kCGImagePropertyExifExposureTime` | ✅ implemented |
| `kCGImagePropertyExifFNumber` | ✅ implemented |
| `kCGImagePropertyExifFileSource` | ✅ implemented |
| `kCGImagePropertyExifFlash` | ✅ implemented |
| `kCGImagePropertyExifFlashEnergy` | ✅ implemented |
| `kCGImagePropertyExifFlashPixVersion` | ✅ implemented |
| `kCGImagePropertyExifFocalLenIn35mmFilm` | ✅ implemented |
| `kCGImagePropertyExifFocalLength` | ✅ implemented |
| `kCGImagePropertyExifFocalPlaneResolutionUnit` | ✅ implemented |
| `kCGImagePropertyExifFocalPlaneXResolution` | ✅ implemented |
| `kCGImagePropertyExifFocalPlaneYResolution` | ✅ implemented |
| `kCGImagePropertyExifGainControl` | ✅ implemented |
| `kCGImagePropertyExifGamma` | ✅ implemented |
| `kCGImagePropertyExifISOSpeed` | ✅ implemented |
| `kCGImagePropertyExifISOSpeedLatitudeyyy` | ✅ implemented |
| `kCGImagePropertyExifISOSpeedLatitudezzz` | ✅ implemented |
| `kCGImagePropertyExifISOSpeedRatings` | ✅ implemented |
| `kCGImagePropertyExifImageUniqueID` | ✅ implemented |
| `kCGImagePropertyExifLensMake` | ✅ implemented |
| `kCGImagePropertyExifLensModel` | ✅ implemented |
| `kCGImagePropertyExifLensSerialNumber` | ✅ implemented |
| `kCGImagePropertyExifLensSpecification` | ✅ implemented |
| `kCGImagePropertyExifLightSource` | ✅ implemented |
| `kCGImagePropertyExifMakerNote` | ✅ implemented |
| `kCGImagePropertyExifMaxApertureValue` | ✅ implemented |
| `kCGImagePropertyExifMeteringMode` | ✅ implemented |
| `kCGImagePropertyExifOECF` | ✅ implemented |
| `kCGImagePropertyExifOffsetTime` | ✅ implemented |
| `kCGImagePropertyExifOffsetTimeDigitized` | ✅ implemented |
| `kCGImagePropertyExifOffsetTimeOriginal` | ✅ implemented |
| `kCGImagePropertyExifPixelXDimension` | ✅ implemented |
| `kCGImagePropertyExifPixelYDimension` | ✅ implemented |
| `kCGImagePropertyExifRecommendedExposureIndex` | ✅ implemented |
| `kCGImagePropertyExifRelatedSoundFile` | ✅ implemented |
| `kCGImagePropertyExifSaturation` | ✅ implemented |
| `kCGImagePropertyExifSceneCaptureType` | ✅ implemented |
| `kCGImagePropertyExifSceneType` | ✅ implemented |
| `kCGImagePropertyExifSensingMethod` | ✅ implemented |
| `kCGImagePropertyExifSensitivityType` | ✅ implemented |
| `kCGImagePropertyExifSharpness` | ✅ implemented |
| `kCGImagePropertyExifShutterSpeedValue` | ✅ implemented |
| `kCGImagePropertyExifSourceExposureTimesOfCompositeImage` | ✅ implemented |
| `kCGImagePropertyExifSourceImageNumberOfCompositeImage` | ✅ implemented |
| `kCGImagePropertyExifSpatialFrequencyResponse` | ✅ implemented |
| `kCGImagePropertyExifSpectralSensitivity` | ✅ implemented |
| `kCGImagePropertyExifStandardOutputSensitivity` | ✅ implemented |
| `kCGImagePropertyExifSubjectArea` | ✅ implemented |
| `kCGImagePropertyExifSubjectDistRange` | ✅ implemented |
| `kCGImagePropertyExifSubjectDistance` | ✅ implemented |
| `kCGImagePropertyExifSubjectLocation` | ✅ implemented |
| `kCGImagePropertyExifSubsecTime` | ✅ implemented |
| `kCGImagePropertyExifSubsecTimeDigitized` | ✅ implemented |
| `kCGImagePropertyExifSubsecTimeOrginal` | ✅ implemented |
| `kCGImagePropertyExifSubsecTimeOriginal` | ✅ implemented |
| `kCGImagePropertyExifUserComment` | ✅ implemented |
| `kCGImagePropertyExifVersion` | ✅ implemented |
| `kCGImagePropertyExifWhiteBalance` | ✅ implemented |
| `kCGImagePropertyFileContentsDictionary` | ✅ implemented |
| `kCGImagePropertyFileSize` | ✅ implemented |
| `kCGImagePropertyGIFCanvasPixelHeight` | ✅ implemented |
| `kCGImagePropertyGIFCanvasPixelWidth` | ✅ implemented |
| `kCGImagePropertyGIFDelayTime` | ✅ implemented |
| `kCGImagePropertyGIFDictionary` | ✅ implemented |
| `kCGImagePropertyGIFFrameInfoArray` | ✅ implemented |
| `kCGImagePropertyGIFHasGlobalColorMap` | ✅ implemented |
| `kCGImagePropertyGIFImageColorMap` | ✅ implemented |
| `kCGImagePropertyGIFLoopCount` | ✅ implemented |
| `kCGImagePropertyGIFUnclampedDelayTime` | ✅ implemented |
| `kCGImagePropertyGPSAltitude` | ✅ implemented |
| `kCGImagePropertyGPSAltitudeRef` | ✅ implemented |
| `kCGImagePropertyGPSAreaInformation` | ✅ implemented |
| `kCGImagePropertyGPSDOP` | ✅ implemented |
| `kCGImagePropertyGPSDateStamp` | ✅ implemented |
| `kCGImagePropertyGPSDestBearing` | ✅ implemented |
| `kCGImagePropertyGPSDestBearingRef` | ✅ implemented |
| `kCGImagePropertyGPSDestDistance` | ✅ implemented |
| `kCGImagePropertyGPSDestDistanceRef` | ✅ implemented |
| `kCGImagePropertyGPSDestLatitude` | ✅ implemented |
| `kCGImagePropertyGPSDestLatitudeRef` | ✅ implemented |
| `kCGImagePropertyGPSDestLongitude` | ✅ implemented |
| `kCGImagePropertyGPSDestLongitudeRef` | ✅ implemented |
| `kCGImagePropertyGPSDictionary` | ✅ implemented |
| `kCGImagePropertyGPSDifferental` | ✅ implemented |
| `kCGImagePropertyGPSHPositioningError` | ✅ implemented |
| `kCGImagePropertyGPSImgDirection` | ✅ implemented |
| `kCGImagePropertyGPSImgDirectionRef` | ✅ implemented |
| `kCGImagePropertyGPSLatitude` | ✅ implemented |
| `kCGImagePropertyGPSLatitudeRef` | ✅ implemented |
| `kCGImagePropertyGPSLongitude` | ✅ implemented |
| `kCGImagePropertyGPSLongitudeRef` | ✅ implemented |
| `kCGImagePropertyGPSMapDatum` | ✅ implemented |
| `kCGImagePropertyGPSMeasureMode` | ✅ implemented |
| `kCGImagePropertyGPSProcessingMethod` | ✅ implemented |
| `kCGImagePropertyGPSSatellites` | ✅ implemented |
| `kCGImagePropertyGPSSpeed` | ✅ implemented |
| `kCGImagePropertyGPSSpeedRef` | ✅ implemented |
| `kCGImagePropertyGPSStatus` | ✅ implemented |
| `kCGImagePropertyGPSTimeStamp` | ✅ implemented |
| `kCGImagePropertyGPSTrack` | ✅ implemented |
| `kCGImagePropertyGPSTrackRef` | ✅ implemented |
| `kCGImagePropertyGPSVersion` | ✅ implemented |
| `kCGImagePropertyGroupImageBaseline` | ✅ implemented |
| `kCGImagePropertyGroupImageDisparityAdjustment` | ✅ implemented |
| `kCGImagePropertyGroupImageIndexLeft` | ✅ implemented |
| `kCGImagePropertyGroupImageIndexMonoscopic` | ✅ implemented |
| `kCGImagePropertyGroupImageIndexRight` | ✅ implemented |
| `kCGImagePropertyGroupImageIsAlternateImage` | ✅ implemented |
| `kCGImagePropertyGroupImageIsLeftImage` | ✅ implemented |
| `kCGImagePropertyGroupImageIsMonoscopicImage` | ✅ implemented |
| `kCGImagePropertyGroupImageIsRightImage` | ✅ implemented |
| `kCGImagePropertyGroupImageStereoAggressors` | ✅ implemented |
| `kCGImagePropertyGroupImagesAlternate` | ✅ implemented |
| `kCGImagePropertyGroupIndex` | ✅ implemented |
| `kCGImagePropertyGroupMonoscopicImageLocation` | ✅ implemented |
| `kCGImagePropertyGroupType` | ✅ implemented |
| `kCGImagePropertyGroupTypeAlternate` | ✅ implemented |
| `kCGImagePropertyGroupTypeStereoPair` | ✅ implemented |
| `kCGImagePropertyGroups` | ✅ implemented |
| `kCGImagePropertyHEICSCanvasPixelHeight` | ✅ implemented |
| `kCGImagePropertyHEICSCanvasPixelWidth` | ✅ implemented |
| `kCGImagePropertyHEICSDelayTime` | ✅ implemented |
| `kCGImagePropertyHEICSDictionary` | ✅ implemented |
| `kCGImagePropertyHEICSFrameInfoArray` | ✅ implemented |
| `kCGImagePropertyHEICSLoopCount` | ✅ implemented |
| `kCGImagePropertyHEICSUnclampedDelayTime` | ✅ implemented |
| `kCGImagePropertyHEIFDictionary` | ✅ implemented |
| `kCGImagePropertyHasAlpha` | ✅ implemented |
| `kCGImagePropertyHeight` | ✅ implemented |
| `kCGImagePropertyIPTCActionAdvised` | ✅ implemented |
| `kCGImagePropertyIPTCByline` | ✅ implemented |
| `kCGImagePropertyIPTCBylineTitle` | ✅ implemented |
| `kCGImagePropertyIPTCCaptionAbstract` | ✅ implemented |
| `kCGImagePropertyIPTCCategory` | ✅ implemented |
| `kCGImagePropertyIPTCCity` | ✅ implemented |
| `kCGImagePropertyIPTCContact` | ✅ implemented |
| `kCGImagePropertyIPTCContactInfoAddress` | ✅ implemented |
| `kCGImagePropertyIPTCContactInfoCity` | ✅ implemented |
| `kCGImagePropertyIPTCContactInfoCountry` | ✅ implemented |
| `kCGImagePropertyIPTCContactInfoEmails` | ✅ implemented |
| `kCGImagePropertyIPTCContactInfoPhones` | ✅ implemented |
| `kCGImagePropertyIPTCContactInfoPostalCode` | ✅ implemented |
| `kCGImagePropertyIPTCContactInfoStateProvince` | ✅ implemented |
| `kCGImagePropertyIPTCContactInfoWebURLs` | ✅ implemented |
| `kCGImagePropertyIPTCContentLocationCode` | ✅ implemented |
| `kCGImagePropertyIPTCContentLocationName` | ✅ implemented |
| `kCGImagePropertyIPTCCopyrightNotice` | ✅ implemented |
| `kCGImagePropertyIPTCCountryPrimaryLocationCode` | ✅ implemented |
| `kCGImagePropertyIPTCCountryPrimaryLocationName` | ✅ implemented |
| `kCGImagePropertyIPTCCreatorContactInfo` | ✅ implemented |
| `kCGImagePropertyIPTCCredit` | ✅ implemented |
| `kCGImagePropertyIPTCDateCreated` | ✅ implemented |
| `kCGImagePropertyIPTCDictionary` | ✅ implemented |
| `kCGImagePropertyIPTCDigitalCreationDate` | ✅ implemented |
| `kCGImagePropertyIPTCDigitalCreationTime` | ✅ implemented |
| `kCGImagePropertyIPTCEditStatus` | ✅ implemented |
| `kCGImagePropertyIPTCEditorialUpdate` | ✅ implemented |
| `kCGImagePropertyIPTCExpirationDate` | ✅ implemented |
| `kCGImagePropertyIPTCExpirationTime` | ✅ implemented |
| `kCGImagePropertyIPTCExtAboutCvTerm` | ✅ implemented |
| `kCGImagePropertyIPTCExtAboutCvTermCvId` | ✅ implemented |
| `kCGImagePropertyIPTCExtAboutCvTermId` | ✅ implemented |
| `kCGImagePropertyIPTCExtAboutCvTermName` | ✅ implemented |
| `kCGImagePropertyIPTCExtAboutCvTermRefinedAbout` | ✅ implemented |
| `kCGImagePropertyIPTCExtAddlModelInfo` | ✅ implemented |
| `kCGImagePropertyIPTCExtArtworkCircaDateCreated` | ✅ implemented |
| `kCGImagePropertyIPTCExtArtworkContentDescription` | ✅ implemented |
| `kCGImagePropertyIPTCExtArtworkContributionDescription` | ✅ implemented |
| `kCGImagePropertyIPTCExtArtworkCopyrightNotice` | ✅ implemented |
| `kCGImagePropertyIPTCExtArtworkCopyrightOwnerID` | ✅ implemented |
| `kCGImagePropertyIPTCExtArtworkCopyrightOwnerName` | ✅ implemented |
| `kCGImagePropertyIPTCExtArtworkCreator` | ✅ implemented |
| `kCGImagePropertyIPTCExtArtworkCreatorID` | ✅ implemented |
| `kCGImagePropertyIPTCExtArtworkDateCreated` | ✅ implemented |
| `kCGImagePropertyIPTCExtArtworkLicensorID` | ✅ implemented |
| `kCGImagePropertyIPTCExtArtworkLicensorName` | ✅ implemented |
| `kCGImagePropertyIPTCExtArtworkOrObject` | ✅ implemented |
| `kCGImagePropertyIPTCExtArtworkPhysicalDescription` | ✅ implemented |
| `kCGImagePropertyIPTCExtArtworkSource` | ✅ implemented |
| `kCGImagePropertyIPTCExtArtworkSourceInvURL` | ✅ implemented |
| `kCGImagePropertyIPTCExtArtworkSourceInventoryNo` | ✅ implemented |
| `kCGImagePropertyIPTCExtArtworkStylePeriod` | ✅ implemented |
| `kCGImagePropertyIPTCExtArtworkTitle` | ✅ implemented |
| `kCGImagePropertyIPTCExtAudioBitrate` | ✅ implemented |
| `kCGImagePropertyIPTCExtAudioBitrateMode` | ✅ implemented |
| `kCGImagePropertyIPTCExtAudioChannelCount` | ✅ implemented |
| `kCGImagePropertyIPTCExtCircaDateCreated` | ✅ implemented |
| `kCGImagePropertyIPTCExtContainerFormat` | ✅ implemented |
| `kCGImagePropertyIPTCExtContainerFormatIdentifier` | ✅ implemented |
| `kCGImagePropertyIPTCExtContainerFormatName` | ✅ implemented |
| `kCGImagePropertyIPTCExtContributor` | ✅ implemented |
| `kCGImagePropertyIPTCExtContributorIdentifier` | ✅ implemented |
| `kCGImagePropertyIPTCExtContributorName` | ✅ implemented |
| `kCGImagePropertyIPTCExtContributorRole` | ✅ implemented |
| `kCGImagePropertyIPTCExtControlledVocabularyTerm` | ✅ implemented |
| `kCGImagePropertyIPTCExtCopyrightYear` | ✅ implemented |
| `kCGImagePropertyIPTCExtCreator` | ✅ implemented |
| `kCGImagePropertyIPTCExtCreatorIdentifier` | ✅ implemented |
| `kCGImagePropertyIPTCExtCreatorName` | ✅ implemented |
| `kCGImagePropertyIPTCExtCreatorRole` | ✅ implemented |
| `kCGImagePropertyIPTCExtDataOnScreen` | ✅ implemented |
| `kCGImagePropertyIPTCExtDataOnScreenRegion` | ✅ implemented |
| `kCGImagePropertyIPTCExtDataOnScreenRegionD` | ✅ implemented |
| `kCGImagePropertyIPTCExtDataOnScreenRegionH` | ✅ implemented |
| `kCGImagePropertyIPTCExtDataOnScreenRegionText` | ✅ implemented |
| `kCGImagePropertyIPTCExtDataOnScreenRegionUnit` | ✅ implemented |
| `kCGImagePropertyIPTCExtDataOnScreenRegionW` | ✅ implemented |
| `kCGImagePropertyIPTCExtDataOnScreenRegionX` | ✅ implemented |
| `kCGImagePropertyIPTCExtDataOnScreenRegionY` | ✅ implemented |
| `kCGImagePropertyIPTCExtDigitalImageGUID` | ✅ implemented |
| `kCGImagePropertyIPTCExtDigitalSourceFileType` | ✅ implemented |
| `kCGImagePropertyIPTCExtDigitalSourceType` | ✅ implemented |
| `kCGImagePropertyIPTCExtDopesheet` | ✅ implemented |
| `kCGImagePropertyIPTCExtDopesheetLink` | ✅ implemented |
| `kCGImagePropertyIPTCExtDopesheetLinkLink` | ✅ implemented |
| `kCGImagePropertyIPTCExtDopesheetLinkLinkQualifier` | ✅ implemented |
| `kCGImagePropertyIPTCExtEmbdEncRightsExpr` | ✅ implemented |
| `kCGImagePropertyIPTCExtEmbeddedEncodedRightsExpr` | ✅ implemented |
| `kCGImagePropertyIPTCExtEmbeddedEncodedRightsExprLangID` | ✅ implemented |
| `kCGImagePropertyIPTCExtEmbeddedEncodedRightsExprType` | ✅ implemented |
| `kCGImagePropertyIPTCExtEpisode` | ✅ implemented |
| `kCGImagePropertyIPTCExtEpisodeIdentifier` | ✅ implemented |
| `kCGImagePropertyIPTCExtEpisodeName` | ✅ implemented |
| `kCGImagePropertyIPTCExtEpisodeNumber` | ✅ implemented |
| `kCGImagePropertyIPTCExtEvent` | ✅ implemented |
| `kCGImagePropertyIPTCExtExternalMetadataLink` | ✅ implemented |
| `kCGImagePropertyIPTCExtFeedIdentifier` | ✅ implemented |
| `kCGImagePropertyIPTCExtGenre` | ✅ implemented |
| `kCGImagePropertyIPTCExtGenreCvId` | ✅ implemented |
| `kCGImagePropertyIPTCExtGenreCvTermId` | ✅ implemented |
| `kCGImagePropertyIPTCExtGenreCvTermName` | ✅ implemented |
| `kCGImagePropertyIPTCExtGenreCvTermRefinedAbout` | ✅ implemented |
| `kCGImagePropertyIPTCExtHeadline` | ✅ implemented |
| `kCGImagePropertyIPTCExtIPTCLastEdited` | ✅ implemented |
| `kCGImagePropertyIPTCExtLinkedEncRightsExpr` | ✅ implemented |
| `kCGImagePropertyIPTCExtLinkedEncodedRightsExpr` | ✅ implemented |
| `kCGImagePropertyIPTCExtLinkedEncodedRightsExprLangID` | ✅ implemented |
| `kCGImagePropertyIPTCExtLinkedEncodedRightsExprType` | ✅ implemented |
| `kCGImagePropertyIPTCExtLocationCity` | ✅ implemented |
| `kCGImagePropertyIPTCExtLocationCountryCode` | ✅ implemented |
| `kCGImagePropertyIPTCExtLocationCountryName` | ✅ implemented |
| `kCGImagePropertyIPTCExtLocationCreated` | ✅ implemented |
| `kCGImagePropertyIPTCExtLocationGPSAltitude` | ✅ implemented |
| `kCGImagePropertyIPTCExtLocationGPSLatitude` | ✅ implemented |
| `kCGImagePropertyIPTCExtLocationGPSLongitude` | ✅ implemented |
| `kCGImagePropertyIPTCExtLocationIdentifier` | ✅ implemented |
| `kCGImagePropertyIPTCExtLocationLocationId` | ✅ implemented |
| `kCGImagePropertyIPTCExtLocationLocationName` | ✅ implemented |
| `kCGImagePropertyIPTCExtLocationProvinceState` | ✅ implemented |
| `kCGImagePropertyIPTCExtLocationShown` | ✅ implemented |
| `kCGImagePropertyIPTCExtLocationSublocation` | ✅ implemented |
| `kCGImagePropertyIPTCExtLocationWorldRegion` | ✅ implemented |
| `kCGImagePropertyIPTCExtMaxAvailHeight` | ✅ implemented |
| `kCGImagePropertyIPTCExtMaxAvailWidth` | ✅ implemented |
| `kCGImagePropertyIPTCExtModelAge` | ✅ implemented |
| `kCGImagePropertyIPTCExtOrganisationInImageCode` | ✅ implemented |
| `kCGImagePropertyIPTCExtOrganisationInImageName` | ✅ implemented |
| `kCGImagePropertyIPTCExtPersonHeard` | ✅ implemented |
| `kCGImagePropertyIPTCExtPersonHeardIdentifier` | ✅ implemented |
| `kCGImagePropertyIPTCExtPersonHeardName` | ✅ implemented |
| `kCGImagePropertyIPTCExtPersonInImage` | ✅ implemented |
| `kCGImagePropertyIPTCExtPersonInImageCharacteristic` | ✅ implemented |
| `kCGImagePropertyIPTCExtPersonInImageCvTermCvId` | ✅ implemented |
| `kCGImagePropertyIPTCExtPersonInImageCvTermId` | ✅ implemented |
| `kCGImagePropertyIPTCExtPersonInImageCvTermName` | ✅ implemented |
| `kCGImagePropertyIPTCExtPersonInImageCvTermRefinedAbout` | ✅ implemented |
| `kCGImagePropertyIPTCExtPersonInImageDescription` | ✅ implemented |
| `kCGImagePropertyIPTCExtPersonInImageId` | ✅ implemented |
| `kCGImagePropertyIPTCExtPersonInImageName` | ✅ implemented |
| `kCGImagePropertyIPTCExtPersonInImageWDetails` | ✅ implemented |
| `kCGImagePropertyIPTCExtProductInImage` | ✅ implemented |
| `kCGImagePropertyIPTCExtProductInImageDescription` | ✅ implemented |
| `kCGImagePropertyIPTCExtProductInImageGTIN` | ✅ implemented |
| `kCGImagePropertyIPTCExtProductInImageName` | ✅ implemented |
| `kCGImagePropertyIPTCExtPublicationEvent` | ✅ implemented |
| `kCGImagePropertyIPTCExtPublicationEventDate` | ✅ implemented |
| `kCGImagePropertyIPTCExtPublicationEventIdentifier` | ✅ implemented |
| `kCGImagePropertyIPTCExtPublicationEventName` | ✅ implemented |
| `kCGImagePropertyIPTCExtRating` | ✅ implemented |
| `kCGImagePropertyIPTCExtRatingRatingRegion` | ✅ implemented |
| `kCGImagePropertyIPTCExtRatingRegionCity` | ✅ implemented |
| `kCGImagePropertyIPTCExtRatingRegionCountryCode` | ✅ implemented |
| `kCGImagePropertyIPTCExtRatingRegionCountryName` | ✅ implemented |
| `kCGImagePropertyIPTCExtRatingRegionGPSAltitude` | ✅ implemented |
| `kCGImagePropertyIPTCExtRatingRegionGPSLatitude` | ✅ implemented |
| `kCGImagePropertyIPTCExtRatingRegionGPSLongitude` | ✅ implemented |
| `kCGImagePropertyIPTCExtRatingRegionIdentifier` | ✅ implemented |
| `kCGImagePropertyIPTCExtRatingRegionLocationId` | ✅ implemented |
| `kCGImagePropertyIPTCExtRatingRegionLocationName` | ✅ implemented |
| `kCGImagePropertyIPTCExtRatingRegionProvinceState` | ✅ implemented |
| `kCGImagePropertyIPTCExtRatingRegionSublocation` | ✅ implemented |
| `kCGImagePropertyIPTCExtRatingRegionWorldRegion` | ✅ implemented |
| `kCGImagePropertyIPTCExtRatingScaleMaxValue` | ✅ implemented |
| `kCGImagePropertyIPTCExtRatingScaleMinValue` | ✅ implemented |
| `kCGImagePropertyIPTCExtRatingSourceLink` | ✅ implemented |
| `kCGImagePropertyIPTCExtRatingValue` | ✅ implemented |
| `kCGImagePropertyIPTCExtRatingValueLogoLink` | ✅ implemented |
| `kCGImagePropertyIPTCExtRegistryEntryRole` | ✅ implemented |
| `kCGImagePropertyIPTCExtRegistryID` | ✅ implemented |
| `kCGImagePropertyIPTCExtRegistryItemID` | ✅ implemented |
| `kCGImagePropertyIPTCExtRegistryOrganisationID` | ✅ implemented |
| `kCGImagePropertyIPTCExtReleaseReady` | ✅ implemented |
| `kCGImagePropertyIPTCExtSeason` | ✅ implemented |
| `kCGImagePropertyIPTCExtSeasonIdentifier` | ✅ implemented |
| `kCGImagePropertyIPTCExtSeasonName` | ✅ implemented |
| `kCGImagePropertyIPTCExtSeasonNumber` | ✅ implemented |
| `kCGImagePropertyIPTCExtSeries` | ✅ implemented |
| `kCGImagePropertyIPTCExtSeriesIdentifier` | ✅ implemented |
| `kCGImagePropertyIPTCExtSeriesName` | ✅ implemented |
| `kCGImagePropertyIPTCExtShownEvent` | ✅ implemented |
| `kCGImagePropertyIPTCExtShownEventIdentifier` | ✅ implemented |
| `kCGImagePropertyIPTCExtShownEventName` | ✅ implemented |
| `kCGImagePropertyIPTCExtStorylineIdentifier` | ✅ implemented |
| `kCGImagePropertyIPTCExtStreamReady` | ✅ implemented |
| `kCGImagePropertyIPTCExtStylePeriod` | ✅ implemented |
| `kCGImagePropertyIPTCExtSupplyChainSource` | ✅ implemented |
| `kCGImagePropertyIPTCExtSupplyChainSourceIdentifier` | ✅ implemented |
| `kCGImagePropertyIPTCExtSupplyChainSourceName` | ✅ implemented |
| `kCGImagePropertyIPTCExtTemporalCoverage` | ✅ implemented |
| `kCGImagePropertyIPTCExtTemporalCoverageFrom` | ✅ implemented |
| `kCGImagePropertyIPTCExtTemporalCoverageTo` | ✅ implemented |
| `kCGImagePropertyIPTCExtTranscript` | ✅ implemented |
| `kCGImagePropertyIPTCExtTranscriptLink` | ✅ implemented |
| `kCGImagePropertyIPTCExtTranscriptLinkLink` | ✅ implemented |
| `kCGImagePropertyIPTCExtTranscriptLinkLinkQualifier` | ✅ implemented |
| `kCGImagePropertyIPTCExtVideoBitrate` | ✅ implemented |
| `kCGImagePropertyIPTCExtVideoBitrateMode` | ✅ implemented |
| `kCGImagePropertyIPTCExtVideoDisplayAspectRatio` | ✅ implemented |
| `kCGImagePropertyIPTCExtVideoEncodingProfile` | ✅ implemented |
| `kCGImagePropertyIPTCExtVideoShotType` | ✅ implemented |
| `kCGImagePropertyIPTCExtVideoShotTypeIdentifier` | ✅ implemented |
| `kCGImagePropertyIPTCExtVideoShotTypeName` | ✅ implemented |
| `kCGImagePropertyIPTCExtVideoStreamsCount` | ✅ implemented |
| `kCGImagePropertyIPTCExtVisualColor` | ✅ implemented |
| `kCGImagePropertyIPTCExtWorkflowTag` | ✅ implemented |
| `kCGImagePropertyIPTCExtWorkflowTagCvId` | ✅ implemented |
| `kCGImagePropertyIPTCExtWorkflowTagCvTermId` | ✅ implemented |
| `kCGImagePropertyIPTCExtWorkflowTagCvTermName` | ✅ implemented |
| `kCGImagePropertyIPTCExtWorkflowTagCvTermRefinedAbout` | ✅ implemented |
| `kCGImagePropertyIPTCFixtureIdentifier` | ✅ implemented |
| `kCGImagePropertyIPTCHeadline` | ✅ implemented |
| `kCGImagePropertyIPTCImageOrientation` | ✅ implemented |
| `kCGImagePropertyIPTCImageType` | ✅ implemented |
| `kCGImagePropertyIPTCKeywords` | ✅ implemented |
| `kCGImagePropertyIPTCLanguageIdentifier` | ✅ implemented |
| `kCGImagePropertyIPTCObjectAttributeReference` | ✅ implemented |
| `kCGImagePropertyIPTCObjectCycle` | ✅ implemented |
| `kCGImagePropertyIPTCObjectName` | ✅ implemented |
| `kCGImagePropertyIPTCObjectTypeReference` | ✅ implemented |
| `kCGImagePropertyIPTCOriginalTransmissionReference` | ✅ implemented |
| `kCGImagePropertyIPTCOriginatingProgram` | ✅ implemented |
| `kCGImagePropertyIPTCProgramVersion` | ✅ implemented |
| `kCGImagePropertyIPTCProvinceState` | ✅ implemented |
| `kCGImagePropertyIPTCReferenceDate` | ✅ implemented |
| `kCGImagePropertyIPTCReferenceNumber` | ✅ implemented |
| `kCGImagePropertyIPTCReferenceService` | ✅ implemented |
| `kCGImagePropertyIPTCReleaseDate` | ✅ implemented |
| `kCGImagePropertyIPTCReleaseTime` | ✅ implemented |
| `kCGImagePropertyIPTCRightsUsageTerms` | ✅ implemented |
| `kCGImagePropertyIPTCScene` | ✅ implemented |
| `kCGImagePropertyIPTCSource` | ✅ implemented |
| `kCGImagePropertyIPTCSpecialInstructions` | ✅ implemented |
| `kCGImagePropertyIPTCStarRating` | ✅ implemented |
| `kCGImagePropertyIPTCSubLocation` | ✅ implemented |
| `kCGImagePropertyIPTCSubjectReference` | ✅ implemented |
| `kCGImagePropertyIPTCSupplementalCategory` | ✅ implemented |
| `kCGImagePropertyIPTCTimeCreated` | ✅ implemented |
| `kCGImagePropertyIPTCUrgency` | ✅ implemented |
| `kCGImagePropertyIPTCWriterEditor` | ✅ implemented |
| `kCGImagePropertyImageCount` | ✅ implemented |
| `kCGImagePropertyImageIndex` | ✅ implemented |
| `kCGImagePropertyImages` | ✅ implemented |
| `kCGImagePropertyIsFloat` | ✅ implemented |
| `kCGImagePropertyIsIndexed` | ✅ implemented |
| `kCGImagePropertyJFIFDensityUnit` | ✅ implemented |
| `kCGImagePropertyJFIFDictionary` | ✅ implemented |
| `kCGImagePropertyJFIFIsProgressive` | ✅ implemented |
| `kCGImagePropertyJFIFVersion` | ✅ implemented |
| `kCGImagePropertyJFIFXDensity` | ✅ implemented |
| `kCGImagePropertyJFIFYDensity` | ✅ implemented |
| `kCGImagePropertyMakerAppleDictionary` | ✅ implemented |
| `kCGImagePropertyMakerCanonAspectRatioInfo` | ✅ implemented |
| `kCGImagePropertyMakerCanonCameraSerialNumber` | ✅ implemented |
| `kCGImagePropertyMakerCanonContinuousDrive` | ✅ implemented |
| `kCGImagePropertyMakerCanonDictionary` | ✅ implemented |
| `kCGImagePropertyMakerCanonFirmware` | ✅ implemented |
| `kCGImagePropertyMakerCanonFlashExposureComp` | ✅ implemented |
| `kCGImagePropertyMakerCanonImageSerialNumber` | ✅ implemented |
| `kCGImagePropertyMakerCanonLensModel` | ✅ implemented |
| `kCGImagePropertyMakerCanonOwnerName` | ✅ implemented |
| `kCGImagePropertyMakerFujiDictionary` | ✅ implemented |
| `kCGImagePropertyMakerMinoltaDictionary` | ✅ implemented |
| `kCGImagePropertyMakerNikonCameraSerialNumber` | ✅ implemented |
| `kCGImagePropertyMakerNikonColorMode` | ✅ implemented |
| `kCGImagePropertyMakerNikonDictionary` | ✅ implemented |
| `kCGImagePropertyMakerNikonDigitalZoom` | ✅ implemented |
| `kCGImagePropertyMakerNikonFlashExposureComp` | ✅ implemented |
| `kCGImagePropertyMakerNikonFlashSetting` | ✅ implemented |
| `kCGImagePropertyMakerNikonFocusDistance` | ✅ implemented |
| `kCGImagePropertyMakerNikonFocusMode` | ✅ implemented |
| `kCGImagePropertyMakerNikonISOSelection` | ✅ implemented |
| `kCGImagePropertyMakerNikonISOSetting` | ✅ implemented |
| `kCGImagePropertyMakerNikonImageAdjustment` | ✅ implemented |
| `kCGImagePropertyMakerNikonLensAdapter` | ✅ implemented |
| `kCGImagePropertyMakerNikonLensInfo` | ✅ implemented |
| `kCGImagePropertyMakerNikonLensType` | ✅ implemented |
| `kCGImagePropertyMakerNikonQuality` | ✅ implemented |
| `kCGImagePropertyMakerNikonSharpenMode` | ✅ implemented |
| `kCGImagePropertyMakerNikonShootingMode` | ✅ implemented |
| `kCGImagePropertyMakerNikonShutterCount` | ✅ implemented |
| `kCGImagePropertyMakerNikonWhiteBalanceMode` | ✅ implemented |
| `kCGImagePropertyMakerOlympusDictionary` | ✅ implemented |
| `kCGImagePropertyMakerPentaxDictionary` | ✅ implemented |
| `kCGImagePropertyNamedColorSpace` | ✅ implemented |
| `kCGImagePropertyOpenEXRAspectRatio` | ✅ implemented |
| `kCGImagePropertyOpenEXRCompression` | ✅ implemented |
| `kCGImagePropertyOpenEXRDictionary` | ✅ implemented |
| `kCGImagePropertyOrientation` | ✅ implemented |
| `kCGImagePropertyPNGAuthor` | ✅ implemented |
| `kCGImagePropertyPNGChromaticities` | ✅ implemented |
| `kCGImagePropertyPNGComment` | ✅ implemented |
| `kCGImagePropertyPNGCompressionFilter` | ✅ implemented |
| `kCGImagePropertyPNGCopyright` | ✅ implemented |
| `kCGImagePropertyPNGCreationTime` | ✅ implemented |
| `kCGImagePropertyPNGDescription` | ✅ implemented |
| `kCGImagePropertyPNGDictionary` | ✅ implemented |
| `kCGImagePropertyPNGDisclaimer` | ✅ implemented |
| `kCGImagePropertyPNGGamma` | ✅ implemented |
| `kCGImagePropertyPNGInterlaceType` | ✅ implemented |
| `kCGImagePropertyPNGModificationTime` | ✅ implemented |
| `kCGImagePropertyPNGPixelsAspectRatio` | ✅ implemented |
| `kCGImagePropertyPNGSoftware` | ✅ implemented |
| `kCGImagePropertyPNGSource` | ✅ implemented |
| `kCGImagePropertyPNGTitle` | ✅ implemented |
| `kCGImagePropertyPNGTransparency` | ✅ implemented |
| `kCGImagePropertyPNGWarning` | ✅ implemented |
| `kCGImagePropertyPNGXPixelsPerMeter` | ✅ implemented |
| `kCGImagePropertyPNGYPixelsPerMeter` | ✅ implemented |
| `kCGImagePropertyPNGsRGBIntent` | ✅ implemented |
| `kCGImagePropertyPixelFormat` | ✅ implemented |
| `kCGImagePropertyPixelHeight` | ✅ implemented |
| `kCGImagePropertyPixelWidth` | ✅ implemented |
| `kCGImagePropertyPrimaryImage` | ✅ implemented |
| `kCGImagePropertyProfileName` | ✅ implemented |
| `kCGImagePropertyRawDictionary` | ✅ implemented |
| `kCGImagePropertyTGACompression` | ✅ implemented |
| `kCGImagePropertyTGADictionary` | ✅ implemented |
| `kCGImagePropertyTIFFArtist` | ✅ implemented |
| `kCGImagePropertyTIFFCompression` | ✅ implemented |
| `kCGImagePropertyTIFFCopyright` | ✅ implemented |
| `kCGImagePropertyTIFFDateTime` | ✅ implemented |
| `kCGImagePropertyTIFFDictionary` | ✅ implemented |
| `kCGImagePropertyTIFFDocumentName` | ✅ implemented |
| `kCGImagePropertyTIFFHostComputer` | ✅ implemented |
| `kCGImagePropertyTIFFImageDescription` | ✅ implemented |
| `kCGImagePropertyTIFFMake` | ✅ implemented |
| `kCGImagePropertyTIFFModel` | ✅ implemented |
| `kCGImagePropertyTIFFOrientation` | ✅ implemented |
| `kCGImagePropertyTIFFPhotometricInterpretation` | ✅ implemented |
| `kCGImagePropertyTIFFPrimaryChromaticities` | ✅ implemented |
| `kCGImagePropertyTIFFResolutionUnit` | ✅ implemented |
| `kCGImagePropertyTIFFSoftware` | ✅ implemented |
| `kCGImagePropertyTIFFTileLength` | ✅ implemented |
| `kCGImagePropertyTIFFTileWidth` | ✅ implemented |
| `kCGImagePropertyTIFFTransferFunction` | ✅ implemented |
| `kCGImagePropertyTIFFWhitePoint` | ✅ implemented |
| `kCGImagePropertyTIFFXPosition` | ✅ implemented |
| `kCGImagePropertyTIFFXResolution` | ✅ implemented |
| `kCGImagePropertyTIFFYPosition` | ✅ implemented |
| `kCGImagePropertyTIFFYResolution` | ✅ implemented |
| `kCGImagePropertyThumbnailImages` | ✅ implemented |
| `kCGImagePropertyWebPCanvasPixelHeight` | ✅ implemented |
| `kCGImagePropertyWebPCanvasPixelWidth` | ✅ implemented |
| `kCGImagePropertyWebPDelayTime` | ✅ implemented |
| `kCGImagePropertyWebPDictionary` | ✅ implemented |
| `kCGImagePropertyWebPFrameInfoArray` | ✅ implemented |
| `kCGImagePropertyWebPLoopCount` | ✅ implemented |
| `kCGImagePropertyWebPUnclampedDelayTime` | ✅ implemented |
| `kCGImagePropertyWidth` | ✅ implemented |
| `kCGImageProviderPreferredTileHeight` | ✅ implemented |
| `kCGImageProviderPreferredTileWidth` | ✅ implemented |
| `kIIOCameraExtrinsics_CoordinateSystemID` | ✅ implemented |
| `kIIOCameraExtrinsics_Position` | ✅ implemented |
| `kIIOCameraExtrinsics_Rotation` | ✅ implemented |
| `kIIOCameraModelType_GenericPinhole` | ✅ implemented |
| `kIIOCameraModelType_SimplifiedPinhole` | ✅ implemented |
| `kIIOCameraModel_Intrinsics` | ✅ implemented |
| `kIIOCameraModel_ModelType` | ✅ implemented |
| `kIIOMetadata_CameraExtrinsicsKey` | ✅ implemented |
| `kIIOMetadata_CameraModelKey` | ✅ implemented |
| `kIIOMonoscopicImageLocation_Center` | ✅ implemented |
| `kIIOMonoscopicImageLocation_Left` | ✅ implemented |
| `kIIOMonoscopicImageLocation_Right` | ✅ implemented |
| `kIIOMonoscopicImageLocation_Unspecified` | ✅ implemented |
| `kIIOStereoAggressors_Severity` | ✅ implemented |
| `kIIOStereoAggressors_SubTypeURI` | ✅ implemented |
| `kIIOStereoAggressors_Type` | ✅ implemented |

