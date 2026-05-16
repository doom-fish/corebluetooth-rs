import CoreBluetooth
import Foundation

public typealias CBPeripheralEventCallback =
    @convention(c) (UnsafeMutableRawPointer?, UnsafePointer<CChar>?) -> Void

private final class CBRustPeripheralDelegate: NSObject, CBPeripheralDelegate {
    let callback: CBPeripheralEventCallback
    let userInfo: UnsafeMutableRawPointer?
    private var isActive = true

    init(callback: @escaping CBPeripheralEventCallback, userInfo: UnsafeMutableRawPointer?) {
        self.callback = callback
        self.userInfo = userInfo
        super.init()
    }

    func deactivate() {
        isActive = false
    }

    private func send(_ payload: [String: Any]) {
        guard isActive else { return }
        let json = cb_json_string(payload)
        json.withCString { callback(userInfo, $0) }
    }

    func peripheral(_ peripheral: CBPeripheral, didDiscoverServices error: Error?) {
        send([
            "event": "didDiscoverServices",
            "service_handles": (peripheral.services ?? []).map(cb_retained_handle),
            "error": cb_optional(error.map(cb_error_object)),
        ])
    }

    func peripheral(
        _ peripheral: CBPeripheral,
        didDiscoverCharacteristicsFor service: CBService,
        error: Error?
    ) {
        send([
            "event": "didDiscoverCharacteristicsForService",
            "service_handle": cb_retained_handle(service),
            "characteristic_handles": (service.characteristics ?? []).map(cb_retained_handle),
            "error": cb_optional(error.map(cb_error_object)),
        ])
    }

    func peripheral(
        _ peripheral: CBPeripheral,
        didUpdateValueFor characteristic: CBCharacteristic,
        error: Error?
    ) {
        send([
            "event": "didUpdateValueForCharacteristic",
            "characteristic_handle": cb_retained_handle(characteristic),
            "error": cb_optional(error.map(cb_error_object)),
        ])
    }

    func peripheral(
        _ peripheral: CBPeripheral,
        didWriteValueFor characteristic: CBCharacteristic,
        error: Error?
    ) {
        send([
            "event": "didWriteValueForCharacteristic",
            "characteristic_handle": cb_retained_handle(characteristic),
            "error": cb_optional(error.map(cb_error_object)),
        ])
    }

    func peripheral(
        _ peripheral: CBPeripheral,
        didUpdateNotificationStateFor characteristic: CBCharacteristic,
        error: Error?
    ) {
        send([
            "event": "didUpdateNotificationStateForCharacteristic",
            "characteristic_handle": cb_retained_handle(characteristic),
            "error": cb_optional(error.map(cb_error_object)),
        ])
    }

    func peripheral(_ peripheral: CBPeripheral, didReadRSSI RSSI: NSNumber, error: Error?) {
        send([
            "event": "didReadRSSI",
            "rssi": RSSI.intValue,
            "error": cb_optional(error.map(cb_error_object)),
        ])
    }
}

private let peripheralDelegateLock = NSLock()
private var peripheralDelegates: [ObjectIdentifier: CBRustPeripheralDelegate] = [:]

private func cb_peripheral(_ ptr: UnsafeMutableRawPointer?) -> CBPeripheral? {
    guard let ptr else {
        return nil
    }
    let peripheral: CBPeripheral = cb_borrow(ptr)
    return peripheral
}

private func cb_service(_ ptr: UnsafeMutableRawPointer?) -> CBService? {
    guard let ptr else {
        return nil
    }
    let service: CBService = cb_borrow(ptr)
    return service
}

private func cb_characteristic(_ ptr: UnsafeMutableRawPointer?) -> CBCharacteristic? {
    guard let ptr else {
        return nil
    }
    let characteristic: CBCharacteristic = cb_borrow(ptr)
    return characteristic
}

private func cb_descriptor(_ ptr: UnsafeMutableRawPointer?) -> CBDescriptor? {
    guard let ptr else {
        return nil
    }
    let descriptor: CBDescriptor = cb_borrow(ptr)
    return descriptor
}

private func cb_store_peripheral_delegate(
    _ delegate: CBRustPeripheralDelegate?,
    for peripheral: CBPeripheral
) {
    peripheralDelegateLock.lock()
    defer { peripheralDelegateLock.unlock() }

    let key = ObjectIdentifier(peripheral)
    if let delegate {
        peripheralDelegates[key] = delegate
        peripheral.delegate = delegate
    } else {
        peripheral.delegate = nil
        if let previous = peripheralDelegates.removeValue(forKey: key) {
            previous.deactivate()
        }
    }
}

@_cdecl("cb_peripheral_set_delegate")
public func cb_peripheral_set_delegate(
    _ peripheralPtr: UnsafeMutableRawPointer?,
    _ callback: CBPeripheralEventCallback?,
    _ userInfo: UnsafeMutableRawPointer?,
    _ errorOut: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    guard let peripheral = cb_peripheral(peripheralPtr) else {
        cb_write_error(errorOut, "peripheral must not be null")
        return CBR_INVALID_ARGUMENT
    }

    if let callback {
        cb_store_peripheral_delegate(CBRustPeripheralDelegate(callback: callback, userInfo: userInfo), for: peripheral)
    } else {
        cb_store_peripheral_delegate(nil, for: peripheral)
    }
    return CBR_OK
}

