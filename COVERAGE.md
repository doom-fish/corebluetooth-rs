# CoreBluetooth.framework coverage audit (`corebluetooth-rs` v0.2.0)

Legend:

- ✅ implemented — exposed by the safe Rust surface in this crate
- 🟡 partial — represented, but intentionally shaped differently from the raw Apple signature
- ⏭️ skipped — iOS/watchOS-only, deprecated on macOS, or not meaningful for a safe macOS Rust API

## Base manager / attribute / peer surface

| Header | Apple API | Status | Rust surface | Notes |
| --- | --- | --- | --- | --- |
| `CBManager.h` | `CBManagerState` | ✅ implemented | `CentralManagerState`, `PeripheralManagerState` | Same raw values as `CBManagerState`. |
| `CBManager.h` | `CBManagerAuthorization` | ✅ implemented | `ManagerAuthorization` | Used by both manager roles. |
| `CBManager.h` | `state` | ✅ implemented | `CentralManager::state()`, `PeripheralManager::state()` | |
| `CBManager.h` | instance/class `authorization` | ✅ implemented | `CentralManager::authorization()`, `CentralManager::current_authorization()`, `PeripheralManager::authorization()`, `PeripheralManager::current_authorization()` | |
| `CBPeer.h` | `identifier` | ✅ implemented | `Peripheral::identifier()`, `Central::identifier()`, `Peer::identifier()` | |
| `CBAttribute.h` | `UUID` | ✅ implemented | `uuid()` / `uuid_object()` on `Service`, `Characteristic`, `Descriptor` | String + typed UUID accessors. |
| `CBCentral.h` | `maximumUpdateValueLength` | ✅ implemented | `Central::maximum_update_value_length()` | Covered under peripheral-manager subscriber handling. |

## CentralManager (`CBCentralManager.h`, `CBCentralManagerConstants.h`)

