﻿/******************************************************************************
 *                                                                             *
 *   PROJECT : EOS Digital Software Development Kit EDSDK                      *
 *      NAME : EdsTypes.h                                                      *
 *                                                                             *
 *   Description: COMMON DEFINITION FOR EDSDK                                  *
 *                                                                             *
 *******************************************************************************
 *                                                                             *
 *   Written and developed by Canon Inc.                                       *
 *   Copyright Canon Inc. 2006-2024 All Rights Reserved                        *
 *                                                                             *
 ******************************************************************************/

#ifndef _EDS_TYPES_H_
#define _EDS_TYPES_H_

#ifdef __MACOS__
#if PRAGMA_STRUCT_ALIGN
#pragma options align = mac68k
#endif
#if defined __LP64__ /*__x86_64__*/
#define MAC64
#endif
#else
#pragma pack(push, 8)
#endif

#if defined __MACOS__ || TARGET_OS_LINUX
#define EDSSTDCALL
#define EDSEXPORT
#define EDSIMPORT
#elif _MSC_VER
#define EDSSTDCALL __stdcall
#define EDSEXPORT __declspec(dllexport)
#define EDSIMPORT __declspec(dllimport)
#else
#define EDSSTDCALL
#define EDSEXPORT
#define EDSIMPORT
#endif

/*----------------------------------------------------------------------------*/

/******************************************************************************
 Definition of Constants
******************************************************************************/
#define EDS_MAX_NAME 256
#define EDS_TRANSFER_BLOCK_SIZE 512

/******************************************************************************
 Definition of Data Types
******************************************************************************/
/*-----------------------------------------------------------------------------
 Callback Types
-----------------------------------------------------------------------------*/
#define EDSCALLBACK EDSSTDCALL

/*-----------------------------------------------------------------------------
 Basic Types
-----------------------------------------------------------------------------*/
#ifndef NULL
#ifdef __cplusplus
#define NULL 0
#else
#define NULL ((void *)0)
#endif
#endif

#ifndef FALSE
#define FALSE 0
#endif

#ifndef TRUE
#define TRUE 1
#endif

typedef void EdsVoid;
typedef int EdsBool;
typedef char EdsChar;

#ifdef __MACOS__
#ifndef SInt8

typedef char EdsInt8;
typedef unsigned char EdsUInt8;
typedef short EdsInt16;
typedef unsigned short EdsUInt16;
typedef int EdsInt32;
typedef unsigned int EdsUInt32;
#if defined __LP64__
typedef long EdsInt64;
typedef unsigned long EdsUInt64;
typedef unsigned long size_t;
#else
typedef long long EdsInt64;
typedef unsigned long long EdsUInt64;
#endif

#else
typedef SInt8 EdsInt8;
typedef UInt8 EdsUInt8;
typedef SInt16 EdsInt16;
typedef UInt16 EdsUInt16;
typedef SInt32 EdsInt32;
typedef UInt32 EdsUInt32;
#ifdef __cplusplus
typedef long long EdsInt64;
typedef unsigned long long EdsUInt64;
#else
typedef SInt64 EdsInt64;
typedef UInt64 EdsUInt64;
#endif
#endif

#elif TARGET_OS_LINUX

typedef char EdsInt8;
typedef unsigned char EdsUInt8;
typedef short EdsInt16;
typedef unsigned short EdsUInt16;
typedef int EdsInt32;
typedef unsigned int EdsUInt32;
#if defined __LP64__
typedef long EdsInt64;
typedef unsigned long EdsUInt64;
typedef unsigned long size_t;
#else
typedef long long EdsInt64;
typedef unsigned long long EdsUInt64;
#endif
#else
typedef char EdsInt8;
typedef unsigned char EdsUInt8;
typedef short EdsInt16;
typedef unsigned short EdsUInt16;
typedef long EdsInt32;
typedef unsigned long EdsUInt32;
typedef long long EdsInt64;
typedef unsigned long long EdsUInt64;
#endif

typedef float EdsFloat;
typedef double EdsDouble;

/*-----------------------------------------------------------------------------
 Error Types
-----------------------------------------------------------------------------*/
typedef EdsUInt32 EdsError;

/*-----------------------------------------------------------------------------
 Reference Types
-----------------------------------------------------------------------------*/
typedef struct __EdsObject *EdsBaseRef;

typedef EdsBaseRef EdsCameraListRef;
typedef EdsBaseRef EdsCameraRef;
typedef EdsBaseRef EdsVolumeRef;
typedef EdsBaseRef EdsDirectoryItemRef;

typedef EdsBaseRef EdsStreamRef;
typedef EdsStreamRef EdsImageRef;

typedef EdsBaseRef EdsEvfImageRef;

/*-----------------------------------------------------------------------------
 Data Types
-----------------------------------------------------------------------------*/
typedef enum
{
    kEdsDataType_Unknown = 0,
    kEdsDataType_Bool = 1,
    kEdsDataType_String = 2,
    kEdsDataType_Int8 = 3,
    kEdsDataType_UInt8 = 6,
    kEdsDataType_Int16 = 4,
    kEdsDataType_UInt16 = 7,
    kEdsDataType_Int32 = 8,
    kEdsDataType_UInt32 = 9,
    kEdsDataType_Int64 = 10,
    kEdsDataType_UInt64 = 11,
    kEdsDataType_Float = 12,
    kEdsDataType_Double = 13,
    kEdsDataType_ByteBlock = 14,
    kEdsDataType_Rational = 20,
    kEdsDataType_Point = 21,
    kEdsDataType_Rect = 22,
    kEdsDataType_Time = 23,

    kEdsDataType_Bool_Array = 30,
    kEdsDataType_Int8_Array = 31,
    kEdsDataType_Int16_Array = 32,
    kEdsDataType_Int32_Array = 33,
    kEdsDataType_UInt8_Array = 34,
    kEdsDataType_UInt16_Array = 35,
    kEdsDataType_UInt32_Array = 36,
    kEdsDataType_Rational_Array = 37,

    kEdsDataType_FocusInfo = 101,
    kEdsDataType_PictureStyleDesc,

} EdsDataType;

/*-----------------------------------------------------------------------------
 Property IDs
-----------------------------------------------------------------------------*/
typedef EdsUInt32 EdsPropertyID;
/*----------------------------------
 Camera Setting Properties
----------------------------------*/
#define kEdsPropID_Unknown 0x0000ffff

#define kEdsPropID_ProductName 0x00000002
#define kEdsPropID_OwnerName 0x00000004
#define kEdsPropID_MakerName 0x00000005
#define kEdsPropID_DateTime 0x00000006
#define kEdsPropID_FirmwareVersion 0x00000007
#define kEdsPropID_BatteryLevel 0x00000008
#define kEdsPropID_SaveTo 0x0000000b
#define kEdsPropID_CurrentStorage 0x0000000c
#define kEdsPropID_CurrentFolder 0x0000000d

#define kEdsPropID_BatteryQuality 0x00000010

#define kEdsPropID_BodyIDEx 0x00000015
#define kEdsPropID_HDDirectoryStructure 0x00000020

/*----------------------------------
 Image Properties
----------------------------------*/
#define kEdsPropID_ImageQuality 0x00000100
#define kEdsPropID_Orientation 0x00000102
#define kEdsPropID_ICCProfile 0x00000103
#define kEdsPropID_FocusInfo 0x00000104
#define kEdsPropID_WhiteBalance 0x00000106
#define kEdsPropID_ColorTemperature 0x00000107
#define kEdsPropID_WhiteBalanceShift 0x00000108
#define kEdsPropID_ColorSpace 0x0000010d
#define kEdsPropID_PictureStyle 0x00000114
#define kEdsPropID_PictureStyleDesc 0x00000115
#define kEdsPropID_PictureStyleCaption 0x00000200

/*----------------------------------
 Image GPS Properties
----------------------------------*/
#define kEdsPropID_GPSVersionID 0x00000800
#define kEdsPropID_GPSLatitudeRef 0x00000801
#define kEdsPropID_GPSLatitude 0x00000802
#define kEdsPropID_GPSLongitudeRef 0x00000803
#define kEdsPropID_GPSLongitude 0x00000804
#define kEdsPropID_GPSAltitudeRef 0x00000805
#define kEdsPropID_GPSAltitude 0x00000806
#define kEdsPropID_GPSTimeStamp 0x00000807
#define kEdsPropID_GPSSatellites 0x00000808
#define kEdsPropID_GPSStatus 0x00000809
#define kEdsPropID_GPSMapDatum 0x00000812
#define kEdsPropID_GPSDateStamp 0x0000081D

