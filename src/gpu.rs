use nvml_wrapper::Nvml;
use nvml_wrapper::enum_wrappers::device::{Clock, TemperatureSensor};

// define an array of available clocks to iterate through 
const CLOCKS_ARRAY: [Clock; 4] = [Clock::Graphics, Clock::SM, Clock::Memory, Clock::Video];

pub struct Gpu {
    pub nvml: Nvml,
    pub power_watts: String,
    pub gpu_temp: String,
    pub gpu_mem_free: String,
    pub gpu_mem_total: String,
    pub gpu_mem_used: String,
    pub fan_speed: String,
    pub gpu_utilization: u32,
    pub mem_utilization: u32,
    pub clock_speed_array: [u32; 4],
    pub clock_speed_max_array: [u32; 4],
}

impl Gpu {
    pub fn new() -> Self {
        Self {
            nvml: Nvml::init().expect("Failed to initialize NVML"),
            power_watts: 0.to_string(),
            gpu_temp: 0.to_string(),
            gpu_mem_free: 0.to_string(),
            gpu_mem_total: 0.to_string(),
            gpu_mem_used: 0.to_string(),
            fan_speed: 0.to_string(),
            gpu_utilization: 0,
            mem_utilization: 0,
            clock_speed_array: [0; 4],
            clock_speed_max_array: [0; 4],
        }
    }

    // function to update the gpu information.
    // this fn will be run on a subscription by the main iced runtime
    pub fn update_gpu_info(&mut self) {
        // create the device to read info from
        let nvml_device = self
            .nvml
            .device_by_index(0)
            .expect("Failed to get device by index 0");

        // get the power from the device in milliwatts
        let power_mw = nvml_device
            .power_usage()
            .expect("Failed to get power usage");

        // divide it out to make Watts
        let power_watts = power_mw / 1000;

        // convert to a string
        self.power_watts = power_watts.to_string();

        // loop through the clocks
        for (index, clock) in CLOCKS_ARRAY.iter().enumerate() {
            // get the current clock speeds
            self.clock_speed_array[index] = nvml_device
                .clock_info(clock.clone())
                .expect("Failed to get clock speed");

            // get the max clock speeds
            self.clock_speed_max_array[index] = nvml_device
                .max_clock_info(clock.clone())
                .expect("Failed to get max clock speed");
        }

        // get the gpu temp
        self.gpu_temp = nvml_device
            .temperature(TemperatureSensor::Gpu)
            .expect("Failed to get temp sensor info")
            .to_string();

        // get the memory info struct from the device
        let mem_info: nvml_wrapper::struct_wrappers::device::MemoryInfo = nvml_device
            .memory_info()
            .expect("Failed to get memopry info");

        // convert memory info to scaled strings 
        self.gpu_mem_free = (mem_info.free / 1024000).to_string();
        self.gpu_mem_used = (mem_info.used / 1024000).to_string();
        self.gpu_mem_total = (mem_info.total / 1024000).to_string();

        // read in the fan speed for fan 0
        // Probably need to check which fans are available and see if there
        // needs to be an array of fans
        self.fan_speed = nvml_device
            .fan_speed(0)
            .expect("Unable to get speed for fan 0")
            .to_string();

        // get the utilization rates
        let utilization_rates: nvml_wrapper::struct_wrappers::device::Utilization = nvml_device
            .utilization_rates()
            .expect("Failed to get utilization rates");

        // split it out into variables
        self.gpu_utilization = utilization_rates.gpu;
        self.mem_utilization = utilization_rates.memory;

    }
   
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
}