| Apple API | Status | Rust surface | Notes |
| --- | --- | --- | --- |
| `CBCentralManagerState` | ✅ implemented | `CentralManagerState` | Deprecated Apple alias retained as the public state enum. |
| `delegate` | ✅ implemented | `CentralManagerDelegate`, `CentralManagerCallbacks` | |
| `isScanning` | ✅ implemented | `CentralManager::is_scanning()` | |
| `initWithDelegate:queue:` / `initWithDelegate:queue:options:` | ✅ implemented | `CentralManager::new()`, `with_options()`, `with_delegate()`, `with_callbacks()`, `with_queue_label()` | Queue label + option builders map to the framework options dictionary. |
| `retrievePeripheralsWithIdentifiers:` | ✅ implemented | `CentralManager::retrieve_peripherals_with_identifiers()` | |
| `retrieveConnectedPeripheralsWithServices:` | ✅ implemented | `CentralManager::retrieve_connected_peripherals()` | |
| `scanForPeripheralsWithServices:options:` | ✅ implemented | `CentralManager::scan_for_peripherals()` | Supports allow-duplicates + solicited-service scan options. |
| `stopScan` | ✅ implemented | `CentralManager::stop_scan()` | |
| `connectPeripheral:options:` | ✅ implemented | `CentralManager::connect()`, `connect_with_options()` | Supports notify-on-connect/disconnect/notification, start delay, and auto reconnect. |
| `cancelPeripheralConnection:` | ✅ implemented | `CentralManager::cancel_peripheral_connection()` | |
| `centralManagerDidUpdateState:` | ✅ implemented | delegate callback | |
| `centralManager:willRestoreState:` | ✅ implemented | `CentralManagerRestoredState` + callback | Restored peripherals, scan services, and scan options are bridged. |
| `centralManager:didDiscoverPeripheral:advertisementData:RSSI:` | ✅ implemented | delegate callback | Raw advertisement payload is bridged as JSON and can be parsed with `AdvertisementData`. |
| `centralManager:didConnectPeripheral:` | ✅ implemented | delegate callback | |
| `centralManager:didFailToConnectPeripheral:error:` | ✅ implemented | delegate callback | |
| `centralManager:didDisconnectPeripheral:error:` | ✅ implemented | delegate callback | |
| `centralManager:didDisconnectPeripheral:timestamp:isReconnecting:error:` | ✅ implemented | `did_disconnect_peripheral_details` callback | Timestamp/reconnect metadata preserved. |
| `CBCentralManagerOptionShowPowerAlertKey` | ✅ implemented | `CentralManagerOptions::with_show_power_alert()` | |
| `CBCentralManagerOptionRestoreIdentifierKey` | ✅ implemented | `CentralManagerOptions::with_restore_identifier()` | |
| `CBCentralManagerScanOptionAllowDuplicatesKey` | ✅ implemented | `ScanOptions::with_allow_duplicates()` | |
| `CBCentralManagerScanOptionSolicitedServiceUUIDsKey` | ✅ implemented | `ScanOptions::with_solicited_service_uuid()` | |
| `CBConnectPeripheralOptionNotifyOnConnectionKey` | ✅ implemented | `ConnectOptions::with_notify_on_connection()` | |
| `CBConnectPeripheralOptionNotifyOnDisconnectionKey` | ✅ implemented | `ConnectOptions::with_notify_on_disconnection()` | |
| `CBConnectPeripheralOptionNotifyOnNotificationKey` | ✅ implemented | `ConnectOptions::with_notify_on_notification()` | |
| `CBConnectPeripheralOptionStartDelayKey` | ✅ implemented | `ConnectOptions::with_start_delay_seconds()` | |
| `CBConnectPeripheralOptionEnableAutoReconnect` | ✅ implemented | `ConnectOptions::with_enable_auto_reconnect()` | macOS 14+ runtime-gated in the Swift bridge. |
| `CBCentralManagerRestoredStatePeripheralsKey` | ✅ implemented | `CentralManagerRestoredState.peripherals` | |
| `CBCentralManagerRestoredStateScanServicesKey` | ✅ implemented | `CentralManagerRestoredState.scan_service_uuids` | |
| `CBCentralManagerRestoredStateScanOptionsKey` | ✅ implemented | `CentralManagerRestoredState.scan_options` | |
| `supportsFeatures:` / `CBCentralManagerFeature` | ⏭️ skipped | — | Declared `API_UNAVAILABLE(macos)`. |
| `registerForConnectionEventsWithOptions:` / matching-option constants / `connectionEventDidOccur` | ⏭️ skipped | — | Declared `API_UNAVAILABLE(macos)`. |
| `CBCentralManagerOptionDeviceAccessForMedia` | ⏭️ skipped | — | iOS-only. |
| `CBConnectPeripheralOptionEnableTransportBridgingKey` | ⏭️ skipped | — | iOS-only. |
| `CBConnectPeripheralOptionRequiresANCS` | ⏭️ skipped | — | iOS-only. |
| `centralManager:didUpdateANCSAuthorizationForPeripheral:` | ⏭️ skipped | — | iOS-only. |

## PeripheralManager (`CBPeripheralManager.h`, `CBPeripheralManagerConstants.h`)