/*----------------------------------
 Capture Properties
----------------------------------*/
#define kEdsPropID_AEMode 0x00000400
#define kEdsPropID_DriveMode 0x00000401
#define kEdsPropID_ISOSpeed 0x00000402
#define kEdsPropID_MeteringMode 0x00000403
#define kEdsPropID_AFMode 0x00000404
#define kEdsPropID_Av 0x00000405
#define kEdsPropID_Tv 0x00000406
#define kEdsPropID_ExposureCompensation 0x00000407
#define kEdsPropID_FocalLength 0x00000409
#define kEdsPropID_AvailableShots 0x0000040a
#define kEdsPropID_Bracket 0x0000040b
#define kEdsPropID_WhiteBalanceBracket 0x0000040c
#define kEdsPropID_LensName 0x0000040d
#define kEdsPropID_AEBracket 0x0000040e
#define kEdsPropID_FEBracket 0x0000040f
#define kEdsPropID_ISOBracket 0x00000410
#define kEdsPropID_NoiseReduction 0x00000411
#define kEdsPropID_FlashOn 0x00000412
#define kEdsPropID_RedEye 0x00000413
#define kEdsPropID_FlashMode 0x00000414
#define kEdsPropID_LensStatus 0x00000416
#define kEdsPropID_Artist 0x00000418
#define kEdsPropID_Copyright 0x00000419
#define kEdsPropID_AEModeSelect 0x00000436
#define kEdsPropID_PowerZoom_Speed 0x00000444
#define kEdsPropID_ColorFilter 0x0000047f
#define kEdsPropID_DigitalZoomSetting 0x00000477
#define kEdsPropID_AfLockState 0x00000480
#define kEdsPropID_BrightnessSetting 0x00000483
/*----------------------------------
 EVF Properties
----------------------------------*/
#define kEdsPropID_Evf_OutputDevice 0x00000500
#define kEdsPropID_Evf_Mode 0x00000501
#define kEdsPropID_Evf_WhiteBalance 0x00000502
#define kEdsPropID_Evf_ColorTemperature 0x00000503
#define kEdsPropID_Evf_DepthOfFieldPreview 0x00000504

// EVF IMAGE DATA Properties
#define kEdsPropID_Evf_Zoom 0x00000507
#define kEdsPropID_Evf_ZoomPosition 0x00000508
#define kEdsPropID_Evf_Histogram 0x0000050A
#define kEdsPropID_Evf_ImagePosition 0x0000050B
#define kEdsPropID_Evf_HistogramStatus 0x0000050C
#define kEdsPropID_Evf_AFMode 0x0000050E

#define kEdsPropID_Record 0x00000510

#define kEdsPropID_Evf_HistogramY 0x00000515
#define kEdsPropID_Evf_HistogramR 0x00000516
#define kEdsPropID_Evf_HistogramG 0x00000517
#define kEdsPropID_Evf_HistogramB 0x00000518

#define kEdsPropID_Evf_CoordinateSystem 0x00000540
#define kEdsPropID_Evf_ZoomRect 0x00000541
#define kEdsPropID_Evf_ImageClipRect 0x00000545

#define kEdsPropID_Evf_PowerZoom_CurPosition 0x00000550
#define kEdsPropID_Evf_PowerZoom_MaxPosition 0x00000551
#define kEdsPropID_Evf_PowerZoom_MinPosition 0x00000552

/*----------------------------------
Limited Properties
----------------------------------*/
#define kEdsPropID_UTCTime 0x01000016
#define kEdsPropID_TimeZone 0x01000017
#define kEdsPropID_SummerTimeSetting 0x01000018
#define kEdsPropID_ManualWhiteBalanceData 0x01000204
#define kEdsPropID_TempStatus 0x01000415
#define kEdsPropID_MirrorLockUpState 0x01000421
#define kEdsPropID_FixedMovie 0x01000422
#define kEdsPropID_MovieParam 0x01000423
#define kEdsPropID_Aspect 0x01000431
#define kEdsPropID_ContinuousAfMode 0x01000433
#define kEdsPropID_MirrorUpSetting 0x01000438
#define kEdsPropID_MovieServoAf 0x0100043e
#define kEdsPropID_AutoPowerOffSetting 0x0100045e
#define kEdsPropID_AFEyeDetect 0x01000455
#define kEdsPropID_FocusShiftSetting 0x01000457
#define kEdsPropID_MovieHFRSetting 0x0100045d
#define kEdsPropID_AFTrackingObject 0x01000468
#define kEdsPropID_RegisterFocusEdge 0x0100046c
#define kEdsPropID_DriveFocusToEdge 0x0100046d
#define kEdsPropID_FocusPosition 0x0100046e
#define kEdsPropID_StillMovieDivideSetting 0x01000470
#define kEdsPropID_CardExtension 0x01000471
#define kEdsPropID_MovieCardExtension 0x01000472
#define kEdsPropID_StillCurrentMedia 0x01000473
#define kEdsPropID_MovieCurrentMedia 0x01000474
#define kEdsPropID_ApertureLockSetting 0x01000476
#define kEdsPropID_LensIsSetting 0x010004c0
#define kEdsPropID_ScreenDimmerTime 0x010004c1
#define kEdsPropID_ScreenOffTime 0x010004c2
#define kEdsPropID_ViewfinderOffTime 0x010004c3
#define kEdsPropID_Evf_ClickWBCoeffs 0x01000506
#define kEdsPropID_EVF_RollingPitching 0x01000544
#define kEdsPropID_Evf_VisibleRect 0x01000546

/*----------------------------------
 DC Properties
----------------------------------*/
#define kEdsPropID_DC_Zoom 0x00000600
#define kEdsPropID_DC_Strobe 0x00000601
#define kEdsPropID_LensBarrelStatus 0x00000605
/*-----------------------------------------------------------------------------
 Camera Commands
-----------------------------------------------------------------------------*/
typedef EdsUInt32 EdsCameraCommand;
/*----------------------------------
 Send Commands
----------------------------------*/
#define kEdsCameraCommand_TakePicture 0x00000000
#define kEdsCameraCommand_ExtendShutDownTimer 0x00000001
#define kEdsCameraCommand_BulbStart 0x00000002
#define kEdsCameraCommand_BulbEnd 0x00000003
#define kEdsCameraCommand_DoEvfAf 0x00000102
#define kEdsCameraCommand_DriveLensEvf 0x00000103
#define kEdsCameraCommand_DoClickWBEvf 0x00000104
#define kEdsCameraCommand_MovieSelectSwON 0x00000107
#define kEdsCameraCommand_MovieSelectSwOFF 0x00000108

#define kEdsCameraCommand_PressShutterButton 0x00000004
#define kEdsCameraCommand_RequestRollPitchLevel 0x00000109
#define kEdsCameraCommand_DrivePowerZoom 0x0000010d
#define kEdsCameraCommand_SetRemoteShootingMode 0x0000010f
#define kEdsCameraCommand_RequestSensorCleaning 0x00000112
#define kEdsCameraCommand_SetModeDialDisable 0x00000113

typedef enum
{
    kEdsCameraCommand_EvfAf_OFF = 0,
    kEdsCameraCommand_EvfAf_ON = 1,
} EdsEvfAf;

typedef enum
{
    kEdsCameraCommand_ShutterButton_OFF = 0x00000000,
    kEdsCameraCommand_ShutterButton_Halfway = 0x00000001,
    kEdsCameraCommand_ShutterButton_Completely = 0x00000003,
    kEdsCameraCommand_ShutterButton_Halfway_NonAF = 0x00010001,
    kEdsCameraCommand_ShutterButton_Completely_NonAF = 0x00010003,
} EdsShutterButton;

typedef EdsUInt32 EdsCameraStatusCommand;
/*----------------------------------
 Camera Status Commands
----------------------------------*/
#define kEdsCameraStatusCommand_UILock 0x00000000
#define kEdsCameraStatusCommand_UIUnLock 0x00000001
#define kEdsCameraStatusCommand_EnterDirectTransfer 0x00000002
#define kEdsCameraStatusCommand_ExitDirectTransfer 0x00000003

/*-----------------------------------------------------------------------------
 Camera Events
-----------------------------------------------------------------------------*/
typedef EdsUInt32 EdsPropertyEvent;
/*----------------------------------
 Property Event
----------------------------------*/

/* Notifies all property events. */
#define kEdsPropertyEvent_All 0x00000100

/* Notifies that a camera property value has been changed.
 The changed property can be retrieved from event data.
 The changed value can be retrieved by means of EdsGetPropertyData.
 If the property type is 0x0000FFFF, the changed property cannot be identified.
 Thus, retrieve all required properties repeatedly. */
#define kEdsPropertyEvent_PropertyChanged 0x00000101

