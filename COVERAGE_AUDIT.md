# corebluetooth-rs coverage audit (vs MacOSX26.2.sdk)

SDK_PUBLIC_SYMBOLS: 75
VERIFIED: 64
GAPS: 0
EXEMPT: 11
COVERAGE_PCT: 100.0%

Audit scope: top-level public CoreBluetooth symbols (`@interface`, `@protocol`, `typedef`, exported constants) from the macOS 26.2 SDK, per the audit instructions. Member-level reachability was spot-checked against `src/**/*.rs`, `swift-bridge/Sources/**/*.swift`, and the crate's existing `COVERAGE.md`.

## 🟢 VERIFIED
| Symbol | Kind | Header | Wrapped by |
| --- | --- | --- | --- |
| `CBATTRequest` | interface | `CBATTRequest.h` | `AttRequest` |
| `CBAttribute` | interface | `CBAttribute.h` | `Service`, `Characteristic`, `Descriptor`, `MutableService`, `MutableCharacteristic`, `MutableDescriptor` |
| `CBCentral` | interface | `CBCentral.h` | `Central` |
| `CBCentralManager` | interface | `CBCentralManager.h` | `CentralManager` |
| `CBCharacteristic` | interface | `CBCharacteristic.h` | `Characteristic` |
| `CBMutableCharacteristic` | interface | `CBCharacteristic.h` | `MutableCharacteristic` |
| `CBDescriptor` | interface | `CBDescriptor.h` | `Descriptor` |
| `CBMutableDescriptor` | interface | `CBDescriptor.h` | `MutableDescriptor` |
| `CBL2CAPChannel` | interface | `CBL2CAPChannel.h` | `L2capChannel` |
| `CBManager` | interface | `CBManager.h` | `CentralManager`, `PeripheralManager` (shared manager base semantics) |
| `CBPeer` | interface | `CBPeer.h` | `Peer`, `Peripheral`, `Central` |
| `CBPeripheral` | interface | `CBPeripheral.h` | `Peripheral` |
| `CBPeripheralManager` | interface | `CBPeripheralManager.h` | `PeripheralManager` |
| `CBService` | interface | `CBService.h` | `Service` |
| `CBMutableService` | interface | `CBService.h` | `MutableService` |
| `CBUUID` | interface | `CBUUID.h` | `BluetoothUuid` |
| `CBCentralManagerDelegate` | protocol | `CBCentralManager.h` | `CentralManagerDelegate`, `CentralManagerCallbacks` |
| `CBPeripheralDelegate` | protocol | `CBPeripheral.h` | `PeripheralDelegate`, `PeripheralCallbacks` |
| `CBPeripheralManagerDelegate` | protocol | `CBPeripheralManager.h` | `PeripheralManagerDelegate`, `PeripheralManagerCallbacks` |
| `CBCharacteristicProperties` | enum | `CBCharacteristic.h` | `CharacteristicProperties` |
| `CBAttributePermissions` | enum | `CBCharacteristic.h` | `AttributePermissions` |
| `CBError` | enum | `CBError.h` | `BluetoothErrorCode` |
| `CBATTError` | enum | `CBError.h` | `AttError` |
| `CBL2CAPPSM` | typedef | `CBL2CAPChannel.h` | `u16` PSMs via `L2capChannel::psm()`, `Peripheral::open_l2cap_channel()`, `PeripheralManager::publish_l2cap_channel()` and `unpublish_l2cap_channel()` |
| `CBManagerState` | enum | `CBManager.h` | `CentralManagerState`, `PeripheralManagerState` |
| `CBManagerAuthorization` | enum | `CBManager.h` | `ManagerAuthorization` |
| `CBPeripheralState` | enum | `CBPeripheral.h` | `PeripheralState` |
| `CBCharacteristicWriteType` | enum | `CBPeripheral.h` | `CharacteristicWriteType` |
| `CBPeripheralManagerConnectionLatency` | enum | `CBPeripheralManager.h` | `PeripheralManagerConnectionLatency` |
| `CBAdvertisementDataLocalNameKey` | constant | `CBAdvertisementData.h` | `AdvertisementData::local_name()`, `AdvertisementData::with_local_name()` |
| `CBAdvertisementDataTxPowerLevelKey` | constant | `CBAdvertisementData.h` | `AdvertisementData::tx_power_level()` |
| `CBAdvertisementDataServiceUUIDsKey` | constant | `CBAdvertisementData.h` | `AdvertisementData::service_uuids()`, `AdvertisementData::with_service_uuid()` |
| `CBAdvertisementDataServiceDataKey` | constant | `CBAdvertisementData.h` | `AdvertisementData::service_data()` |
| `CBAdvertisementDataManufacturerDataKey` | constant | `CBAdvertisementData.h` | `AdvertisementData::manufacturer_data()` |
| `CBAdvertisementDataOverflowServiceUUIDsKey` | constant | `CBAdvertisementData.h` | `AdvertisementData::overflow_service_uuids()` |
| `CBAdvertisementDataIsConnectable` | constant | `CBAdvertisementData.h` | `AdvertisementData::is_connectable()` |
| `CBAdvertisementDataSolicitedServiceUUIDsKey` | constant | `CBAdvertisementData.h` | `AdvertisementData::solicited_service_uuids()` |
| `CBCentralManagerOptionShowPowerAlertKey` | constant | `CBCentralManagerConstants.h` | `CentralManagerOptions::with_show_power_alert()` |
| `CBCentralManagerOptionRestoreIdentifierKey` | constant | `CBCentralManagerConstants.h` | `CentralManagerOptions::with_restore_identifier()` |
| `CBCentralManagerScanOptionAllowDuplicatesKey` | constant | `CBCentralManagerConstants.h` | `ScanOptions::with_allow_duplicates()` |
| `CBCentralManagerScanOptionSolicitedServiceUUIDsKey` | constant | `CBCentralManagerConstants.h` | `ScanOptions::with_solicited_service_uuid()` |
| `CBConnectPeripheralOptionNotifyOnConnectionKey` | constant | `CBCentralManagerConstants.h` | `ConnectOptions::with_notify_on_connection()` |
| `CBConnectPeripheralOptionNotifyOnDisconnectionKey` | constant | `CBCentralManagerConstants.h` | `ConnectOptions::with_notify_on_disconnection()` |
| `CBConnectPeripheralOptionNotifyOnNotificationKey` | constant | `CBCentralManagerConstants.h` | `ConnectOptions::with_notify_on_notification()` |
| `CBConnectPeripheralOptionStartDelayKey` | constant | `CBCentralManagerConstants.h` | `ConnectOptions::with_start_delay_seconds()` |
| `CBCentralManagerRestoredStatePeripheralsKey` | constant | `CBCentralManagerConstants.h` | `CentralManagerRestoredState::peripherals` |
| `CBCentralManagerRestoredStateScanServicesKey` | constant | `CBCentralManagerConstants.h` | `CentralManagerRestoredState::scan_service_uuids` |
| `CBCentralManagerRestoredStateScanOptionsKey` | constant | `CBCentralManagerConstants.h` | `CentralManagerRestoredState::scan_options` |
| `CBConnectPeripheralOptionEnableAutoReconnect` | constant | `CBCentralManagerConstants.h` | `ConnectOptions::with_enable_auto_reconnect()` |
| `CBErrorDomain` | constant | `CBError.h` | `AttRequest::error_domain()` |
| `CBATTErrorDomain` | constant | `CBError.h` | `AttRequest::att_error_domain()` |
| `CBPeripheralManagerOptionShowPowerAlertKey` | constant | `CBPeripheralManagerConstants.h` | `PeripheralManagerOptions::with_show_power_alert()` |
| `CBPeripheralManagerOptionRestoreIdentifierKey` | constant | `CBPeripheralManagerConstants.h` | `PeripheralManagerOptions::with_restore_identifier()` |
| `CBPeripheralManagerRestoredStateServicesKey` | constant | `CBPeripheralManagerConstants.h` | `PeripheralManagerRestoredState::services` |
| `CBPeripheralManagerRestoredStateAdvertisementDataKey` | constant | `CBPeripheralManagerConstants.h` | `PeripheralManagerRestoredState::advertisement_data` |
| `CBUUIDCharacteristicExtendedPropertiesString` | constant | `CBUUID.h` | `BluetoothUuid::characteristic_extended_properties()` |
| `CBUUIDCharacteristicUserDescriptionString` | constant | `CBUUID.h` | `BluetoothUuid::characteristic_user_description()` |
| `CBUUIDClientCharacteristicConfigurationString` | constant | `CBUUID.h` | `BluetoothUuid::client_characteristic_configuration()` |
| `CBUUIDServerCharacteristicConfigurationString` | constant | `CBUUID.h` | `BluetoothUuid::server_characteristic_configuration()` |
| `CBUUIDCharacteristicFormatString` | constant | `CBUUID.h` | `BluetoothUuid::characteristic_format()` |
| `CBUUIDCharacteristicAggregateFormatString` | constant | `CBUUID.h` | `BluetoothUuid::characteristic_aggregate_format()` |
| `CBUUIDCharacteristicValidRangeString` | constant | `CBUUID.h` | `BluetoothUuid::characteristic_valid_range()` |
| `CBUUIDCharacteristicObservationScheduleString` | constant | `CBUUID.h` | `BluetoothUuid::characteristic_observation_schedule()` |
| `CBUUIDL2CAPPSMCharacteristicString` | constant | `CBUUID.h` | `BluetoothUuid::l2cap_psm_characteristic()` |