| Apple API | Status | Rust surface | Notes |
| --- | --- | --- | --- |
| `CBPeripheralManagerState` | ✅ implemented | `PeripheralManagerState` | Same raw values as the framework state. |
| `CBPeripheralManagerConnectionLatency` | ✅ implemented | `PeripheralManagerConnectionLatency` | |
| `delegate` | ✅ implemented | `PeripheralManagerDelegate`, `PeripheralManagerCallbacks` | |
| `isAdvertising` | ✅ implemented | `PeripheralManager::is_advertising()` | |
| `initWithDelegate:queue:` / `initWithDelegate:queue:options:` | ✅ implemented | `PeripheralManager::new()`, `with_options()`, `with_delegate()`, `with_callbacks()`, `with_queue_label()` | |
| `startAdvertising:` | ✅ implemented | `PeripheralManager::start_advertising()` | Uses `AdvertisementData` local-name/service-UUID payload. |
| `stopAdvertising` | ✅ implemented | `PeripheralManager::stop_advertising()` | |
| `setDesiredConnectionLatency:forCentral:` | ✅ implemented | `PeripheralManager::set_desired_connection_latency()` | |
| `addService:` | ✅ implemented | `PeripheralManager::add_service()` | |
| `removeService:` | ✅ implemented | `PeripheralManager::remove_service()` | |
| `removeAllServices` | ✅ implemented | `PeripheralManager::remove_all_services()` | |
| `respondToRequest:withResult:` | ✅ implemented | `PeripheralManager::respond_to_request()` | Uses `AttError`. |
| `updateValue:forCharacteristic:onSubscribedCentrals:` | ✅ implemented | `PeripheralManager::update_value()` | Returns the framework boolean result. |
| `publishL2CAPChannelWithEncryption:` | ✅ implemented | `PeripheralManager::publish_l2cap_channel()` | macOS 10.14+ runtime-gated in the Swift bridge. |
| `unpublishL2CAPChannel:` | ✅ implemented | `PeripheralManager::unpublish_l2cap_channel()` | macOS 10.14+ runtime-gated in the Swift bridge. |
| `peripheralManagerDidUpdateState:` | ✅ implemented | delegate callback | |
| `peripheralManager:willRestoreState:` | ✅ implemented | `PeripheralManagerRestoredState` + callback | Restored services + advertisement payload bridged. |
| `peripheralManagerDidStartAdvertising:error:` | ✅ implemented | delegate callback | |
| `peripheralManager:didAddService:error:` | ✅ implemented | delegate callback | |
| `peripheralManager:central:didSubscribeToCharacteristic:` | ✅ implemented | delegate callback | Uses `Central` + `Characteristic`. |
| `peripheralManager:central:didUnsubscribeFromCharacteristic:` | ✅ implemented | delegate callback | |
| `peripheralManagerIsReadyToUpdateSubscribers:` | ✅ implemented | delegate callback | |
| `peripheralManager:didReceiveReadRequest:` | ✅ implemented | delegate callback | Uses `AttRequest`. |
| `peripheralManager:didReceiveWriteRequests:` | ✅ implemented | delegate callback | Uses `Vec<AttRequest>`. |
| `peripheralManager:didPublishL2CAPChannel:error:` | ✅ implemented | delegate callback | |
| `peripheralManager:didUnpublishL2CAPChannel:error:` | ✅ implemented | delegate callback | |
| `peripheralManager:didOpenL2CAPChannel:error:` | ✅ implemented | delegate callback | |
| `CBPeripheralManagerOptionShowPowerAlertKey` | ✅ implemented | `PeripheralManagerOptions::with_show_power_alert()` | |
| `CBPeripheralManagerOptionRestoreIdentifierKey` | ✅ implemented | `PeripheralManagerOptions::with_restore_identifier()` | |
| `CBPeripheralManagerRestoredStateServicesKey` | ✅ implemented | `PeripheralManagerRestoredState.services` | |
| `CBPeripheralManagerRestoredStateAdvertisementDataKey` | ✅ implemented | `PeripheralManagerRestoredState.advertisement_data` | |
| `CBPeripheralManagerAuthorizationStatus` / `authorizationStatus` | ⏭️ skipped | — | Deprecated on macOS in favor of `CBManagerAuthorization`. |

## Peripheral (`CBPeripheral.h`)

