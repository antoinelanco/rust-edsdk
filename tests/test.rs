use edsdk::{
    EdsCameraListRef, EdsCameraRef, EdsDataType, EdsError, EdsInt8, EdsInt16, EdsInt32, EdsInt64,
    EdsPropertyID, EdsUInt8, EdsUInt16, EdsUInt32, EdsUInt64, EdsVoid, eds_close_session,
    eds_get_camera_list, eds_get_child_at_index, eds_get_property_data, eds_get_property_size,
    eds_initialize_sdk, eds_open_session, eds_release, eds_terminate_sdk,
};

#[test]
fn main() -> Result<(), EdsError> {
    eds_initialize_sdk()?;
    let list: EdsCameraListRef = eds_get_camera_list()?;
    let camera: EdsCameraRef = eds_get_child_at_index(list, 0)?;
    eds_open_session(camera)?;

    let in_param: EdsInt32 = 0;
    let in_property_id: EdsPropertyID = EdsPropertyID::BatteryLevel;
    let (_data_type, size): (EdsDataType, EdsUInt32) =
        eds_get_property_size(camera, in_property_id, in_param)?;
    let data: *mut EdsVoid = eds_get_property_data(camera, in_property_id, in_param, size)?;

    println!("{:?}", _data_type);
    match _data_type {
        EdsDataType::Int8 => println!("{:?}: {}", in_property_id, data as EdsInt8),
        EdsDataType::UInt8 => println!("{:?}: {}", in_property_id, data as EdsUInt8),
        EdsDataType::Int16 => println!("{:?}: {}", in_property_id, data as EdsInt16),
        EdsDataType::UInt16 => println!("{:?}: {}", in_property_id, data as EdsUInt16),
        EdsDataType::Int32 => println!("{:?}: {}", in_property_id, data as EdsInt32),
        EdsDataType::UInt32 => println!("{:?}: {}", in_property_id, data as EdsUInt32),
        EdsDataType::Int64 => println!("{:?}: {}", in_property_id, data as EdsInt64),
        EdsDataType::UInt64 => println!("{:?}: {}", in_property_id, data as EdsUInt64),
        _ => todo!(),
    }

    eds_close_session(camera)?;
    eds_release(camera)?;
    eds_release(list)?;
    eds_terminate_sdk()
}
