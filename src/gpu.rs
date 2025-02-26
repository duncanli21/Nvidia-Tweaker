use nvml_wrapper::Nvml;
use nvml_wrapper::enum_wrappers::device::{Clock, TemperatureSensor};

pub struct Gpu {
    pub nvml: Nvml,
    pub power_watts: String,
    pub core_freq: String,
    pub mem_freq: String,
    pub gpu_temp: String,
    //pub gpu_name: String,
    //pub gpu_driver_version: String,
    pub gpu_mem_free: String,
    pub gpu_mem_total: String,
    pub gpu_mem_used: String,
    pub fan_speed: String,
    pub gpu_utilization: u32,
    pub mem_utilization: u32,
}

impl Gpu {
    pub fn new() -> Self {
        Self {
            nvml: Nvml::init().expect("Failed to initialize NVML"),
            power_watts: 0.to_string(),
            core_freq: 0.to_string(),
            mem_freq: 0.to_string(),
            gpu_temp: 0.to_string(),
            //gpu_name: 0.to_string(),
            //gpu_driver_version: 0.to_string(),
            gpu_mem_free: 0.to_string(),
            gpu_mem_total: 0.to_string(),
            gpu_mem_used: 0.to_string(),
            fan_speed: 0.to_string(),
            gpu_utilization: 0,
            mem_utilization: 0,
        }
    }
    pub fn update_gpu_info(&mut self) {
        // create the device to read info from
        let nvml_device = self
            .nvml
            .device_by_index(0)
            .expect("Failed to get device by index 0");

        let power_mw = nvml_device
            .power_usage()
            .expect("Failed to get power usage");

        let power_watts = power_mw / 1000;

        self.power_watts = power_watts.to_string();

        self.core_freq = nvml_device
            .clock_info(Clock::Graphics)
            .expect("Failed to get core clock info")
            .to_string();

        self.mem_freq = nvml_device
            .clock_info(Clock::Memory)
            .expect("Failed to get mem clock info")
            .to_string();

        self.gpu_temp = nvml_device
            .temperature(TemperatureSensor::Gpu)
            .expect("Failed to get temp sensor info")
            .to_string();

        let mem_info: nvml_wrapper::struct_wrappers::device::MemoryInfo = nvml_device
            .memory_info()
            .expect("Failed to get memopry info");

        self.gpu_mem_free = (mem_info.free / 1024000).to_string();
        self.gpu_mem_used = (mem_info.used / 1024000).to_string();
        self.gpu_mem_total = (mem_info.total / 1024000).to_string();

        self.fan_speed = nvml_device
            .fan_speed(0)
            .expect("Unable to get speed for fan 0")
            .to_string();

        let utilization_rates: nvml_wrapper::struct_wrappers::device::Utilization =
            nvml_device.utilization_rates()
            .expect("Failed to get utilization rates");

        self.gpu_utilization = utilization_rates.gpu;
        self.mem_utilization = utilization_rates.memory;
    }

    //pub fn get_power_watts(&self) -> String {
    //    //let nvml_device = self
    //    //    .nvml
    //    //    .device_by_index(0)
    //    //    .expect("Failed to get device by index 0");
    //    //
    //    //let power_mw = nvml_device
    //    //    .power_usage()
    //    //    .expect("Failed to get power usage");
    //    //
    //    //let power_watts = power_mw / 1000;
    //    //
    //    //power_watts.to_string()
    //
    //    self.power_watts.clone()
    //}
    //
    //pub fn get_core_freq(&self) -> &str {
    //    //let nvml_device = self
    //    //    .nvml
    //    //    .device_by_index(0)
    //    //    .expect("Failed to get device by index 0");
    //    //
    //    //nvml_device
    //    //    .clock_info(Clock::Graphics)
    //    //    .expect("Failed to get core clock info")
    //    //    .to_string()
    //    //
    //    &self.core_freq
    //}
    //
    //pub fn get_mem_freq(&self) -> String {
    //    //let nvml_device = self
    //    //    .nvml
    //    //    .device_by_index(0)
    //    //    .expect("Failed to get device by index 0");
    //    //
    //    //nvml_device
    //    //    .clock_info(Clock::Memory)
    //    //    .expect("Failed to get mem clock info")
    //    //    .to_string()
    //
    //    self.mem_freq.clone()
    //}
    //
    //pub fn get_gpu_temp(&self) -> String {
    //    //let nvml_device = self
    //    //    .nvml
    //    //    .device_by_index(0)
    //    //    .expect("Failed to get device by index 0");
    //    //
    //    //nvml_device
    //    //    .temperature(TemperatureSensor::Gpu)
    //    //    .expect("Failed to get temp sensor info")
    //    //    .to_string()
    //    //
    //    self.gpu_temp.clone()
    //}

    pub fn get_gpu_name(&self) -> String {
        let nvml_device = self
            .nvml
            .device_by_index(0)
            .expect("Failed to get device by index 0");

        nvml_device.name().expect("Failed to get device name")
    }

    pub fn get_driver_version(&self) -> String {
        self.nvml
            .sys_driver_version()
            .expect("Could not get driver version")
    }

    //pub fn get_mem_free(&self) -> String {
    //    let nvml_device = self
    //        .nvml
    //        .device_by_index(0)
    //        .expect("Failed to get device by index 0");
    //
    //    let mem_info: nvml_wrapper::struct_wrappers::device::MemoryInfo = nvml_device
    //        .memory_info()
    //        .expect("Failed to get memopry info");
    //
    //    mem_info.free.to_string()
    //}
}