@_cdecl("cb_peripheral_clear_delegate")
public func cb_peripheral_clear_delegate(_ peripheralPtr: UnsafeMutableRawPointer?) {
    guard let peripheral = cb_peripheral(peripheralPtr) else {
        return
    }
    cb_store_peripheral_delegate(nil, for: peripheral)
}

@_cdecl("cb_peripheral_name")
public func cb_peripheral_name(_ peripheralPtr: UnsafeMutableRawPointer?) -> UnsafeMutablePointer<CChar>? {
    cb_peripheral(peripheralPtr).flatMap { cb_string($0.name ?? "") }
}

@_cdecl("cb_peripheral_identifier")
public func cb_peripheral_identifier(_ peripheralPtr: UnsafeMutableRawPointer?) -> UnsafeMutablePointer<CChar>? {
    cb_peripheral(peripheralPtr).flatMap { cb_string($0.identifier.uuidString) }
}

@_cdecl("cb_peripheral_state")
public func cb_peripheral_state(_ peripheralPtr: UnsafeMutableRawPointer?) -> Int32 {
    Int32(cb_peripheral(peripheralPtr)?.state.rawValue ?? CBPeripheralState.disconnected.rawValue)
}

@_cdecl("cb_peripheral_services")
public func cb_peripheral_services(
    _ peripheralPtr: UnsafeMutableRawPointer?,
    _ outArray: UnsafeMutablePointer<UnsafeMutableRawPointer?>,
    _ outCount: UnsafeMutablePointer<Int>
) {
    cb_make_pointer_array(cb_peripheral(peripheralPtr)?.services ?? [], outArray, outCount)
}

@_cdecl("cb_peripheral_discover_services")
public func cb_peripheral_discover_services(
    _ peripheralPtr: UnsafeMutableRawPointer?,
    _ serviceUUIDsJSON: UnsafePointer<CChar>?,
    _ errorOut: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    guard let peripheral = cb_peripheral(peripheralPtr) else {
        cb_write_error(errorOut, "peripheral must not be null")
        return CBR_INVALID_ARGUMENT
    }

    do {
        peripheral.discoverServices(try cb_service_uuids(serviceUUIDsJSON))
        return CBR_OK
    } catch {
        cb_write_error(errorOut, error.localizedDescription)
        return CBR_INVALID_ARGUMENT
    }
}

@_cdecl("cb_peripheral_read_rssi")
public func cb_peripheral_read_rssi(
    _ peripheralPtr: UnsafeMutableRawPointer?,
    _ errorOut: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    guard let peripheral = cb_peripheral(peripheralPtr) else {
        cb_write_error(errorOut, "peripheral must not be null")
        return CBR_INVALID_ARGUMENT
    }

    peripheral.readRSSI()
    return CBR_OK
}

@_cdecl("cb_peripheral_discover_characteristics")
public func cb_peripheral_discover_characteristics(
    _ peripheralPtr: UnsafeMutableRawPointer?,
    _ servicePtr: UnsafeMutableRawPointer?,
    _ characteristicUUIDsJSON: UnsafePointer<CChar>?,
    _ errorOut: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    guard let peripheral = cb_peripheral(peripheralPtr), let service = cb_service(servicePtr) else {
        cb_write_error(errorOut, "peripheral and service must not be null")
        return CBR_INVALID_ARGUMENT
    }

    do {
        peripheral.discoverCharacteristics(try cb_service_uuids(characteristicUUIDsJSON), for: service)
        return CBR_OK
    } catch {
        cb_write_error(errorOut, error.localizedDescription)
        return CBR_INVALID_ARGUMENT
    }
}

@_cdecl("cb_peripheral_read_value_for_characteristic")
public func cb_peripheral_read_value_for_characteristic(
    _ peripheralPtr: UnsafeMutableRawPointer?,
    _ characteristicPtr: UnsafeMutableRawPointer?,
    _ errorOut: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    guard let peripheral = cb_peripheral(peripheralPtr), let characteristic = cb_characteristic(characteristicPtr) else {
        cb_write_error(errorOut, "peripheral and characteristic must not be null")
        return CBR_INVALID_ARGUMENT
    }

    peripheral.readValue(for: characteristic)
    return CBR_OK
}

@_cdecl("cb_peripheral_write_value_for_characteristic")
public func cb_peripheral_write_value_for_characteristic(
    _ peripheralPtr: UnsafeMutableRawPointer?,
    _ characteristicPtr: UnsafeMutableRawPointer?,
    _ bytes: UnsafePointer<UInt8>?,
    _ length: Int,
    _ withResponse: Bool,
    _ errorOut: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    guard let peripheral = cb_peripheral(peripheralPtr), let characteristic = cb_characteristic(characteristicPtr), let bytes else {
        cb_write_error(errorOut, "peripheral, characteristic, and value bytes must not be null")
        return CBR_INVALID_ARGUMENT
    }

    let data = Data(bytes: bytes, count: length)
    peripheral.writeValue(data, for: characteristic, type: withResponse ? .withResponse : .withoutResponse)
    return CBR_OK
}