| Apple API | Status | Rust surface | Notes |
| --- | --- | --- | --- |
| `CBPeripheralState` | ✅ implemented | `PeripheralState` | |
| `CBCharacteristicWriteType` | ✅ implemented | `CharacteristicWriteType` | |
| `delegate` | ✅ implemented | `PeripheralDelegate`, `PeripheralCallbacks` | |
| `name` | ✅ implemented | `Peripheral::name()` | |
| `state` | ✅ implemented | `Peripheral::state()` | |
| `services` | ✅ implemented | `Peripheral::services()` | |
| `canSendWriteWithoutResponse` | ✅ implemented | `Peripheral::can_send_write_without_response()` | |
| `readRSSI` | ✅ implemented | `Peripheral::read_rssi()` | |
| `discoverServices:` | ✅ implemented | `Peripheral::discover_services()` | |
| `discoverIncludedServices:forService:` | ✅ implemented | `Peripheral::discover_included_services()` | |
| `discoverCharacteristics:forService:` | ✅ implemented | `Peripheral::discover_characteristics()` | |
| `readValueForCharacteristic:` | ✅ implemented | `Peripheral::read_value_for_characteristic()` | |
| `maximumWriteValueLengthForType:` | ✅ implemented | `Peripheral::maximum_write_value_length()` | |
| `writeValue:forCharacteristic:type:` | ✅ implemented | `Peripheral::write_value_for_characteristic()` | |
| `setNotifyValue:forCharacteristic:` | ✅ implemented | `Peripheral::set_notify_value()` | |
| `discoverDescriptorsForCharacteristic:` | ✅ implemented | `Peripheral::discover_descriptors()` | |
| `readValueForDescriptor:` | ✅ implemented | `Peripheral::read_value_for_descriptor()` | |
| `writeValue:forDescriptor:` | ✅ implemented | `Peripheral::write_value_for_descriptor()` | |
| `openL2CAPChannel:` | ✅ implemented | `Peripheral::open_l2cap_channel()` | macOS 10.14+ runtime-gated in the Swift bridge. |
| `peripheralDidUpdateName:` | ✅ implemented | delegate callback | |
| `peripheral:didModifyServices:` | ✅ implemented | delegate callback | |
| `peripheral:didReadRSSI:error:` | ✅ implemented | delegate callback | |
| `peripheral:didDiscoverServices:` | ✅ implemented | delegate callback | |
| `peripheral:didDiscoverIncludedServicesForService:error:` | ✅ implemented | delegate callback | |
| `peripheral:didDiscoverCharacteristicsForService:error:` | ✅ implemented | delegate callback | |
| `peripheral:didUpdateValueForCharacteristic:error:` | ✅ implemented | delegate callback | |
| `peripheral:didWriteValueForCharacteristic:error:` | ✅ implemented | delegate callback | |
| `peripheral:didUpdateNotificationStateForCharacteristic:error:` | ✅ implemented | delegate callback | |
| `peripheral:didDiscoverDescriptorsForCharacteristic:error:` | ✅ implemented | delegate callback | |
| `peripheral:didUpdateValueForDescriptor:error:` | ✅ implemented | delegate callback | |
| `peripheral:didWriteValueForDescriptor:error:` | ✅ implemented | delegate callback | |
| `peripheralIsReadyToSendWriteWithoutResponse:` | ✅ implemented | delegate callback | |
| `peripheral:didOpenL2CAPChannel:error:` | ✅ implemented | delegate callback | |
| `RSSI` / `peripheralDidUpdateRSSI:error:` | ⏭️ skipped | — | Deprecated on macOS; `readRSSI`/`didReadRSSI` is exposed instead. |
| `ancsAuthorized` | ⏭️ skipped | — | iOS-only. |

## Service / MutableService (`CBService.h`)

| Apple API | Status | Rust surface | Notes |
| --- | --- | --- | --- |
| `CBService.peripheral` | ✅ implemented | `Service::peripheral()` | |
| `CBService.isPrimary` | ✅ implemented | `Service::is_primary()` | |
| `CBService.includedServices` | ✅ implemented | `Service::included_services()` | |
| `CBService.characteristics` | ✅ implemented | `Service::characteristics()` | |
| `CBMutableService.initWithType:primary:` | ✅ implemented | `MutableService::new()` | |
| `CBMutableService.includedServices` (read/write) | ✅ implemented | `included_services()`, `set_included_services()`, `set_included_service_views()` | Supports mutable and immutable service views. |
| `CBMutableService.characteristics` (read/write) | ✅ implemented | `characteristics()`, `set_characteristics()`, `set_characteristic_views()` | Supports mutable and immutable characteristic views. |

