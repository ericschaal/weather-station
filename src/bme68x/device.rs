use crate::bme68x::driver::{bme68x_calib_data, bme68x_dev, bme68x_intf_BME68X_I2C_INTF};
use crate::bme68x::wrapper::{delay_wrapper, read_wrapper, write_wrapper};
use std::ptr;

impl Default for bme68x_dev {
    fn default() -> Self {
        Self {
            chip_id: 0,
            variant_id: 0,
            intf: bme68x_intf_BME68X_I2C_INTF,
            mem_page: 0,
            amb_temp: 0,
            calib: bme68x_calib_data {
                par_h1: 0,
                par_h2: 0,
                par_h3: 0,
                par_h4: 0,
                par_h5: 0,
                par_h6: 0,
                par_h7: 0,
                par_gh1: 0,
                par_gh2: 0,
                par_gh3: 0,
                par_t1: 0,
                par_t2: 0,
                par_t3: 0,
                par_p1: 0,
                par_p2: 0,
                par_p3: 0,
                par_p4: 0,
                par_p5: 0,
                par_p6: 0,
                par_p7: 0,
                par_p8: 0,
                par_p9: 0,
                par_p10: 0,
                t_fine: 0.0,
                res_heat_range: 0,
                res_heat_val: 0,
                range_sw_err: 0,
            },
            read: Some(read_wrapper),
            write: Some(write_wrapper),
            delay_us: Some(delay_wrapper),
            intf_rslt: 0,
            intf_ptr: ptr::null_mut(),
            info_msg: 0,
        }
    }
}
