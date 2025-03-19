use std::{mem::MaybeUninit, ptr};

use ndk_sys::{
    ALooper_pollAll, ALooper_prepare, ASensor, ASensorEvent, ASensorEventQueue,
    ASensorEventQueue_disableSensor, ASensorEventQueue_enableSensor, ASensorEventQueue_getEvents,
    ASensorManager, ASensorManager_createEventQueue, ASensorManager_destroyEventQueue,
    ASensorManager_getDefaultSensor, ASensorManager_getInstance,
    ALOOPER_PREPARE_ALLOW_NON_CALLBACKS, ASENSOR_TYPE_GRAVITY,
};

// pub struct AndroidSensor {
//     sensor_manager: *mut ASensorManager,
//     sensor: *const ASensor,
//     event_queue: *mut ASensorEventQueue,
//     pub event: *mut ASensorEvent,
// }

// impl Default for AndroidSensor {
//     fn default() -> Self {
//         unsafe {
//             let sensor_manager = ASensorManager_getInstance();
//             let sensor = ASensorManager_getDefaultSensor(sensor_manager, ASENSOR_TYPE_GRAVITY);
//             let event_queue = ASensorManager_createEventQueue(
//                 sensor_manager,
//                 ALooper_prepare(ALOOPER_PREPARE_ALLOW_NON_CALLBACKS as i32),
//                 0,
//                 None,
//                 ptr::null_mut(),
//             );
//             let event = MaybeUninit::<ASensorEvent>::uninit().as_mut_ptr();

//             ASensorEventQueue_enableSensor(event_queue, sensor);

//             Self {
//                 sensor_manager,
//                 sensor,
//                 event_queue,
//                 event,
//             }
//         }
//     }
// }

// impl Drop for AndroidSensor {
//     fn drop(&mut self) {
//         unsafe {
//             ASensorEventQueue_disableSensor(self.event_queue, self.sensor);
//             ASensorManager_destroyEventQueue(self.sensor_manager, self.event_queue);
//         }
//     }
// }

// impl AndroidSensor {
//     pub fn update(&mut self) {
//         unsafe {
//             ALooper_pollAll(-1, ptr::null_mut(), ptr::null_mut(), ptr::null_mut());
//             ASensorEventQueue_getEvents(self.event_queue, self.event, 1);
//         }
//     }
// }

pub fn get_sensor_event() -> ASensorEvent {
    unsafe {
        let sensor_manager = ASensorManager_getInstance();
        let sensor = ASensorManager_getDefaultSensor(sensor_manager, ASENSOR_TYPE_GRAVITY);
        let event_queue = ASensorManager_createEventQueue(
            sensor_manager,
            ALooper_prepare(ALOOPER_PREPARE_ALLOW_NON_CALLBACKS as i32),
            0,
            None,
            ptr::null_mut(),
        );

        ASensorEventQueue_enableSensor(event_queue, sensor);
        ALooper_pollAll(-1, ptr::null_mut(), ptr::null_mut(), ptr::null_mut());

        let mut event = MaybeUninit::<ASensorEvent>::uninit();
        ASensorEventQueue_getEvents(event_queue, event.as_mut_ptr(), 1);
        let event = event.assume_init();

        ASensorEventQueue_disableSensor(event_queue, sensor);
        ASensorManager_destroyEventQueue(sensor_manager, event_queue);

        event
    }
}