/* Notifies of changes in the list of camera properties with configurable values.
 The list of configurable values for property IDs indicated in event data
  can be retrieved by means of EdsGetPropertyDesc.
 For type 1 protocol standard cameras, the property ID is identified as "Unknown"
  during notification.
  Thus, you must retrieve a list of configurable values for all properties and
  retrieve the property values repeatedly.
 (For details on properties for which you can retrieve a list of configurable
  properties,
  see the description of EdsGetPropertyDesc). */
#define kEdsPropertyEvent_PropertyDescChanged 0x00000102

typedef EdsUInt32 EdsObjectEvent;
/*----------------------------------
 Object Event
----------------------------------*/

/* Notifies all object events. */
#define kEdsObjectEvent_All 0x00000200

/* Notifies that the volume object (memory card) state (VolumeInfo)
  has been changed.
 Changed objects are indicated by event data.
 The changed value can be retrieved by means of EdsGetVolumeInfo.
 Notification of this event is not issued for type 1 protocol standard cameras. */
#define kEdsObjectEvent_VolumeInfoChanged 0x00000201

/* Notifies if the designated volume on a camera has been formatted.
 If notification of this event is received, get sub-items of the designated
  volume again as needed.
 Changed volume objects can be retrieved from event data.
 Objects cannot be identified on cameras earlier than the D30
  if files are added or deleted.
 Thus, these events are subject to notification. */
#define kEdsObjectEvent_VolumeUpdateItems 0x00000202

/* Notifies if many images are deleted in a designated folder on a camera.
 If notification of this event is received, get sub-items of the designated
  folder again as needed.
 Changed folders (specifically, directory item objects) can be retrieved
  from event data. */
#define kEdsObjectEvent_FolderUpdateItems 0x00000203

/* Notifies of the creation of objects such as new folders or files
  on a camera compact flash card or the like.
 This event is generated if the camera has been set to store captured
  images simultaneously on the camera and a computer,
  for example, but not if the camera is set to store images
  on the computer alone.
 Newly created objects are indicated by event data.
 Because objects are not indicated for type 1 protocol standard cameras,
  (that is, objects are indicated as NULL),
 you must again retrieve child objects under the camera object to
 identify the new objects. */
#define kEdsObjectEvent_DirItemCreated 0x00000204

/* Notifies of the deletion of objects such as folders or files on a camera
  compact flash card or the like.
 Deleted objects are indicated in event data.
 Because objects are not indicated for type 1 protocol standard cameras,
 you must again retrieve child objects under the camera object to
  identify deleted objects. */
#define kEdsObjectEvent_DirItemRemoved 0x00000205

/* Notifies that information of DirItem objects has been changed.
 Changed objects are indicated by event data.
 The changed value can be retrieved by means of EdsGetDirectoryItemInfo.
 Notification of this event is not issued for type 1 protocol standard cameras. */
#define kEdsObjectEvent_DirItemInfoChanged 0x00000206

/* Notifies that header information has been updated, as for rotation information
  of image files on the camera.
 If this event is received, get the file header information again, as needed.
 This function is for type 2 protocol standard cameras only. */
#define kEdsObjectEvent_DirItemContentChanged 0x00000207

/* Notifies that there are objects on a camera to be transferred to a computer.
 This event is generated after remote release from a computer or local release
  from a camera.
 If this event is received, objects indicated in the event data must be downloaded.
  Furthermore, if the application does not require the objects, instead
  of downloading them,
   execute EdsDownloadCancel and release resources held by the camera.
 The order of downloading from type 1 protocol standard cameras must be the order
  in which the events are received. */
#define kEdsObjectEvent_DirItemRequestTransfer 0x00000208

/* Notifies if the camera's direct transfer button is pressed.
 If this event is received, objects indicated in the event data must be downloaded.
 Furthermore, if the application does not require the objects, instead of
  downloading them,
  execute EdsDownloadCancel and release resources held by the camera.
 Notification of this event is not issued for type 1 protocol standard cameras. */
#define kEdsObjectEvent_DirItemRequestTransferDT 0x00000209

/* Notifies of requests from a camera to cancel object transfer
  if the button to cancel direct transfer is pressed on the camera.
 If the parameter is 0, it means that cancellation of transfer is requested for
  objects still not downloaded,
  with these objects indicated by kEdsObjectEvent_DirItemRequestTransferDT.
 Notification of this event is not issued for type 1 protocol standard cameras. */
#define kEdsObjectEvent_DirItemCancelTransferDT 0x0000020a

#define kEdsObjectEvent_VolumeAdded 0x0000020c
#define kEdsObjectEvent_VolumeRemoved 0x0000020d

typedef EdsUInt32 EdsStateEvent;
/*----------------------------------
 State Event
----------------------------------*/

/* Notifies all state events. */
#define kEdsStateEvent_All 0x00000300

/* Indicates that a camera is no longer connected to a computer,
 whether it was disconnected by unplugging a cord, opening
  the compact flash compartment,
  turning the camera off, auto shut-off, or by other means. */
#define kEdsStateEvent_Shutdown 0x00000301

/* Notifies of whether or not there are objects waiting to
  be transferred to a host computer.
 This is useful when ensuring all shot images have been transferred
 when the application is closed.
 Notification of this event is not issued for type 1 protocol
 standard cameras. */
#define kEdsStateEvent_JobStatusChanged 0x00000302

/* Notifies that the camera will shut down after a specific period.
 Generated only if auto shut-off is set.
 Exactly when notification is issued (that is, the number of
  seconds until shutdown) varies depending on the camera model.
 To continue operation without having the camera shut down,
 use EdsSendCommand to extend the auto shut-off timer.
 The time in seconds until the camera shuts down is returned
  as the initial value. */
#define kEdsStateEvent_WillSoonShutDown 0x00000303

/* As the counterpart event to kEdsStateEvent_WillSoonShutDown,
 this event notifies of updates to the number of seconds until
  a camera shuts down.
 After the update, the period until shutdown is model-dependent. */
#define kEdsStateEvent_ShutDownTimerUpdate 0x00000304

/* Notifies that a requested release has failed, due to focus
  failure or similar factors. */
#define kEdsStateEvent_CaptureError 0x00000305

/* Notifies of internal SDK errors.
 If this error event is received, the issuing device will probably
  not be able to continue working properly,
  so cancel the remote connection. */
#define kEdsStateEvent_InternalError 0x00000306

#define kEdsStateEvent_AfResult 0x00000309

#define kEdsStateEvent_BulbExposureTime 0x00000310

#define kEdsStateEvent_PowerZoomInfoChanged 0x00000311

/*-----------------------------------------------------------------------------
 Drive Lens
-----------------------------------------------------------------------------*/
typedef enum
{
    kEdsEvfDriveLens_Near1 = 0x00000001,
    kEdsEvfDriveLens_Near2 = 0x00000002,
    kEdsEvfDriveLens_Near3 = 0x00000003,
    kEdsEvfDriveLens_Far1 = 0x00008001,
    kEdsEvfDriveLens_Far2 = 0x00008002,
    kEdsEvfDriveLens_Far3 = 0x00008003,
} EdsEvfDriveLens;

/*-----------------------------------------------------------------------------
Drive PowerZoom
-----------------------------------------------------------------------------*/
typedef enum
{
    kEdsDrivePowerZoom_Stop = 0x00000000,
    kEdsDrivePowerZoom_LimitOff_Wide = 0x00000001,
    kEdsDrivePowerZoom_LimitOff_Tele = 0x00000002,
    kEdsDrivePowerZoom_LimitOn_Wide = 0x00000011,
    kEdsDrivePowerZoom_LimitOn_Tele = 0x00000012,

} EdsDrivePowerZoom;

/*-----------------------------------------------------------------------------
 Depth of Field Preview
-----------------------------------------------------------------------------*/
typedef enum
{
    kEdsEvfDepthOfFieldPreview_OFF = 0x00000000,
    kEdsEvfDepthOfFieldPreview_ON = 0x00000001,
} EdsEvfDepthOfFieldPreview;

/*-----------------------------------------------------------------------------
 Stream Seek Origins
-----------------------------------------------------------------------------*/
typedef enum
{
    kEdsSeek_Cur = 0,
    kEdsSeek_Begin,
    kEdsSeek_End,

} EdsSeekOrigin;

/*-----------------------------------------------------------------------------
 File and Propaties Access
-----------------------------------------------------------------------------*/
typedef enum
{
    kEdsAccess_Read = 0,
    kEdsAccess_Write,
    kEdsAccess_ReadWrite,
    kEdsAccess_Error = 0xFFFFFFFF,

} EdsAccess;