## 🔴 GAPS
| Symbol | Kind | Header | Notes |
| --- | --- | --- | --- |
| _None_ | — | — | All non-exempt top-level CoreBluetooth symbols have a public Rust representation in `corebluetooth-rs`. |

## ⏭️ EXEMPT
| Symbol | Kind | Header | Reason | SDK attribute |
| --- | --- | --- | --- | --- |
| `CBCentralManagerState` | enum | `CBCentralManager.h` | 10.x-deprecated alias of `CBManagerState`; excluded from scoring per audit instructions. | `NS_DEPRECATED(10_7, 10_13, 5_0, 10_0, "Use CBManagerState instead")` |
| `CBConnectionEvent` | enum | `CBCentralManager.h` | Connection-events API is unavailable on macOS. | `CB_CM_API_AVAILABLE` (= `API_UNAVAILABLE(macos)`) |
| `CBCentralManagerFeature` | enum | `CBCentralManager.h` | Feature flags are only consumed by the connection-events API, which is unavailable on macOS. | `CB_CM_API_AVAILABLE` on the feature cases / `supportsFeatures:` API |
| `CBConnectionEventMatchingOption` | typed enum | `CBCentralManagerConstants.h` | Typed option namespace is only used by `registerForConnectionEventsWithOptions:`, which is unavailable on macOS. | `CB_CM_API_AVAILABLE` companion APIs |
| `CBCentralManagerOptionDeviceAccessForMedia` | constant | `CBCentralManagerConstants.h` | iOS-only media-device-access option. | `NS_AVAILABLE_IOS(16_0)` |
| `CBConnectPeripheralOptionEnableTransportBridgingKey` | constant | `CBCentralManagerConstants.h` | iOS-only classic-transport bridging option. | `NS_AVAILABLE_IOS(13_0)` |
| `CBConnectPeripheralOptionRequiresANCS` | constant | `CBCentralManagerConstants.h` | iOS-only ANCS requirement option. | `NS_AVAILABLE_IOS(13_0)` |
| `CBConnectionEventMatchingOptionServiceUUIDs` | constant | `CBCentralManagerConstants.h` | Matching-option constant for the unavailable macOS connection-events API. | `CB_CM_API_AVAILABLE` (= `API_UNAVAILABLE(macos)`) |
| `CBConnectionEventMatchingOptionPeripheralUUIDs` | constant | `CBCentralManagerConstants.h` | Matching-option constant for the unavailable macOS connection-events API. | `CB_CM_API_AVAILABLE` (= `API_UNAVAILABLE(macos)`) |
| `CBPeripheralManagerAuthorizationStatus` | enum | `CBPeripheralManager.h` | Deprecated authorization enum superseded by `CBManagerAuthorization`. | `NS_DEPRECATED(10_9, 10_15, 7_0, 13_0, "Use CBManagerAuthorization instead")` |
| `CBPeripheralManagerState` | enum | `CBPeripheralManager.h` | 10.x-deprecated alias of `CBManagerState`; excluded from scoring per audit instructions. | `NS_DEPRECATED(10_9, 10_13, 6_0, 10_0, "Use CBManagerState instead")` |