## Characteristic / MutableCharacteristic (`CBCharacteristic.h`)

| Apple API | Status | Rust surface | Notes |
| --- | --- | --- | --- |
| `CBCharacteristicProperties` | ✅ implemented | `CharacteristicProperties` | |
| `CBAttributePermissions` | ✅ implemented | `AttributePermissions` | |
| `CBCharacteristic.service` | ✅ implemented | `Characteristic::service()` | |
| `CBCharacteristic.properties` | ✅ implemented | `Characteristic::properties()` | |
| `CBCharacteristic.value` | ✅ implemented | `Characteristic::value()` | |
| `CBCharacteristic.descriptors` | ✅ implemented | `Characteristic::descriptors()` | |
| `CBCharacteristic.isNotifying` | ✅ implemented | `Characteristic::is_notifying()` | |
| `CBMutableCharacteristic.initWithType:properties:value:permissions:` | ✅ implemented | `MutableCharacteristic::new()` | |
| `CBMutableCharacteristic.permissions` | ✅ implemented | `permissions()`, `set_permissions()` | |
| `CBMutableCharacteristic.subscribedCentrals` | ✅ implemented | `subscribed_centrals()` | |
| `CBMutableCharacteristic.properties` (read/write) | ✅ implemented | `properties()`, `set_properties()` | |
| `CBMutableCharacteristic.value` (read/write) | ✅ implemented | `value()`, `set_value()` | |
| `CBMutableCharacteristic.descriptors` (read/write) | ✅ implemented | `descriptors()`, `set_descriptors()`, `set_descriptor_views()` | Supports mutable and immutable descriptor views. |
| `CBCharacteristic.isBroadcasted` | ⏭️ skipped | — | Deprecated on macOS. |

## Descriptor / MutableDescriptor (`CBDescriptor.h`)

| Apple API | Status | Rust surface | Notes |
| --- | --- | --- | --- |
| `CBDescriptor.characteristic` | ✅ implemented | `Descriptor::characteristic()` | |
| `CBDescriptor.value` | ✅ implemented | `Descriptor::value()` | Bridged as `serde_json::Value`. |
| `CBMutableDescriptor.initWithType:value:` | ✅ implemented | `MutableDescriptor::new()` | Supports string, byte, integer, boolean, and null payloads via `DescriptorValue`. |

## ATT / Errors (`CBATTRequest.h`, `CBError.h`)

| Apple API | Status | Rust surface | Notes |
| --- | --- | --- | --- |
| `CBATTRequest.central` | ✅ implemented | `AttRequest::central()` | |
| `CBATTRequest.characteristic` | ✅ implemented | `AttRequest::characteristic()` | |
| `CBATTRequest.offset` | ✅ implemented | `AttRequest::offset()` | |
| `CBATTRequest.value` (read/write) | ✅ implemented | `AttRequest::value()`, `set_value()` | |
| `CBATTError` / `CBATTErrorDomain` | ✅ implemented | `AttError`, `AttRequest::att_error_domain()` | |
| `CBError` / `CBErrorDomain` | ✅ implemented | `BluetoothErrorCode`, `AttRequest::error_domain()` | |
| `CBErrorUnkownDevice` deprecated alias | ⏭️ skipped | — | Deprecated spelling alias on macOS; `BluetoothErrorCode::UnknownDevice` is exposed instead. |
| watchOS-only `CBErrorLeGatt*` values | ⏭️ skipped | — | Declared unavailable on macOS. |

## L2CAPChannel (`CBL2CAPChannel.h`)