/*-----------------------------------------------------------------------------
 File Create Disposition
-----------------------------------------------------------------------------*/
typedef enum
{
    kEdsFileCreateDisposition_CreateNew = 0,
    kEdsFileCreateDisposition_CreateAlways,
    kEdsFileCreateDisposition_OpenExisting,
    kEdsFileCreateDisposition_OpenAlways,
    kEdsFileCreateDisposition_TruncateExsisting,

} EdsFileCreateDisposition;

/*-----------------------------------------------------------------------------
 Image Types
-----------------------------------------------------------------------------*/
typedef enum
{
    kEdsImageType_Unknown = 0x00000000,
    kEdsImageType_Jpeg = 0x00000001,
    kEdsImageType_CRW = 0x00000002,
    kEdsImageType_RAW = 0x00000004,
    kEdsImageType_CR2 = 0x00000006,
    kEdsImageType_HEIF = 0x00000008,

} EdsImageType;

/*-----------------------------------------------------------------------------
 Image Size
-----------------------------------------------------------------------------*/
typedef enum
{
    kEdsImageSize_Large = 0,
    kEdsImageSize_Middle = 1,
    kEdsImageSize_Small = 2,
    kEdsImageSize_Middle1 = 5,
    kEdsImageSize_Middle2 = 6,
    kEdsImageSize_Small1 = 14,
    kEdsImageSize_Small2 = 15,
    kEdsImageSize_Small3 = 16,
    kEdsImageSize_Unknown = 0xffffffff,

} EdsImageSize;

/*-----------------------------------------------------------------------------
 Image Compress Quality
-----------------------------------------------------------------------------*/
typedef enum
{
    kEdsCompressQuality_Normal = 2,
    kEdsCompressQuality_Fine = 3,
    kEdsCompressQuality_Lossless = 4,
    kEdsCompressQuality_SuperFine = 5,
    kEdsCompressQuality_Unknown = 0xffffffff,

} EdsCompressQuality;

/*-----------------------------------------------------------------------------
 Image Quality
-----------------------------------------------------------------------------*/

