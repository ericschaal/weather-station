use crate::bme68x::interface::Bme68xInterface;
use esp_idf_hal::delay::Delay;
use std::ptr::NonNull;

pub unsafe extern "C" fn read_wrapper(
    reg_addr: u8,
    reg_data: *mut u8,
    length: u32,
    intf_ptr: *mut ::std::os::raw::c_void,
) -> i8 {
    let instance = NonNull::new(intf_ptr as *mut Bme68xInterface);
    match instance {
        Some(mut instance) => instance.as_mut().read_raw(reg_addr, reg_data, length),
        None => -1,
    }
}

pub unsafe extern "C" fn write_wrapper(
    reg_addr: u8,
    reg_data: *const u8,
    length: u32,
    intf_ptr: *mut ::std::os::raw::c_void,
) -> i8 {
    let instance = NonNull::new(intf_ptr as *mut Bme68xInterface);
    match instance {
        Some(mut instance) => instance.as_mut().write_raw(reg_addr, reg_data, length),
        None => -1,
    }
}

pub unsafe extern "C" fn delay_wrapper(period: u32, _: *mut ::std::os::raw::c_void) {
    Delay::delay_us(period);
}