| Apple API | Status | Rust surface | Notes |
| --- | --- | --- | --- |
| `CBL2CAPPSM` | ✅ implemented | `u16` PSM values in `L2capChannel` / manager methods | |
| `CBL2CAPChannel.peer` | ✅ implemented | `L2capChannel::peer()` | Uses generic `Peer` wrapper with `identifier()`. |
| `CBL2CAPChannel.inputStream` | ✅ implemented | `L2capChannel::input_stream()` | Returned as `InputStreamHandle`. |
| `CBL2CAPChannel.outputStream` | ✅ implemented | `L2capChannel::output_stream()` | Returned as `OutputStreamHandle`. |
| `CBL2CAPChannel.PSM` | ✅ implemented | `L2capChannel::psm()` | |

## Advertisement (`CBAdvertisementData.h`)

| Apple API | Status | Rust surface | Notes |
| --- | --- | --- | --- |
| `CBAdvertisementDataLocalNameKey` | ✅ implemented | `AdvertisementData::local_name()` / advertising builder | |
| `CBAdvertisementDataTxPowerLevelKey` | ✅ implemented | `AdvertisementData::tx_power_level()` | |
| `CBAdvertisementDataServiceUUIDsKey` | ✅ implemented | `AdvertisementData::service_uuids()` / advertising builder | |
| `CBAdvertisementDataServiceDataKey` | ✅ implemented | `AdvertisementData::service_data()` | |
| `CBAdvertisementDataManufacturerDataKey` | ✅ implemented | `AdvertisementData::manufacturer_data()` | |
| `CBAdvertisementDataOverflowServiceUUIDsKey` | ✅ implemented | `AdvertisementData::overflow_service_uuids()` | |
| `CBAdvertisementDataIsConnectable` | ✅ implemented | `AdvertisementData::is_connectable()` | |
| `CBAdvertisementDataSolicitedServiceUUIDsKey` | ✅ implemented | `AdvertisementData::solicited_service_uuids()` | |

## UUID (`CBUUID.h`)

| Apple API | Status | Rust surface | Notes |
| --- | --- | --- | --- |
| `data` | ✅ implemented | `BluetoothUuid::data()` | |
| `UUIDString` | ✅ implemented | `BluetoothUuid::uuid_string()` / `Display` | |
| `UUIDWithString:` | ✅ implemented | `BluetoothUuid::from_string()` | |
| `UUIDWithData:` | ✅ implemented | `BluetoothUuid::from_bytes()`, `from_data()` | |
| `UUIDWithNSUUID:` | ✅ implemented | `BluetoothUuid::from_uuid_string()` | Pass the UUID string form from Rust. |
| `CBUUIDCharacteristicExtendedPropertiesString` | ✅ implemented | `BluetoothUuid::characteristic_extended_properties()` | |
| `CBUUIDCharacteristicUserDescriptionString` | ✅ implemented | `BluetoothUuid::characteristic_user_description()` | |
| `CBUUIDClientCharacteristicConfigurationString` | ✅ implemented | `BluetoothUuid::client_characteristic_configuration()` | |
| `CBUUIDServerCharacteristicConfigurationString` | ✅ implemented | `BluetoothUuid::server_characteristic_configuration()` | |
| `CBUUIDCharacteristicFormatString` | ✅ implemented | `BluetoothUuid::characteristic_format()` | |
| `CBUUIDCharacteristicAggregateFormatString` | ✅ implemented | `BluetoothUuid::characteristic_aggregate_format()` | |
| `CBUUIDCharacteristicValidRangeString` | ✅ implemented | `BluetoothUuid::characteristic_valid_range()` | |
| `CBUUIDCharacteristicObservationScheduleString` | ✅ implemented | `BluetoothUuid::characteristic_observation_schedule()` | |
| `CBUUIDL2CAPPSMCharacteristicString` | ✅ implemented | `BluetoothUuid::l2cap_psm_characteristic()` | |
| `UUIDWithCFUUID:` | ⏭️ skipped | — | Deprecated on macOS; safe Rust surface uses string/data constructors instead. |