typedef enum
{
    /* Jpeg Only */
    EdsImageQuality_LJ = 0x0010ff0f,   /* Jpeg Large */
    EdsImageQuality_MJ = 0x0110ff0f,   /* Jpeg Middle */
    EdsImageQuality_M1J = 0x0510ff0f,  /* Jpeg Middle1 */
    EdsImageQuality_M1F = 0x0513FF0F,  /* Jpeg Middle1 Fine */
    EdsImageQuality_M1N = 0x0512FF0F,  /* Jpeg Middle1 Normal */
    EdsImageQuality_M2J = 0x0610ff0f,  /* Jpeg Middle2 */
    EdsImageQuality_M2F = 0x0613FF0F,  /* Jpeg Middle2 Fine */
    EdsImageQuality_M2N = 0x0612FF0F,  /* Jpeg Middle2 Normal */
    EdsImageQuality_SJ = 0x0210ff0f,   /* Jpeg Small */
    EdsImageQuality_S1J = 0x0e10ff0f,  /* Jpeg Small1 */
    EdsImageQuality_S2J = 0x0f10ff0f,  /* Jpeg Small2 */
    EdsImageQuality_LJF = 0x0013ff0f,  /* Jpeg Large Fine */
    EdsImageQuality_LJN = 0x0012ff0f,  /* Jpeg Large Normal */
    EdsImageQuality_MJF = 0x0113ff0f,  /* Jpeg Middle Fine */
    EdsImageQuality_MJN = 0x0112ff0f,  /* Jpeg Middle Normal */
    EdsImageQuality_SJF = 0x0213ff0f,  /* Jpeg Small Fine */
    EdsImageQuality_SJN = 0x0212ff0f,  /* Jpeg Small Normal */
    EdsImageQuality_S1JF = 0x0E13ff0f, /* Jpeg Small1 Fine */
    EdsImageQuality_S1JN = 0x0E12ff0f, /* Jpeg Small1 Normal */
    EdsImageQuality_S2JF = 0x0F13ff0f, /* Jpeg Small2 */
    EdsImageQuality_S3JF = 0x1013ff0f, /* Jpeg Small3 */

    /* RAW + Jpeg */
    EdsImageQuality_LR = 0x0064ff0f,     /* RAW */
    EdsImageQuality_LRLJF = 0x00640013,  /* RAW + Jpeg Large Fine */
    EdsImageQuality_LRLJN = 0x00640012,  /* RAW + Jpeg Large Normal */
    EdsImageQuality_LRMJF = 0x00640113,  /* RAW + Jpeg Middle Fine */
    EdsImageQuality_LRMJN = 0x00640112,  /* RAW + Jpeg Middle Normal */
    EdsImageQuality_LRSJF = 0x00640213,  /* RAW + Jpeg Small Fine */
    EdsImageQuality_LRSJN = 0x00640212,  /* RAW + Jpeg Small Normal */
    EdsImageQuality_LRS1JF = 0x00640E13, /* RAW + Jpeg Small1 Fine */
    EdsImageQuality_LRS1JN = 0x00640E12, /* RAW + Jpeg Small1 Normal */
    EdsImageQuality_LRS2JF = 0x00640F13, /* RAW + Jpeg Small2 */
    EdsImageQuality_LRS3JF = 0x00641013, /* RAW + Jpeg Small3 */

    EdsImageQuality_LRLJ = 0x00640010,  /* RAW + Jpeg Large */
    EdsImageQuality_LRMJ = 0x00640110,  /* RAW + Jpeg Middle */
    EdsImageQuality_LRM1J = 0x00640510, /* RAW + Jpeg Middle1 */
    EdsImageQuality_LRM1F = 0x00640513, /* RAW + Jpeg Middle1 Fine */
    EdsImageQuality_LRM1N = 0x00640512, /* RAW + Jpeg Middle1 Normal */
    EdsImageQuality_LRM2J = 0x00640610, /* RAW + Jpeg Middle2 */
    EdsImageQuality_LRM2F = 0x00640613, /* RAW + Jpeg Middle2 Fine */
    EdsImageQuality_LRM2N = 0x00640612, /* RAW + Jpeg Middle2 Normal */
    EdsImageQuality_LRSJ = 0x00640210,  /* RAW + Jpeg Small */
    EdsImageQuality_LRS1J = 0x00640e10, /* RAW + Jpeg Small1 */
    EdsImageQuality_LRS2J = 0x00640f10, /* RAW + Jpeg Small2 */

    /* MRAW(SRAW1) + Jpeg */
    EdsImageQuality_MR = 0x0164ff0f,     /* MRAW(SRAW1) */
    EdsImageQuality_MRLJF = 0x01640013,  /* MRAW(SRAW1) + Jpeg Large Fine */
    EdsImageQuality_MRLJN = 0x01640012,  /* MRAW(SRAW1) + Jpeg Large Normal */
    EdsImageQuality_MRMJF = 0x01640113,  /* MRAW(SRAW1) + Jpeg Middle Fine */
    EdsImageQuality_MRMJN = 0x01640112,  /* MRAW(SRAW1) + Jpeg Middle Normal */
    EdsImageQuality_MRSJF = 0x01640213,  /* MRAW(SRAW1) + Jpeg Small Fine */
    EdsImageQuality_MRSJN = 0x01640212,  /* MRAW(SRAW1) + Jpeg Small Normal */
    EdsImageQuality_MRS1JF = 0x01640E13, /* MRAW(SRAW1) + Jpeg Small1 Fine */
    EdsImageQuality_MRS1JN = 0x01640E12, /* MRAW(SRAW1) + Jpeg Small1 Normal */
    EdsImageQuality_MRS2JF = 0x01640F13, /* MRAW(SRAW1) + Jpeg Small2 */
    EdsImageQuality_MRS3JF = 0x01641013, /* MRAW(SRAW1) + Jpeg Small3 */

    EdsImageQuality_MRLJ = 0x01640010,  /* MRAW(SRAW1) + Jpeg Large */
    EdsImageQuality_MRM1J = 0x01640510, /* MRAW(SRAW1) + Jpeg Middle1 */
    EdsImageQuality_MRM1F = 0x01640513, /* MRAW(SRAW1) + Jpeg Middle1 Fine */
    EdsImageQuality_MRM1N = 0x01640512, /* MRAW(SRAW1) + Jpeg Middle1 Normal */
    EdsImageQuality_MRM2J = 0x01640610, /* MRAW(SRAW1) + Jpeg Middle2 */
    EdsImageQuality_MRM2F = 0x01640613, /* MRAW(SRAW1) + Jpeg Middle2 Fine */
    EdsImageQuality_MRM2N = 0x01640612, /* MRAW(SRAW1) + Jpeg Middle2 Normal */
    EdsImageQuality_MRSJ = 0x01640210,  /* MRAW(SRAW1) + Jpeg Small */

    /* SRAW(SRAW2) + Jpeg */
    EdsImageQuality_SR = 0x0264ff0f,     /* SRAW(SRAW2) */
    EdsImageQuality_SRLJF = 0x02640013,  /* SRAW(SRAW2) + Jpeg Large Fine */
    EdsImageQuality_SRLJN = 0x02640012,  /* SRAW(SRAW2) + Jpeg Large Normal */
    EdsImageQuality_SRMJF = 0x02640113,  /* SRAW(SRAW2) + Jpeg Middle Fine */
    EdsImageQuality_SRMJN = 0x02640112,  /* SRAW(SRAW2) + Jpeg Middle Normal */
    EdsImageQuality_SRSJF = 0x02640213,  /* SRAW(SRAW2) + Jpeg Small Fine */
    EdsImageQuality_SRSJN = 0x02640212,  /* SRAW(SRAW2) + Jpeg Small Normal */
    EdsImageQuality_SRS1JF = 0x02640E13, /* SRAW(SRAW2) + Jpeg Small1 Fine */
    EdsImageQuality_SRS1JN = 0x02640E12, /* SRAW(SRAW2) + Jpeg Small1 Normal */
    EdsImageQuality_SRS2JF = 0x02640F13, /* SRAW(SRAW2) + Jpeg Small2 */
    EdsImageQuality_SRS3JF = 0x02641013, /* SRAW(SRAW2) + Jpeg Small3 */

    EdsImageQuality_SRLJ = 0x02640010,  /* SRAW(SRAW2) + Jpeg Large */
    EdsImageQuality_SRM1J = 0x02640510, /* SRAW(SRAW2) + Jpeg Middle1 */
    EdsImageQuality_SRM1F = 0x02640513, /* SRAW(SRAW2) + Jpeg Middle1 Fine */
    EdsImageQuality_SRM1N = 0x02640512, /* SRAW(SRAW2) + Jpeg Middle1 Normal */
    EdsImageQuality_SRM2J = 0x02640610, /* SRAW(SRAW2) + Jpeg Middle2 */
    EdsImageQuality_SRM2F = 0x02640613, /* SRAW(SRAW2) + Jpeg Middle2 Fine */
    EdsImageQuality_SRM2N = 0x02640612, /* SRAW(SRAW2) + Jpeg Middle2 Normal */
    EdsImageQuality_SRSJ = 0x02640210,  /* SRAW(SRAW2) + Jpeg Small */

    /* CRAW + Jpeg */
    EdsImageQuality_CR = 0x0063ff0f,     /* CRAW */
    EdsImageQuality_CRLJF = 0x00630013,  /* CRAW + Jpeg Large Fine */
    EdsImageQuality_CRMJF = 0x00630113,  /* CRAW + Jpeg Middle Fine  */
    EdsImageQuality_CRM1JF = 0x00630513, /* CRAW + Jpeg Middle1 Fine  */
    EdsImageQuality_CRM2JF = 0x00630613, /* CRAW + Jpeg Middle2 Fine  */
    EdsImageQuality_CRSJF = 0x00630213,  /* CRAW + Jpeg Small Fine  */
    EdsImageQuality_CRS1JF = 0x00630E13, /* CRAW + Jpeg Small1 Fine  */
    EdsImageQuality_CRS2JF = 0x00630F13, /* CRAW + Jpeg Small2 Fine  */
    EdsImageQuality_CRS3JF = 0x00631013, /* CRAW + Jpeg Small3 Fine  */
    EdsImageQuality_CRLJN = 0x00630012,  /* CRAW + Jpeg Large Normal */
    EdsImageQuality_CRMJN = 0x00630112,  /* CRAW + Jpeg Middle Normal */
    EdsImageQuality_CRM1JN = 0x00630512, /* CRAW + Jpeg Middle1 Normal */
    EdsImageQuality_CRM2JN = 0x00630612, /* CRAW + Jpeg Middle2 Normal */
    EdsImageQuality_CRSJN = 0x00630212,  /* CRAW + Jpeg Small Normal */
    EdsImageQuality_CRS1JN = 0x00630E12, /* CRAW + Jpeg Small1 Normal */

    EdsImageQuality_CRLJ = 0x00630010,  /* CRAW + Jpeg Large */
    EdsImageQuality_CRMJ = 0x00630110,  /* CRAW + Jpeg Middle */
    EdsImageQuality_CRM1J = 0x00630510, /* CRAW + Jpeg Middle1 */
    EdsImageQuality_CRM2J = 0x00630610, /* CRAW + Jpeg Middle2 */
    EdsImageQuality_CRSJ = 0x00630210,  /* CRAW + Jpeg Small */
    EdsImageQuality_CRS1J = 0x00630e10, /* CRAW + Jpeg Small1 */
    EdsImageQuality_CRS2J = 0x00630f10, /* CRAW + Jpeg Small2 */

    /* HEIF */
    EdsImageQuality_HEIFL = 0x0080ff0f,  /* HEIF Large */
    EdsImageQuality_HEIFM = 0x0180ff0f,  /* HEIF Middle */
    EdsImageQuality_HEIFM1 = 0x0580FF0F, /* HEIF Middle1 */
    EdsImageQuality_HEIFM2 = 0x0680FF0F, /* HEIF Middle2 */

    EdsImageQuality_HEIFLF = 0x0083ff0f,    /* HEIF Large Fine */
    EdsImageQuality_HEIFLN = 0x0082ff0f,    /* HEIF Large Normal */
    EdsImageQuality_HEIFMF = 0x0183ff0f,    /* HEIF Middle Fine */
    EdsImageQuality_HEIFMN = 0x0182ff0f,    /* HEIF Middle Normal */
    EdsImageQuality_HEIFS1 = 0x0e80ff0f,    /* HEIF Small1 */
    EdsImageQuality_HEIFS1F = 0x0e83ff0f,   /* HEIF Small1 Fine */
    EdsImageQuality_HEIFS1N = 0x0e82ff0f,   /* HEIF Small1 Normal */
    EdsImageQuality_HEIFS2 = 0x0f80ff0f,    /* HEIF Small2 */
    EdsImageQuality_HEIFS2F = 0x0f83ff0f,   /* HEIF Small2 Fine */
    EdsImageQuality_RHEIFL = 0x00640080,    /* RAW  + HEIF Large */
    EdsImageQuality_RHEIFLF = 0x00640083,   /* RAW + HEIF Large Fine */
    EdsImageQuality_RHEIFLN = 0x00640082,   /* RAW + HEIF Large Normal */
    EdsImageQuality_RHEIFM = 0x00640180,    /* RAW + HEIF Middle */
    EdsImageQuality_RHEIFM1 = 0x00640580,   /* RAW + HEIF Middle1 */
    EdsImageQuality_RHEIFM2 = 0x00640680,   /* RAW + HEIF Middle2 */
    EdsImageQuality_RHEIFMF = 0x00640183,   /* RAW + HEIF Middle Fine */
    EdsImageQuality_RHEIFMN = 0x00640182,   /* RAW + HEIF Middle Normal */
    EdsImageQuality_RHEIFS1 = 0x00640e80,   /* RAW + HEIF Small1 */
    EdsImageQuality_RHEIFS1F = 0x00640e83,  /* RAW + HEIF Small1 Fine */
    EdsImageQuality_RHEIFS1N = 0x00640e82,  /* RAW + HEIF Small1 Normal */
    EdsImageQuality_RHEIFS2 = 0x00640f80,   /* RAW + HEIF Small2 */
    EdsImageQuality_RHEIFS2F = 0x00640f83,  /* RAW + HEIF Small2 Fine */
    EdsImageQuality_CRHEIFL = 0x00630080,   /* CRAW + HEIF Large */
    EdsImageQuality_CRHEIFLF = 0x00630083,  /* CRAW + HEIF Large Fine */
    EdsImageQuality_CRHEIFLN = 0x00630082,  /* CRAW + HEIF Large Normal */
    EdsImageQuality_CRHEIFM = 0x00630180,   /* CRAW + HEIF Middle */
    EdsImageQuality_CRHEIFMF = 0x00630183,  /* CRAW + HEIF Middle Fine */
    EdsImageQuality_CRHEIFMN = 0x00630182,  /* CRAW + HEIF Middle Normal */
    EdsImageQuality_CRHEIFM1 = 0x00630580,  /* CRAW + HEIF Middle1 */
    EdsImageQuality_CRHEIFM2 = 0x00630680,  /* CRAW + HEIF Middle2 */
    EdsImageQuality_CRHEIFS1 = 0x00630e80,  /* CRAW + HEIF Small1 */
    EdsImageQuality_CRHEIFS1F = 0x00630e83, /* CRAW + HEIF Small1 Fine */
    EdsImageQuality_CRHEIFS1N = 0x00630e82, /* CRAW + HEIF Small1 Normal */
    EdsImageQuality_CRHEIFS2 = 0x00630f80,  /* CRAW + HEIF Small2 */
    EdsImageQuality_CRHEIFS2F = 0x00630f83, /* CRAW + HEIF Small2 Fine */

    EdsImageQuality_Unknown = 0xffffffff,
} EdsImageQuality;

