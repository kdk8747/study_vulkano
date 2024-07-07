
use vulkano::instance::{Instance, InstanceCreateInfo};
use vulkano::{Version, VulkanLibrary};


fn main() {
    let instance = {
        let library = VulkanLibrary::new().expect("no local Vulkan library/DLL");
        //let extensions = vulkano_win::required_extensions(&library);

        Instance::new(
            library,
            InstanceCreateInfo {
                //enabled_extensions: extensions,
                max_api_version: Some(Version::V1_2),
                ..Default::default()
            },
        )
        .expect("failed to create instance")
    };

    let physical_device = instance
        .enumerate_physical_devices()
        .expect("could not enumerate devices")
        .next()
        .expect("no devices available");

    println!(
        "Using device: {} (type: {:?})",
        physical_device.properties().device_name,
        physical_device.properties().device_type
    );
    println!(
        "Our physical device supports Vulkan: {:?}",
        physical_device.properties().api_version
    );
}