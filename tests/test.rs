use edsdk::*;

#[test]
fn main() -> Result<(), EdsError> {
    eds_initialize_sdk()?;
    let list = eds_get_camera_list()?;
    let count = eds_get_child_count(list)?;
    assert!(count.gt(&0), "Camera not found");
    let camera = eds_get_child_at_index(list, 0)?;
    eds_open_session(camera)?;
    let in_param = 0;
    let in_property_id = EdsPropertyID::BatteryLevel;
    let (_data_type, size) = eds_get_property_size(camera, in_property_id, in_param)?;
    let data = eds_get_property_data(camera, in_property_id, in_param, size)?;

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

#[test]
fn min() -> Result<(), EdsError> {
    eds_initialize_sdk()?;
    let list = eds_get_camera_list()?;
    let camera = eds_get_child_at_index(list, 0)?;
    eds_open_session(camera)?;
    let _data = eds_get_property_data(camera, EdsPropertyID::BatteryLevel, 0, 4)?;
    println!("{:?}", _data);

    eds_close_session(camera)?;
    eds_release(camera)?;
    eds_release(list)?;
    eds_terminate_sdk()
}