/*-----------------------------------------------------------------------------
 Image Source
-----------------------------------------------------------------------------*/
typedef enum
{
    kEdsImageSrc_FullView = 0,
    kEdsImageSrc_Thumbnail,
    kEdsImageSrc_Preview,
    kEdsImageSrc_RAWThumbnail,
    kEdsImageSrc_RAWFullView,

} EdsImageSource;

/*-----------------------------------------------------------------------------
 Target Image Types
-----------------------------------------------------------------------------*/
typedef enum
{
    kEdsTargetImageType_Unknown = 0x00000000,
    kEdsTargetImageType_Jpeg = 0x00000001,
    kEdsTargetImageType_TIFF = 0x00000007,
    kEdsTargetImageType_TIFF16 = 0x00000008,
    kEdsTargetImageType_RGB = 0x00000009,
    kEdsTargetImageType_RGB16 = 0x0000000A,
    kEdsTargetImageType_DIB = 0x0000000B

} EdsTargetImageType;

/*-----------------------------------------------------------------------------
 Progress Option
-----------------------------------------------------------------------------*/
typedef enum
{
    kEdsProgressOption_NoReport = 0,
    kEdsProgressOption_Done,
    kEdsProgressOption_Periodically,

} EdsProgressOption;

/*-----------------------------------------------------------------------------
 File attribute
-----------------------------------------------------------------------------*/
typedef enum
{
    kEdsFileAttribute_Normal = 0x00000000,
    kEdsFileAttribute_ReadOnly = 0x00000001,
    kEdsFileAttribute_Hidden = 0x00000002,
    kEdsFileAttribute_System = 0x00000004,
    kEdsFileAttribute_Archive = 0x00000020,

} EdsFileAttributes;

/*-----------------------------------------------------------------------------
ObjectFormat Code
-----------------------------------------------------------------------------*/
typedef enum
{
    kEdsObjectFormat_Unknown = 0x00000000,
    kEdsObjectFormat_Jpeg = 0x3801,
    kEdsObjectFormat_CR2 = 0xB103,
    kEdsObjectFormat_MP4 = 0xB982,
    kEdsObjectFormat_CR3 = 0xB108,
    kEdsObjectFormat_HEIF_CODE = 0xB10B,
} EdsObjectFormat;

/*-----------------------------------------------------------------------------
 Battery level
-----------------------------------------------------------------------------*/
typedef enum
{
    kEdsBatteryLevel2_Empty = 0,
    kEdsBatteryLevel2_Low = 9,
    kEdsBatteryLevel2_Half = 49,
    kEdsBatteryLevel2_Normal = 80,
    kEdsBatteryLevel2_Hi = 69,
    kEdsBatteryLevel2_Quarter = 19,
    kEdsBatteryLevel2_Error = 0,
    kEdsBatteryLevel2_BCLevel = 0,
    kEdsBatteryLevel2_AC = 0xFFFFFFFF,
    kEdsBatteryLevel2_Unknown = 0xFFFFFFFE,
} EdsBatteryLevel2;

/*-----------------------------------------------------------------------------
 Save To
-----------------------------------------------------------------------------*/
typedef enum
{
    kEdsSaveTo_Camera = 1,
    kEdsSaveTo_Host = 2,
    kEdsSaveTo_Both = kEdsSaveTo_Camera | kEdsSaveTo_Host,

} EdsSaveTo;

/*-----------------------------------------------------------------------------
 StorageType
-----------------------------------------------------------------------------*/
typedef enum
{
    kEdsStorageType_Non = 0,
    kEdsStorageType_CF = 1,
    kEdsStorageType_SD = 2,
    kEdsStorageType_HD = 4,
    kEdsStorageType_CFast = 5,
    kEdsStorageType_CFe = 7,

} EdsStorageType;

/*-----------------------------------------------------------------------------
 White Balance
-----------------------------------------------------------------------------*/
typedef enum
{
    kEdsWhiteBalance_Auto = 0,
    kEdsWhiteBalance_Daylight = 1,
    kEdsWhiteBalance_Cloudy = 2,
    kEdsWhiteBalance_Tungsten = 3,
    kEdsWhiteBalance_Fluorescent = 4,
    kEdsWhiteBalance_Strobe = 5,
    kEdsWhiteBalance_WhitePaper = 6,
    kEdsWhiteBalance_Shade = 8,
    kEdsWhiteBalance_ColorTemp = 9,
    kEdsWhiteBalance_PCSet1 = 10,
    kEdsWhiteBalance_PCSet2 = 11,
    kEdsWhiteBalance_PCSet3 = 12,
    kEdsWhiteBalance_WhitePaper2 = 15,
    kEdsWhiteBalance_WhitePaper3 = 16,
    kEdsWhiteBalance_WhitePaper4 = 18,
    kEdsWhiteBalance_WhitePaper5 = 19,
    kEdsWhiteBalance_PCSet4 = 20,
    kEdsWhiteBalance_PCSet5 = 21,
    kEdsWhiteBalance_AwbWhite = 23,
    kEdsWhiteBalance_Click = -1,
    kEdsWhiteBalance_Pasted = -2,

} EdsWhiteBalance;

/*-----------------------------------------------------------------------------
 Color Space
-----------------------------------------------------------------------------*/
typedef enum
{
    kEdsColorSpace_sRGB = 1,
    kEdsColorSpace_AdobeRGB = 2,
    kEdsColorSpace_Unknown = 0xffffffff,

} EdsColorSpace;

/*-----------------------------------------------------------------------------
 PictureStyle
-----------------------------------------------------------------------------*/
typedef enum
{
    kEdsPictureStyle_Standard = 0x0081,
    kEdsPictureStyle_Portrait = 0x0082,
    kEdsPictureStyle_Landscape = 0x0083,
    kEdsPictureStyle_Neutral = 0x0084,
    kEdsPictureStyle_Faithful = 0x0085,
    kEdsPictureStyle_Monochrome = 0x0086,
    kEdsPictureStyle_Auto = 0x0087,
    kEdsPictureStyle_FineDetail = 0x0088,
    kEdsPictureStyle_User1 = 0x0021,
    kEdsPictureStyle_User2 = 0x0022,
    kEdsPictureStyle_User3 = 0x0023,
    kEdsPictureStyle_PC1 = 0x0041,
    kEdsPictureStyle_PC2 = 0x0042,
    kEdsPictureStyle_PC3 = 0x0043,

} EdsPictureStyle;

/*-----------------------------------------------------------------------------
 Transfer Option
-----------------------------------------------------------------------------*/
typedef enum
{
    kEdsTransferOption_ByDirectTransfer = 1,
    kEdsTransferOption_ByRelease = 2,
    kEdsTransferOption_ToDesktop = 0x00000100,

} EdsTransferOption;

