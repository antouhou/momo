use super::{
    BluetoothDeviceSection, build_bluetooth_state, has_presentable_device_name, is_device_connected,
};
use system_control::{
    BluetoothAdapterState, BluetoothCapabilities, BluetoothConnectionState, BluetoothDevice,
    BluetoothDeviceCategory, BluetoothDeviceId, BluetoothDiscoveryState, BluetoothFeatureState,
    BluetoothOperationId, BluetoothPowerState, FeatureState,
};

fn bluetooth_device_with_connection_state(
    connection_state: BluetoothConnectionState,
) -> BluetoothDevice {
    BluetoothDevice {
        device_identifier: BluetoothDeviceId("device-1".to_string()),
        display_name: "Test device".to_string(),
        category: BluetoothDeviceCategory::Unknown,
        is_paired: false,
        is_trusted: false,
        connection_state,
        signal_strength_dbm: Some(-48),
        battery_percentage: None,
    }
}

#[test]
fn bluetooth_name_filter_keeps_regular_names() {
    assert!(has_presentable_device_name("Sony WH-1000XM5"));
    assert!(has_presentable_device_name("Magic Keyboard"));
}

#[test]
fn bluetooth_name_filter_drops_address_like_names() {
    assert!(!has_presentable_device_name("0D-48-AC-11-22-33"));
    assert!(!has_presentable_device_name("0d:48:ac:11:22:33"));
    assert!(!has_presentable_device_name("  0D-48-AC-11-22-33  "));
}

#[test]
fn bluetooth_connection_grouping_does_not_treat_connecting_as_connected() {
    let device = bluetooth_device_with_connection_state(BluetoothConnectionState::Connecting {
        operation_id: BluetoothOperationId(7),
    });

    assert!(!is_device_connected(&device));
}

#[test]
fn bluetooth_connection_grouping_keeps_disconnecting_as_connected() {
    let device = bluetooth_device_with_connection_state(BluetoothConnectionState::Disconnecting {
        operation_id: BluetoothOperationId(7),
    });

    assert!(is_device_connected(&device));
}

fn ready_bluetooth_feature_state(devices: Vec<BluetoothDevice>) -> BluetoothFeatureState {
    FeatureState::Ready(system_control::BluetoothState {
        adapter: BluetoothAdapterState {
            adapter_identifier: "adapter-1".to_string(),
            adapter_name: Some("Adapter".to_string()),
            power_state: BluetoothPowerState::On,
            discovery_state: BluetoothDiscoveryState::Scanning,
            capabilities: BluetoothCapabilities {
                can_change_power: true,
                can_start_discovery: true,
                can_connect_devices: true,
            },
        },
        devices,
        pending_operations: Vec::new(),
        last_error: None,
        revision: 1,
    })
}

#[test]
fn bluetooth_recent_devices_include_paired_disconnected_devices() {
    let mut disconnected_device =
        bluetooth_device_with_connection_state(BluetoothConnectionState::Disconnected);
    disconnected_device.is_paired = true;

    let next_state = build_bluetooth_state(ready_bluetooth_feature_state(vec![
        disconnected_device.clone(),
    ]));

    let BluetoothDeviceSection::Ready(recent_devices) = next_state.recent_devices else {
        panic!("expected ready recent devices section");
    };
    let BluetoothDeviceSection::Ready(nearby_devices) = next_state.nearby_devices else {
        panic!("expected ready nearby devices section");
    };

    assert_eq!(recent_devices.len(), 1);
    assert_eq!(
        recent_devices[0].device_identifier,
        disconnected_device.device_identifier
    );
    assert!(nearby_devices.is_empty());
}