@_cdecl("cb_peripheral_set_notify_value")
public func cb_peripheral_set_notify_value(
    _ peripheralPtr: UnsafeMutableRawPointer?,
    _ characteristicPtr: UnsafeMutableRawPointer?,
    _ enabled: Bool,
    _ errorOut: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    guard let peripheral = cb_peripheral(peripheralPtr), let characteristic = cb_characteristic(characteristicPtr) else {
        cb_write_error(errorOut, "peripheral and characteristic must not be null")
        return CBR_INVALID_ARGUMENT
    }

    peripheral.setNotifyValue(enabled, for: characteristic)
    return CBR_OK
}

@_cdecl("cb_peripheral_discover_descriptors")
public func cb_peripheral_discover_descriptors(
    _ peripheralPtr: UnsafeMutableRawPointer?,
    _ characteristicPtr: UnsafeMutableRawPointer?,
    _ errorOut: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    guard let peripheral = cb_peripheral(peripheralPtr), let characteristic = cb_characteristic(characteristicPtr) else {
        cb_write_error(errorOut, "peripheral and characteristic must not be null")
        return CBR_INVALID_ARGUMENT
    }

    peripheral.discoverDescriptors(for: characteristic)
    return CBR_OK
}

@_cdecl("cb_service_uuid")
public func cb_service_uuid(_ servicePtr: UnsafeMutableRawPointer?) -> UnsafeMutablePointer<CChar>? {
    cb_service(servicePtr).flatMap { cb_string($0.uuid.uuidString) }
}

@_cdecl("cb_service_is_primary")
public func cb_service_is_primary(_ servicePtr: UnsafeMutableRawPointer?) -> Bool {
    cb_service(servicePtr)?.isPrimary ?? false
}

@_cdecl("cb_service_included_services")
public func cb_service_included_services(
    _ servicePtr: UnsafeMutableRawPointer?,
    _ outArray: UnsafeMutablePointer<UnsafeMutableRawPointer?>,
    _ outCount: UnsafeMutablePointer<Int>
) {
    cb_make_pointer_array(cb_service(servicePtr)?.includedServices ?? [], outArray, outCount)
}

@_cdecl("cb_service_characteristics")
public func cb_service_characteristics(
    _ servicePtr: UnsafeMutableRawPointer?,
    _ outArray: UnsafeMutablePointer<UnsafeMutableRawPointer?>,
    _ outCount: UnsafeMutablePointer<Int>
) {
    cb_make_pointer_array(cb_service(servicePtr)?.characteristics ?? [], outArray, outCount)
}

@_cdecl("cb_characteristic_uuid")
public func cb_characteristic_uuid(_ characteristicPtr: UnsafeMutableRawPointer?) -> UnsafeMutablePointer<CChar>? {
    cb_characteristic(characteristicPtr).flatMap { cb_string($0.uuid.uuidString) }
}

@_cdecl("cb_characteristic_properties")
public func cb_characteristic_properties(_ characteristicPtr: UnsafeMutableRawPointer?) -> UInt64 {
    UInt64(cb_characteristic(characteristicPtr)?.properties.rawValue ?? 0)
}

@_cdecl("cb_characteristic_value_json")
public func cb_characteristic_value_json(_ characteristicPtr: UnsafeMutableRawPointer?) -> UnsafeMutablePointer<CChar>? {
    guard let characteristic = cb_characteristic(characteristicPtr), let value = characteristic.value else {
        return nil
    }
    return cb_string(cb_json_string([UInt8](value)))
}

@_cdecl("cb_characteristic_is_notifying")
public func cb_characteristic_is_notifying(_ characteristicPtr: UnsafeMutableRawPointer?) -> Bool {
    cb_characteristic(characteristicPtr)?.isNotifying ?? false
}

@_cdecl("cb_characteristic_descriptors")
public func cb_characteristic_descriptors(
    _ characteristicPtr: UnsafeMutableRawPointer?,
    _ outArray: UnsafeMutablePointer<UnsafeMutableRawPointer?>,
    _ outCount: UnsafeMutablePointer<Int>
) {
    cb_make_pointer_array(cb_characteristic(characteristicPtr)?.descriptors ?? [], outArray, outCount)
}

@_cdecl("cb_descriptor_uuid")
public func cb_descriptor_uuid(_ descriptorPtr: UnsafeMutableRawPointer?) -> UnsafeMutablePointer<CChar>? {
    cb_descriptor(descriptorPtr).flatMap { cb_string($0.uuid.uuidString) }
}

@_cdecl("cb_descriptor_value_json")
public func cb_descriptor_value_json(_ descriptorPtr: UnsafeMutableRawPointer?) -> UnsafeMutablePointer<CChar>? {
    guard let descriptor = cb_descriptor(descriptorPtr), let value = descriptor.value else {
        return nil
    }
    return cb_string(cb_json_string(value))
}