/*-----------------------------------------------------------------------------
 AE Mode
-----------------------------------------------------------------------------*/
typedef enum
{
    kEdsAEMode_Program = 0x00,
    kEdsAEMode_Tv = 0x01,
    kEdsAEMode_Av = 0x02,
    kEdsAEMode_Manual = 0x03,
    kEdsAEMode_Bulb = 0x04,
    kEdsAEMode_A_DEP = 0x05,
    kEdsAEMode_DEP = 0x06,
    kEdsAEMode_Custom = 0x07,
    kEdsAEMode_Lock = 0x08,
    kEdsAEMode_Green = 0x09,
    kEdsAEMode_NightPortrait = 0x0A,
    kEdsAEMode_Sports = 0x0B,
    kEdsAEMode_Portrait = 0x0C,
    kEdsAEMode_Landscape = 0x0D,
    kEdsAEMode_Closeup = 0x0E,
    kEdsAEMode_FlashOff = 0x0F,
    kEdsAEMode_CreativeAuto = 0x13,
    kEdsAEMode_Movie = 0x14,
    kEdsAEMode_PhotoInMovie = 0x15,
    kEdsAEMode_SceneIntelligentAuto = 0x16,
    kEdsAEMode_SCN = 0x19,
    kEdsAEMode_NightScenes = 0x17,
    kEdsAEMode_BacklitScenes = 0x18,
    kEdsAEMode_Children = 0x1A,
    kEdsAEMode_Food = 0x1B,
    kEdsAEMode_CandlelightPortraits = 0x1C,
    kEdsAEMode_CreativeFilter = 0x1D,
    kEdsAEMode_RoughMonoChrome = 0x1E,
    kEdsAEMode_SoftFocus = 0x1F,
    kEdsAEMode_ToyCamera = 0x20,
    kEdsAEMode_Fisheye = 0x21,
    kEdsAEMode_WaterColor = 0x22,
    kEdsAEMode_Miniature = 0x23,
    kEdsAEMode_Hdr_Standard = 0x24,
    kEdsAEMode_Hdr_Vivid = 0x25,
    kEdsAEMode_Hdr_Bold = 0x26,
    kEdsAEMode_Hdr_Embossed = 0x27,
    kEdsAEMode_Movie_Fantasy = 0x28,
    kEdsAEMode_Movie_Old = 0x29,
    kEdsAEMode_Movie_Memory = 0x2A,
    kEdsAEMode_Movie_DirectMono = 0x2B,
    kEdsAEMode_Movie_Mini = 0x2C,
    kEdsAEMode_PanningAssist = 0x2D,
    kEdsAEMode_GroupPhoto = 0x2E,
    kEdsAEMode_Myself = 0x32,
    kEdsAEMode_PlusMovieAuto = 0x33,
    kEdsAEMode_SmoothSkin = 0x34,
    kEdsAEMode_Panorama = 0x35,
    kEdsAEMode_Silent = 0x36,
    kEdsAEMode_Flexible = 0x37,
    kEdsAEMode_OilPainting = 0x38,
    kEdsAEMode_Fireworks = 0x39,
    kEdsAEMode_StarPortrait = 0x3A,
    kEdsAEMode_StarNightscape = 0x3B,
    kEdsAEMode_StarTrails = 0x3C,
    kEdsAEMode_StarTimelapseMovie = 0x3D,
    kEdsAEMode_BackgroundBlur = 0x3E,
    kEdsAEMode_VideoBlog = 0x3F,
    kEdsAEMode_Unknown = 0xffffffff,

} EdsAEMode;

/*-----------------------------------------------------------------------------
 Bracket
-----------------------------------------------------------------------------*/
typedef enum
{
    kEdsBracket_AEB = 0x01,
    kEdsBracket_ISOB = 0x02,
    kEdsBracket_WBB = 0x04,
    kEdsBracket_FEB = 0x08,
    kEdsBracket_Unknown = 0xffffffff,

} EdsBracket;

/*-----------------------------------------------------------------------------
 EVF Output Device [Flag]
-----------------------------------------------------------------------------*/
typedef enum
{
    kEdsEvfOutputDevice_TFT = 1,
    kEdsEvfOutputDevice_PC = 2,
    kEdsEvfOutputDevice_PC_Small = 8,
} EdsEvfOutputDevice;

/*-----------------------------------------------------------------------------
 EVF Zoom
-----------------------------------------------------------------------------*/
typedef enum
{
    kEdsEvfZoom_Fit = 1,
    kEdsEvfZoom_x5 = 5,
    kEdsEvfZoom_x6 = 6,
    kEdsEvfZoom_x10 = 10,
    kEdsEvfZoom_x15 = 15,
} EdsEvfZoom;

/*-----------------------------------------------------------------------------
 EVF AF Mode
-----------------------------------------------------------------------------*/
typedef enum
{
    Evf_AFMode_Quick = 0x00,
    Evf_AFMode_Live = 0x01,
    Evf_AFMode_LiveFace = 0x02,
    Evf_AFMode_LiveMulti = 0x03,
    Evf_AFMode_LiveZone = 0x04,
    Evf_AFMode_LiveSingleExpandCross = 0x05,
    Evf_AFMode_LiveSingleExpandAround = 0x06,
    Evf_AFMode_LiveZoneLargeH = 0x07,
    Evf_AFMode_LiveZoneLargeV = 0x08,
    Evf_AFMode_LiveCatchAF = 0x09,
    Evf_AFMode_LiveSpotAF = 0x0a,
    Evf_AFMode_FlexibleZone1 = 0x0b,
    Evf_AFMode_FlexibleZone2 = 0x0c,
    Evf_AFMode_FlexibleZone3 = 0x0d,
    Evf_AFMode_WholeArea = 0x0e,
    Evf_AFMode_NoTraking_Spot = 0x0f,
    Evf_AFMode_NoTraking_1Point = 0x10,
    Evf_AFMode_NoTraking_ExpandCross = 0x11,
    Evf_AFMode_NoTraking_ExpandAround = 0x12,
} EdsEvfAFMode;

/*-----------------------------------------------------------------------------
 Strobo Mode
-----------------------------------------------------------------------------*/
typedef enum
{
    kEdsStroboModeInternal = 0,
    kEdsStroboModeExternalETTL = 1,
    kEdsStroboModeExternalATTL = 2,
    kEdsStroboModeExternalTTL = 3,
    kEdsStroboModeExternalAuto = 4,
    kEdsStroboModeExternalManual = 5,
    kEdsStroboModeManual = 6,
} EdsStroboMode;

/*-----------------------------------------------------------------------------
 ETTL-II Mode
-----------------------------------------------------------------------------*/
typedef enum
{
    kEdsETTL2ModeEvaluative = 0,
    kEdsETTL2ModeAverage = 1,
} EdsETTL2Mode;

/*-----------------------------------------------------------------------------
 DC Strobe
-----------------------------------------------------------------------------*/
typedef enum
{
    kEdsDcStrobeAuto = 0,
    kEdsDcStrobeOn = 1,
    kEdsDcStrobeSlowsynchro = 2,
    kEdsDcStrobeOff = 3,
} EdsDcStrobe;

/*-----------------------------------------------------------------------------
 DC Lens Barrel State
-----------------------------------------------------------------------------*/
typedef enum
{
    kDcLensBarrelStateInner = 0,
    kDcLensBarrelStateOuter = 1,
} EdsDcLensBarrelState;

/*-----------------------------------------------------------------------------
 DC Remote Shooting Mode
-----------------------------------------------------------------------------*/
typedef enum
{
    kDcRemoteShootingModeStop = 0,
    kDcRemoteShootingModeStart = 1,
} EdsDcRemoteShootingMode;

/*-----------------------------------------------------------------------------
 Mirror Lockup State
-----------------------------------------------------------------------------*/
typedef enum
{
    kEdsMirrorLockupStateDisable = 0,
    kEdsMirrorLockupStateEnable = 1,
    kEdsMirrorLockupStateDuringShooting = 2,
} EdsMirrorLockupState;

/*-----------------------------------------------------------------------------
 Mirror Up Setting
-----------------------------------------------------------------------------*/
typedef enum
{
    kEdsMirrorUpSettingOff = 0,
    kEdsMirrorUpSettingOn = 1,
} EdsMirrorUpSetting;

/****************************************************************************
 Definition of base Structures
******************************************************************************/
/*-----------------------------------------------------------------------------
 Point
-----------------------------------------------------------------------------*/
typedef struct tagEdsPoint
{
    EdsInt32 x;
    EdsInt32 y;

} EdsPoint;

/*-----------------------------------------------------------------------------
 Size
-----------------------------------------------------------------------------*/
typedef struct tagEdsSize
{
    EdsInt32 width;
    EdsInt32 height;

} EdsSize;

/*-----------------------------------------------------------------------------
 Rectangle
-----------------------------------------------------------------------------*/
typedef struct tagEdsRect
{
    EdsPoint point;
    EdsSize size;

} EdsRect;

/*-----------------------------------------------------------------------------
 Rational
-----------------------------------------------------------------------------*/
typedef struct tagEdsRational
{
    EdsInt32 numerator;
    EdsUInt32 denominator;
} EdsRational;

/*-----------------------------------------------------------------------------
 Time
-----------------------------------------------------------------------------*/
typedef struct tagEdsTime
{
    EdsUInt32 year;
    EdsUInt32 month;
    EdsUInt32 day;
    EdsUInt32 hour;
    EdsUInt32 minute;
    EdsUInt32 second;
    EdsUInt32 milliseconds;

} EdsTime;

/*-----------------------------------------------------------------------------
GpsMetaData
-----------------------------------------------------------------------------*/
typedef struct tagEdsGpsMetaData
{
    EdsUInt8 latitudeRef;
    EdsUInt8 longitudeRef;
    EdsUInt8 altitudeRef;
    EdsUInt8 status;
    EdsRational latitude[3];
    EdsRational longitude[3];
    EdsRational altitude;
    EdsRational timeStamp[3];
    EdsUInt16 dateStampYear;
    EdsUInt8 dateStampMonth;
    EdsUInt8 dateStampDay;
} EdsGpsMetaData;

/*-----------------------------------------------------------------------------
 Device Info
-----------------------------------------------------------------------------*/
typedef struct tagEdsDeviceInfo
{
    EdsChar szPortName[EDS_MAX_NAME];
    EdsChar szDeviceDescription[EDS_MAX_NAME];
    EdsUInt32 deviceSubType;
    EdsUInt32 reserved;
} EdsDeviceInfo;

/*-----------------------------------------------------------------------------
 Volume Info
-----------------------------------------------------------------------------*/
typedef struct tagEdsVolumeInfo
{
    EdsUInt32 storageType;
    EdsAccess access;
    EdsUInt64 maxCapacity;
    EdsUInt64 freeSpaceInBytes;
    EdsChar szVolumeLabel[EDS_MAX_NAME];

} EdsVolumeInfo;

/*-----------------------------------------------------------------------------
 DirectoryItem Info
-----------------------------------------------------------------------------*/
typedef struct tagEdsDirectoryItemInfo
{
    EdsUInt64 size;
    EdsBool isFolder;
    EdsUInt32 groupID;
    EdsUInt32 option;
    EdsChar szFileName[EDS_MAX_NAME];

    EdsUInt32 format;
    EdsUInt32 dateTime;

} EdsDirectoryItemInfo;

/*-----------------------------------------------------------------------------
 Image Info
-----------------------------------------------------------------------------*/
typedef struct tagEdsImageInfo
{
    EdsUInt32 width;
    EdsUInt32 height;
    EdsUInt32 numOfComponents;
    EdsUInt32 componentDepth;
    EdsRect effectiveRect;
    EdsUInt32 reserved1;
    EdsUInt32 reserved2;

} EdsImageInfo;

/*-----------------------------------------------------------------------------
 SaveImage Setting
-----------------------------------------------------------------------------*/
typedef struct tagEdsSaveImageSetting
{
    EdsUInt32 JPEGQuality;
    EdsStreamRef iccProfileStream;
    EdsUInt32 reserved;

} EdsSaveImageSetting;

/*-----------------------------------------------------------------------------
 Property Desc
-----------------------------------------------------------------------------*/
typedef struct tagEdsPropertyDesc
{
    EdsInt32 form;
    EdsInt32 access;
    EdsInt32 numElements;
    EdsInt32 propDesc[128];

} EdsPropertyDesc;

/*-----------------------------------------------------------------------------
 Picture Style Desc
-----------------------------------------------------------------------------*/
typedef struct tagEdsPictureStyleDesc
{
    EdsInt32 contrast;
    EdsUInt32 sharpness;
    EdsInt32 saturation;
    EdsInt32 colorTone;
    EdsUInt32 filterEffect;
    EdsUInt32 toningEffect;
    EdsUInt32 sharpFineness;
    EdsUInt32 sharpThreshold;
} EdsPictureStyleDesc;

/*-----------------------------------------------------------------------------
 Focus Info
-----------------------------------------------------------------------------*/
typedef struct tagEdsFrameDesc
{
    EdsUInt32 valid;
    EdsUInt32 selected;
    EdsUInt32 justFocus;
    EdsRect rect;
    EdsUInt32 reserved;

} EdsFocusPoint;

typedef struct tagEdsFocusInfo
{
    EdsRect imageRect;
    EdsUInt32 pointNumber;
    EdsFocusPoint focusPoint[1053];
    EdsUInt32 executeMode;

} EdsFocusInfo;

/*-----------------------------------------------------------------------------
 User WhiteBalance (PC set1,2,3)/ User ToneCurve / User PictureStyle dataset
-----------------------------------------------------------------------------*/
typedef struct tagEdsUsersetData
{
    EdsUInt32 valid;
    EdsUInt32 dataSize;
    EdsChar szCaption[32];
    EdsUInt8 data[1];

} EdsUsersetData;

/*-----------------------------------------------------------------------------
 Capacity
-----------------------------------------------------------------------------*/
typedef struct tagEdsCapacity
{
    EdsInt32 numberOfFreeClusters;
    EdsInt32 bytesPerSector;
    EdsBool reset;

} EdsCapacity;

/*-----------------------------------------------------------------------------
 FramePoint
 -----------------------------------------------------------------------------*/
typedef struct tagEdsFramePoint
{
    EdsInt32 x;
    EdsInt32 y;

} EdsFramePoint;

/*-----------------------------------------------------------------------------
 Point
-----------------------------------------------------------------------------*/
typedef struct tagEdsCameraPos
{
    EdsInt32 status;
    EdsInt32 position;
    EdsInt32 rolling;
    EdsInt32 pitching;

} EdsCameraPos;

/*-----------------------------------------------------------------------------
 FocusShiftSet
-----------------------------------------------------------------------------*/
typedef struct tagEdsFocusShiftSet
{
    EdsInt32 version;
    EdsInt32 focusShiftFunction;
    EdsInt32 shootingNumber;
    EdsInt32 stepWidth;
    EdsInt32 exposureSmoothing;
    EdsInt32 focusStackingFunction;
    EdsInt32 focusStackingTrimming;
    EdsInt32 flashInterval;
} EdsFocusShiftSet;

/*-----------------------------------------------------------------------------
 Manual WhiteBalance Data
-----------------------------------------------------------------------------*/
typedef struct tagEdsManualWBData
{
    EdsInt32 valid;
    EdsInt32 dataSize;
    EdsChar szCaption[32];
    EdsInt8 data[8];

} EdsManualWBData;

/*-----------------------------------------------------------------------------
 ApertureLockSetting
-----------------------------------------------------------------------------*/
typedef struct tagApertureLockSetting
{
    EdsUInt32 apertureLockStatus;
    EdsUInt32 avValue;
} ApertureLockSetting;

/******************************************************************************
 Callback Functions
******************************************************************************/
/*-----------------------------------------------------------------------------
 EdsProgressCallback
-----------------------------------------------------------------------------*/
typedef EdsError(EDSCALLBACK *EdsProgressCallback)(
    EdsUInt32 inPercent,
    EdsVoid *inContext,
    EdsBool *outCancel);

/*-----------------------------------------------------------------------------
 EdsCameraAddedHandler
-----------------------------------------------------------------------------*/
typedef EdsError(EDSCALLBACK *EdsCameraAddedHandler)(
    EdsVoid *inContext);

/*-----------------------------------------------------------------------------
 EdsPropertyEventHandler
-----------------------------------------------------------------------------*/
typedef EdsError(EDSCALLBACK *EdsPropertyEventHandler)(
    EdsPropertyEvent inEvent,
    EdsPropertyID inPropertyID,
    EdsUInt32 inParam,
    EdsVoid *inContext);

/*-----------------------------------------------------------------------------
 EdsObjectEventHandler
-----------------------------------------------------------------------------*/
typedef EdsError(EDSCALLBACK *EdsObjectEventHandler)(
    EdsObjectEvent inEvent,
    EdsBaseRef inRef,
    EdsVoid *inContext);

/*-----------------------------------------------------------------------------
 EdsStateEventHandler
-----------------------------------------------------------------------------*/
typedef EdsError(EDSCALLBACK *EdsStateEventHandler)(
    EdsStateEvent inEvent,
    EdsUInt32 inEventData,
    EdsVoid *inContext);

/*----------------------------------------------------------------------------*/
typedef EdsError EDSSTDCALL EdsReadStream(void *inContext, EdsUInt32 inReadSize, EdsVoid *outBuffer, EdsUInt32 *outReadSize);
typedef EdsError EDSSTDCALL EdsWriteStream(void *inContext, EdsUInt32 inWriteSize, const EdsVoid *inBuffer, EdsUInt32 *outWrittenSize);
typedef EdsError EDSSTDCALL EdsSeekStream(void *inContext, EdsInt32 inSeekOffset, EdsSeekOrigin inSeekOrigin);
typedef EdsError EDSSTDCALL EdsTellStream(void *inContext, EdsInt32 *outPosition);
typedef EdsError EDSSTDCALL EdsGetStreamLength(void *inContext, EdsUInt32 *outLength);

typedef struct
{
    void *context;

    EdsReadStream *read;
    EdsWriteStream *write;
    EdsSeekStream *seek;
    EdsTellStream *tell;
    EdsGetStreamLength *getLength;
} EdsIStream;

#ifdef __MACOS__
#if PRAGMA_STRUCT_ALIGN
#pragma options align = reset
#endif
#else
#pragma pack(pop)
#endif

#endif /* _EDS_TYPES_H_ */
