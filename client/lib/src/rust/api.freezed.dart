// coverage:ignore-file
// GENERATED CODE - DO NOT MODIFY BY HAND
// ignore_for_file: type=lint
// ignore_for_file: unused_element, deprecated_member_use, deprecated_member_use_from_same_package, use_function_type_syntax_for_parameters, unnecessary_const, avoid_init_to_null, invalid_override_different_default_values_named, prefer_expression_function_bodies, annotate_overrides, invalid_annotation_target, unnecessary_question_mark

part of 'api.dart';

// **************************************************************************
// FreezedGenerator
// **************************************************************************

T _$identity<T>(T value) => value;

final _privateConstructorUsedError = UnsupportedError(
  'It seems like you constructed your class using `MyClass._()`. This constructor is only meant to be used by freezed and you are not supposed to need it nor use it.\nPlease check the documentation here for more information: https://github.com/rrousselGit/freezed#adding-getters-and-methods-to-our-models',
);

/// @nodoc
mixin _$FfiDevice {
  String get id => throw _privateConstructorUsedError;
  String get name => throw _privateConstructorUsedError;
  String get deviceType => throw _privateConstructorUsedError;
  String get address => throw _privateConstructorUsedError;
  String get protocol => throw _privateConstructorUsedError;

  /// Create a copy of FfiDevice
  /// with the given fields replaced by the non-null parameter values.
  @JsonKey(includeFromJson: false, includeToJson: false)
  $FfiDeviceCopyWith<FfiDevice> get copyWith =>
      throw _privateConstructorUsedError;
}

/// @nodoc
abstract class $FfiDeviceCopyWith<$Res> {
  factory $FfiDeviceCopyWith(FfiDevice value, $Res Function(FfiDevice) then) =
      _$FfiDeviceCopyWithImpl<$Res, FfiDevice>;
  @useResult
  $Res call({
    String id,
    String name,
    String deviceType,
    String address,
    String protocol,
  });
}

/// @nodoc
class _$FfiDeviceCopyWithImpl<$Res, $Val extends FfiDevice>
    implements $FfiDeviceCopyWith<$Res> {
  _$FfiDeviceCopyWithImpl(this._value, this._then);

  // ignore: unused_field
  final $Val _value;
  // ignore: unused_field
  final $Res Function($Val) _then;

  /// Create a copy of FfiDevice
  /// with the given fields replaced by the non-null parameter values.
  @pragma('vm:prefer-inline')
  @override
  $Res call({
    Object? id = null,
    Object? name = null,
    Object? deviceType = null,
    Object? address = null,
    Object? protocol = null,
  }) {
    return _then(
      _value.copyWith(
            id: null == id
                ? _value.id
                : id // ignore: cast_nullable_to_non_nullable
                      as String,
            name: null == name
                ? _value.name
                : name // ignore: cast_nullable_to_non_nullable
                      as String,
            deviceType: null == deviceType
                ? _value.deviceType
                : deviceType // ignore: cast_nullable_to_non_nullable
                      as String,
            address: null == address
                ? _value.address
                : address // ignore: cast_nullable_to_non_nullable
                      as String,
            protocol: null == protocol
                ? _value.protocol
                : protocol // ignore: cast_nullable_to_non_nullable
                      as String,
          )
          as $Val,
    );
  }
}

/// @nodoc
abstract class _$$FfiDeviceImplCopyWith<$Res>
    implements $FfiDeviceCopyWith<$Res> {
  factory _$$FfiDeviceImplCopyWith(
    _$FfiDeviceImpl value,
    $Res Function(_$FfiDeviceImpl) then,
  ) = __$$FfiDeviceImplCopyWithImpl<$Res>;
  @override
  @useResult
  $Res call({
    String id,
    String name,
    String deviceType,
    String address,
    String protocol,
  });
}

/// @nodoc
class __$$FfiDeviceImplCopyWithImpl<$Res>
    extends _$FfiDeviceCopyWithImpl<$Res, _$FfiDeviceImpl>
    implements _$$FfiDeviceImplCopyWith<$Res> {
  __$$FfiDeviceImplCopyWithImpl(
    _$FfiDeviceImpl _value,
    $Res Function(_$FfiDeviceImpl) _then,
  ) : super(_value, _then);

  /// Create a copy of FfiDevice
  /// with the given fields replaced by the non-null parameter values.
  @pragma('vm:prefer-inline')
  @override
  $Res call({
    Object? id = null,
    Object? name = null,
    Object? deviceType = null,
    Object? address = null,
    Object? protocol = null,
  }) {
    return _then(
      _$FfiDeviceImpl(
        id: null == id
            ? _value.id
            : id // ignore: cast_nullable_to_non_nullable
                  as String,
        name: null == name
            ? _value.name
            : name // ignore: cast_nullable_to_non_nullable
                  as String,
        deviceType: null == deviceType
            ? _value.deviceType
            : deviceType // ignore: cast_nullable_to_non_nullable
                  as String,
        address: null == address
            ? _value.address
            : address // ignore: cast_nullable_to_non_nullable
                  as String,
        protocol: null == protocol
            ? _value.protocol
            : protocol // ignore: cast_nullable_to_non_nullable
                  as String,
      ),
    );
  }
}

/// @nodoc

class _$FfiDeviceImpl implements _FfiDevice {
  const _$FfiDeviceImpl({
    required this.id,
    required this.name,
    required this.deviceType,
    required this.address,
    required this.protocol,
  });

  @override
  final String id;
  @override
  final String name;
  @override
  final String deviceType;
  @override
  final String address;
  @override
  final String protocol;

  @override
  String toString() {
    return 'FfiDevice(id: $id, name: $name, deviceType: $deviceType, address: $address, protocol: $protocol)';
  }

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is _$FfiDeviceImpl &&
            (identical(other.id, id) || other.id == id) &&
            (identical(other.name, name) || other.name == name) &&
            (identical(other.deviceType, deviceType) ||
                other.deviceType == deviceType) &&
            (identical(other.address, address) || other.address == address) &&
            (identical(other.protocol, protocol) ||
                other.protocol == protocol));
  }

  @override
  int get hashCode =>
      Object.hash(runtimeType, id, name, deviceType, address, protocol);

  /// Create a copy of FfiDevice
  /// with the given fields replaced by the non-null parameter values.
  @JsonKey(includeFromJson: false, includeToJson: false)
  @override
  @pragma('vm:prefer-inline')
  _$$FfiDeviceImplCopyWith<_$FfiDeviceImpl> get copyWith =>
      __$$FfiDeviceImplCopyWithImpl<_$FfiDeviceImpl>(this, _$identity);
}

abstract class _FfiDevice implements FfiDevice {
  const factory _FfiDevice({
    required final String id,
    required final String name,
    required final String deviceType,
    required final String address,
    required final String protocol,
  }) = _$FfiDeviceImpl;

  @override
  String get id;
  @override
  String get name;
  @override
  String get deviceType;
  @override
  String get address;
  @override
  String get protocol;

  /// Create a copy of FfiDevice
  /// with the given fields replaced by the non-null parameter values.
  @override
  @JsonKey(includeFromJson: false, includeToJson: false)
  _$$FfiDeviceImplCopyWith<_$FfiDeviceImpl> get copyWith =>
      throw _privateConstructorUsedError;
}

/// @nodoc
mixin _$FfiEvent {
  @optionalTypeArgs
  TResult when<TResult extends Object?>({
    required TResult Function(FfiDevice device) deviceDiscovered,
    required TResult Function(String deviceId) deviceLost,
    required TResult Function(FfiTransferRequest request) transferRequested,
    required TResult Function(FfiTransferProgress progress) transferProgress,
    required TResult Function(String transferId) transferCompleted,
    required TResult Function(String transferId, String error) transferFailed,
    required TResult Function(String message) error,
  }) => throw _privateConstructorUsedError;
  @optionalTypeArgs
  TResult? whenOrNull<TResult extends Object?>({
    TResult? Function(FfiDevice device)? deviceDiscovered,
    TResult? Function(String deviceId)? deviceLost,
    TResult? Function(FfiTransferRequest request)? transferRequested,
    TResult? Function(FfiTransferProgress progress)? transferProgress,
    TResult? Function(String transferId)? transferCompleted,
    TResult? Function(String transferId, String error)? transferFailed,
    TResult? Function(String message)? error,
  }) => throw _privateConstructorUsedError;
  @optionalTypeArgs
  TResult maybeWhen<TResult extends Object?>({
    TResult Function(FfiDevice device)? deviceDiscovered,
    TResult Function(String deviceId)? deviceLost,
    TResult Function(FfiTransferRequest request)? transferRequested,
    TResult Function(FfiTransferProgress progress)? transferProgress,
    TResult Function(String transferId)? transferCompleted,
    TResult Function(String transferId, String error)? transferFailed,
    TResult Function(String message)? error,
    required TResult orElse(),
  }) => throw _privateConstructorUsedError;
  @optionalTypeArgs
  TResult map<TResult extends Object?>({
    required TResult Function(FfiEvent_DeviceDiscovered value) deviceDiscovered,
    required TResult Function(FfiEvent_DeviceLost value) deviceLost,
    required TResult Function(FfiEvent_TransferRequested value)
    transferRequested,
    required TResult Function(FfiEvent_TransferProgress value) transferProgress,
    required TResult Function(FfiEvent_TransferCompleted value)
    transferCompleted,
    required TResult Function(FfiEvent_TransferFailed value) transferFailed,
    required TResult Function(FfiEvent_Error value) error,
  }) => throw _privateConstructorUsedError;
  @optionalTypeArgs
  TResult? mapOrNull<TResult extends Object?>({
    TResult? Function(FfiEvent_DeviceDiscovered value)? deviceDiscovered,
    TResult? Function(FfiEvent_DeviceLost value)? deviceLost,
    TResult? Function(FfiEvent_TransferRequested value)? transferRequested,
    TResult? Function(FfiEvent_TransferProgress value)? transferProgress,
    TResult? Function(FfiEvent_TransferCompleted value)? transferCompleted,
    TResult? Function(FfiEvent_TransferFailed value)? transferFailed,
    TResult? Function(FfiEvent_Error value)? error,
  }) => throw _privateConstructorUsedError;
  @optionalTypeArgs
  TResult maybeMap<TResult extends Object?>({
    TResult Function(FfiEvent_DeviceDiscovered value)? deviceDiscovered,
    TResult Function(FfiEvent_DeviceLost value)? deviceLost,
    TResult Function(FfiEvent_TransferRequested value)? transferRequested,
    TResult Function(FfiEvent_TransferProgress value)? transferProgress,
    TResult Function(FfiEvent_TransferCompleted value)? transferCompleted,
    TResult Function(FfiEvent_TransferFailed value)? transferFailed,
    TResult Function(FfiEvent_Error value)? error,
    required TResult orElse(),
  }) => throw _privateConstructorUsedError;
}

/// @nodoc
abstract class $FfiEventCopyWith<$Res> {
  factory $FfiEventCopyWith(FfiEvent value, $Res Function(FfiEvent) then) =
      _$FfiEventCopyWithImpl<$Res, FfiEvent>;
}

/// @nodoc
class _$FfiEventCopyWithImpl<$Res, $Val extends FfiEvent>
    implements $FfiEventCopyWith<$Res> {
  _$FfiEventCopyWithImpl(this._value, this._then);

  // ignore: unused_field
  final $Val _value;
  // ignore: unused_field
  final $Res Function($Val) _then;

  /// Create a copy of FfiEvent
  /// with the given fields replaced by the non-null parameter values.
}

/// @nodoc
abstract class _$$FfiEvent_DeviceDiscoveredImplCopyWith<$Res> {
  factory _$$FfiEvent_DeviceDiscoveredImplCopyWith(
    _$FfiEvent_DeviceDiscoveredImpl value,
    $Res Function(_$FfiEvent_DeviceDiscoveredImpl) then,
  ) = __$$FfiEvent_DeviceDiscoveredImplCopyWithImpl<$Res>;
  @useResult
  $Res call({FfiDevice device});

  $FfiDeviceCopyWith<$Res> get device;
}

/// @nodoc
class __$$FfiEvent_DeviceDiscoveredImplCopyWithImpl<$Res>
    extends _$FfiEventCopyWithImpl<$Res, _$FfiEvent_DeviceDiscoveredImpl>
    implements _$$FfiEvent_DeviceDiscoveredImplCopyWith<$Res> {
  __$$FfiEvent_DeviceDiscoveredImplCopyWithImpl(
    _$FfiEvent_DeviceDiscoveredImpl _value,
    $Res Function(_$FfiEvent_DeviceDiscoveredImpl) _then,
  ) : super(_value, _then);

  /// Create a copy of FfiEvent
  /// with the given fields replaced by the non-null parameter values.
  @pragma('vm:prefer-inline')
  @override
  $Res call({Object? device = null}) {
    return _then(
      _$FfiEvent_DeviceDiscoveredImpl(
        device: null == device
            ? _value.device
            : device // ignore: cast_nullable_to_non_nullable
                  as FfiDevice,
      ),
    );
  }

  /// Create a copy of FfiEvent
  /// with the given fields replaced by the non-null parameter values.
  @override
  @pragma('vm:prefer-inline')
  $FfiDeviceCopyWith<$Res> get device {
    return $FfiDeviceCopyWith<$Res>(_value.device, (value) {
      return _then(_value.copyWith(device: value));
    });
  }
}

/// @nodoc

class _$FfiEvent_DeviceDiscoveredImpl extends FfiEvent_DeviceDiscovered {
  const _$FfiEvent_DeviceDiscoveredImpl({required this.device}) : super._();

  @override
  final FfiDevice device;

  @override
  String toString() {
    return 'FfiEvent.deviceDiscovered(device: $device)';
  }

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is _$FfiEvent_DeviceDiscoveredImpl &&
            (identical(other.device, device) || other.device == device));
  }

  @override
  int get hashCode => Object.hash(runtimeType, device);

  /// Create a copy of FfiEvent
  /// with the given fields replaced by the non-null parameter values.
  @JsonKey(includeFromJson: false, includeToJson: false)
  @override
  @pragma('vm:prefer-inline')
  _$$FfiEvent_DeviceDiscoveredImplCopyWith<_$FfiEvent_DeviceDiscoveredImpl>
  get copyWith =>
      __$$FfiEvent_DeviceDiscoveredImplCopyWithImpl<
        _$FfiEvent_DeviceDiscoveredImpl
      >(this, _$identity);

  @override
  @optionalTypeArgs
  TResult when<TResult extends Object?>({
    required TResult Function(FfiDevice device) deviceDiscovered,
    required TResult Function(String deviceId) deviceLost,
    required TResult Function(FfiTransferRequest request) transferRequested,
    required TResult Function(FfiTransferProgress progress) transferProgress,
    required TResult Function(String transferId) transferCompleted,
    required TResult Function(String transferId, String error) transferFailed,
    required TResult Function(String message) error,
  }) {
    return deviceDiscovered(device);
  }

  @override
  @optionalTypeArgs
  TResult? whenOrNull<TResult extends Object?>({
    TResult? Function(FfiDevice device)? deviceDiscovered,
    TResult? Function(String deviceId)? deviceLost,
    TResult? Function(FfiTransferRequest request)? transferRequested,
    TResult? Function(FfiTransferProgress progress)? transferProgress,
    TResult? Function(String transferId)? transferCompleted,
    TResult? Function(String transferId, String error)? transferFailed,
    TResult? Function(String message)? error,
  }) {
    return deviceDiscovered?.call(device);
  }

  @override
  @optionalTypeArgs
  TResult maybeWhen<TResult extends Object?>({
    TResult Function(FfiDevice device)? deviceDiscovered,
    TResult Function(String deviceId)? deviceLost,
    TResult Function(FfiTransferRequest request)? transferRequested,
    TResult Function(FfiTransferProgress progress)? transferProgress,
    TResult Function(String transferId)? transferCompleted,
    TResult Function(String transferId, String error)? transferFailed,
    TResult Function(String message)? error,
    required TResult orElse(),
  }) {
    if (deviceDiscovered != null) {
      return deviceDiscovered(device);
    }
    return orElse();
  }

  @override
  @optionalTypeArgs
  TResult map<TResult extends Object?>({
    required TResult Function(FfiEvent_DeviceDiscovered value) deviceDiscovered,
    required TResult Function(FfiEvent_DeviceLost value) deviceLost,
    required TResult Function(FfiEvent_TransferRequested value)
    transferRequested,
    required TResult Function(FfiEvent_TransferProgress value) transferProgress,
    required TResult Function(FfiEvent_TransferCompleted value)
    transferCompleted,
    required TResult Function(FfiEvent_TransferFailed value) transferFailed,
    required TResult Function(FfiEvent_Error value) error,
  }) {
    return deviceDiscovered(this);
  }

  @override
  @optionalTypeArgs
  TResult? mapOrNull<TResult extends Object?>({
    TResult? Function(FfiEvent_DeviceDiscovered value)? deviceDiscovered,
    TResult? Function(FfiEvent_DeviceLost value)? deviceLost,
    TResult? Function(FfiEvent_TransferRequested value)? transferRequested,
    TResult? Function(FfiEvent_TransferProgress value)? transferProgress,
    TResult? Function(FfiEvent_TransferCompleted value)? transferCompleted,
    TResult? Function(FfiEvent_TransferFailed value)? transferFailed,
    TResult? Function(FfiEvent_Error value)? error,
  }) {
    return deviceDiscovered?.call(this);
  }

  @override
  @optionalTypeArgs
  TResult maybeMap<TResult extends Object?>({
    TResult Function(FfiEvent_DeviceDiscovered value)? deviceDiscovered,
    TResult Function(FfiEvent_DeviceLost value)? deviceLost,
    TResult Function(FfiEvent_TransferRequested value)? transferRequested,
    TResult Function(FfiEvent_TransferProgress value)? transferProgress,
    TResult Function(FfiEvent_TransferCompleted value)? transferCompleted,
    TResult Function(FfiEvent_TransferFailed value)? transferFailed,
    TResult Function(FfiEvent_Error value)? error,
    required TResult orElse(),
  }) {
    if (deviceDiscovered != null) {
      return deviceDiscovered(this);
    }
    return orElse();
  }
}

abstract class FfiEvent_DeviceDiscovered extends FfiEvent {
  const factory FfiEvent_DeviceDiscovered({required final FfiDevice device}) =
      _$FfiEvent_DeviceDiscoveredImpl;
  const FfiEvent_DeviceDiscovered._() : super._();

  FfiDevice get device;

  /// Create a copy of FfiEvent
  /// with the given fields replaced by the non-null parameter values.
  @JsonKey(includeFromJson: false, includeToJson: false)
  _$$FfiEvent_DeviceDiscoveredImplCopyWith<_$FfiEvent_DeviceDiscoveredImpl>
  get copyWith => throw _privateConstructorUsedError;
}

/// @nodoc
abstract class _$$FfiEvent_DeviceLostImplCopyWith<$Res> {
  factory _$$FfiEvent_DeviceLostImplCopyWith(
    _$FfiEvent_DeviceLostImpl value,
    $Res Function(_$FfiEvent_DeviceLostImpl) then,
  ) = __$$FfiEvent_DeviceLostImplCopyWithImpl<$Res>;
  @useResult
  $Res call({String deviceId});
}

/// @nodoc
class __$$FfiEvent_DeviceLostImplCopyWithImpl<$Res>
    extends _$FfiEventCopyWithImpl<$Res, _$FfiEvent_DeviceLostImpl>
    implements _$$FfiEvent_DeviceLostImplCopyWith<$Res> {
  __$$FfiEvent_DeviceLostImplCopyWithImpl(
    _$FfiEvent_DeviceLostImpl _value,
    $Res Function(_$FfiEvent_DeviceLostImpl) _then,
  ) : super(_value, _then);

  /// Create a copy of FfiEvent
  /// with the given fields replaced by the non-null parameter values.
  @pragma('vm:prefer-inline')
  @override
  $Res call({Object? deviceId = null}) {
    return _then(
      _$FfiEvent_DeviceLostImpl(
        deviceId: null == deviceId
            ? _value.deviceId
            : deviceId // ignore: cast_nullable_to_non_nullable
                  as String,
      ),
    );
  }
}

/// @nodoc

class _$FfiEvent_DeviceLostImpl extends FfiEvent_DeviceLost {
  const _$FfiEvent_DeviceLostImpl({required this.deviceId}) : super._();

  @override
  final String deviceId;

  @override
  String toString() {
    return 'FfiEvent.deviceLost(deviceId: $deviceId)';
  }

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is _$FfiEvent_DeviceLostImpl &&
            (identical(other.deviceId, deviceId) ||
                other.deviceId == deviceId));
  }

  @override
  int get hashCode => Object.hash(runtimeType, deviceId);

  /// Create a copy of FfiEvent
  /// with the given fields replaced by the non-null parameter values.
  @JsonKey(includeFromJson: false, includeToJson: false)
  @override
  @pragma('vm:prefer-inline')
  _$$FfiEvent_DeviceLostImplCopyWith<_$FfiEvent_DeviceLostImpl> get copyWith =>
      __$$FfiEvent_DeviceLostImplCopyWithImpl<_$FfiEvent_DeviceLostImpl>(
        this,
        _$identity,
      );

  @override
  @optionalTypeArgs
  TResult when<TResult extends Object?>({
    required TResult Function(FfiDevice device) deviceDiscovered,
    required TResult Function(String deviceId) deviceLost,
    required TResult Function(FfiTransferRequest request) transferRequested,
    required TResult Function(FfiTransferProgress progress) transferProgress,
    required TResult Function(String transferId) transferCompleted,
    required TResult Function(String transferId, String error) transferFailed,
    required TResult Function(String message) error,
  }) {
    return deviceLost(deviceId);
  }

  @override
  @optionalTypeArgs
  TResult? whenOrNull<TResult extends Object?>({
    TResult? Function(FfiDevice device)? deviceDiscovered,
    TResult? Function(String deviceId)? deviceLost,
    TResult? Function(FfiTransferRequest request)? transferRequested,
    TResult? Function(FfiTransferProgress progress)? transferProgress,
    TResult? Function(String transferId)? transferCompleted,
    TResult? Function(String transferId, String error)? transferFailed,
    TResult? Function(String message)? error,
  }) {
    return deviceLost?.call(deviceId);
  }

  @override
  @optionalTypeArgs
  TResult maybeWhen<TResult extends Object?>({
    TResult Function(FfiDevice device)? deviceDiscovered,
    TResult Function(String deviceId)? deviceLost,
    TResult Function(FfiTransferRequest request)? transferRequested,
    TResult Function(FfiTransferProgress progress)? transferProgress,
    TResult Function(String transferId)? transferCompleted,
    TResult Function(String transferId, String error)? transferFailed,
    TResult Function(String message)? error,
    required TResult orElse(),
  }) {
    if (deviceLost != null) {
      return deviceLost(deviceId);
    }
    return orElse();
  }

  @override
  @optionalTypeArgs
  TResult map<TResult extends Object?>({
    required TResult Function(FfiEvent_DeviceDiscovered value) deviceDiscovered,
    required TResult Function(FfiEvent_DeviceLost value) deviceLost,
    required TResult Function(FfiEvent_TransferRequested value)
    transferRequested,
    required TResult Function(FfiEvent_TransferProgress value) transferProgress,
    required TResult Function(FfiEvent_TransferCompleted value)
    transferCompleted,
    required TResult Function(FfiEvent_TransferFailed value) transferFailed,
    required TResult Function(FfiEvent_Error value) error,
  }) {
    return deviceLost(this);
  }

  @override
  @optionalTypeArgs
  TResult? mapOrNull<TResult extends Object?>({
    TResult? Function(FfiEvent_DeviceDiscovered value)? deviceDiscovered,
    TResult? Function(FfiEvent_DeviceLost value)? deviceLost,
    TResult? Function(FfiEvent_TransferRequested value)? transferRequested,
    TResult? Function(FfiEvent_TransferProgress value)? transferProgress,
    TResult? Function(FfiEvent_TransferCompleted value)? transferCompleted,
    TResult? Function(FfiEvent_TransferFailed value)? transferFailed,
    TResult? Function(FfiEvent_Error value)? error,
  }) {
    return deviceLost?.call(this);
  }

  @override
  @optionalTypeArgs
  TResult maybeMap<TResult extends Object?>({
    TResult Function(FfiEvent_DeviceDiscovered value)? deviceDiscovered,
    TResult Function(FfiEvent_DeviceLost value)? deviceLost,
    TResult Function(FfiEvent_TransferRequested value)? transferRequested,
    TResult Function(FfiEvent_TransferProgress value)? transferProgress,
    TResult Function(FfiEvent_TransferCompleted value)? transferCompleted,
    TResult Function(FfiEvent_TransferFailed value)? transferFailed,
    TResult Function(FfiEvent_Error value)? error,
    required TResult orElse(),
  }) {
    if (deviceLost != null) {
      return deviceLost(this);
    }
    return orElse();
  }
}

abstract class FfiEvent_DeviceLost extends FfiEvent {
  const factory FfiEvent_DeviceLost({required final String deviceId}) =
      _$FfiEvent_DeviceLostImpl;
  const FfiEvent_DeviceLost._() : super._();

  String get deviceId;

  /// Create a copy of FfiEvent
  /// with the given fields replaced by the non-null parameter values.
  @JsonKey(includeFromJson: false, includeToJson: false)
  _$$FfiEvent_DeviceLostImplCopyWith<_$FfiEvent_DeviceLostImpl> get copyWith =>
      throw _privateConstructorUsedError;
}

/// @nodoc
abstract class _$$FfiEvent_TransferRequestedImplCopyWith<$Res> {
  factory _$$FfiEvent_TransferRequestedImplCopyWith(
    _$FfiEvent_TransferRequestedImpl value,
    $Res Function(_$FfiEvent_TransferRequestedImpl) then,
  ) = __$$FfiEvent_TransferRequestedImplCopyWithImpl<$Res>;
  @useResult
  $Res call({FfiTransferRequest request});

  $FfiTransferRequestCopyWith<$Res> get request;
}

/// @nodoc
class __$$FfiEvent_TransferRequestedImplCopyWithImpl<$Res>
    extends _$FfiEventCopyWithImpl<$Res, _$FfiEvent_TransferRequestedImpl>
    implements _$$FfiEvent_TransferRequestedImplCopyWith<$Res> {
  __$$FfiEvent_TransferRequestedImplCopyWithImpl(
    _$FfiEvent_TransferRequestedImpl _value,
    $Res Function(_$FfiEvent_TransferRequestedImpl) _then,
  ) : super(_value, _then);

  /// Create a copy of FfiEvent
  /// with the given fields replaced by the non-null parameter values.
  @pragma('vm:prefer-inline')
  @override
  $Res call({Object? request = null}) {
    return _then(
      _$FfiEvent_TransferRequestedImpl(
        request: null == request
            ? _value.request
            : request // ignore: cast_nullable_to_non_nullable
                  as FfiTransferRequest,
      ),
    );
  }

  /// Create a copy of FfiEvent
  /// with the given fields replaced by the non-null parameter values.
  @override
  @pragma('vm:prefer-inline')
  $FfiTransferRequestCopyWith<$Res> get request {
    return $FfiTransferRequestCopyWith<$Res>(_value.request, (value) {
      return _then(_value.copyWith(request: value));
    });
  }
}

/// @nodoc

class _$FfiEvent_TransferRequestedImpl extends FfiEvent_TransferRequested {
  const _$FfiEvent_TransferRequestedImpl({required this.request}) : super._();

  @override
  final FfiTransferRequest request;

  @override
  String toString() {
    return 'FfiEvent.transferRequested(request: $request)';
  }

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is _$FfiEvent_TransferRequestedImpl &&
            (identical(other.request, request) || other.request == request));
  }

  @override
  int get hashCode => Object.hash(runtimeType, request);

  /// Create a copy of FfiEvent
  /// with the given fields replaced by the non-null parameter values.
  @JsonKey(includeFromJson: false, includeToJson: false)
  @override
  @pragma('vm:prefer-inline')
  _$$FfiEvent_TransferRequestedImplCopyWith<_$FfiEvent_TransferRequestedImpl>
  get copyWith =>
      __$$FfiEvent_TransferRequestedImplCopyWithImpl<
        _$FfiEvent_TransferRequestedImpl
      >(this, _$identity);

  @override
  @optionalTypeArgs
  TResult when<TResult extends Object?>({
    required TResult Function(FfiDevice device) deviceDiscovered,
    required TResult Function(String deviceId) deviceLost,
    required TResult Function(FfiTransferRequest request) transferRequested,
    required TResult Function(FfiTransferProgress progress) transferProgress,
    required TResult Function(String transferId) transferCompleted,
    required TResult Function(String transferId, String error) transferFailed,
    required TResult Function(String message) error,
  }) {
    return transferRequested(request);
  }

  @override
  @optionalTypeArgs
  TResult? whenOrNull<TResult extends Object?>({
    TResult? Function(FfiDevice device)? deviceDiscovered,
    TResult? Function(String deviceId)? deviceLost,
    TResult? Function(FfiTransferRequest request)? transferRequested,
    TResult? Function(FfiTransferProgress progress)? transferProgress,
    TResult? Function(String transferId)? transferCompleted,
    TResult? Function(String transferId, String error)? transferFailed,
    TResult? Function(String message)? error,
  }) {
    return transferRequested?.call(request);
  }

  @override
  @optionalTypeArgs
  TResult maybeWhen<TResult extends Object?>({
    TResult Function(FfiDevice device)? deviceDiscovered,
    TResult Function(String deviceId)? deviceLost,
    TResult Function(FfiTransferRequest request)? transferRequested,
    TResult Function(FfiTransferProgress progress)? transferProgress,
    TResult Function(String transferId)? transferCompleted,
    TResult Function(String transferId, String error)? transferFailed,
    TResult Function(String message)? error,
    required TResult orElse(),
  }) {
    if (transferRequested != null) {
      return transferRequested(request);
    }
    return orElse();
  }

  @override
  @optionalTypeArgs
  TResult map<TResult extends Object?>({
    required TResult Function(FfiEvent_DeviceDiscovered value) deviceDiscovered,
    required TResult Function(FfiEvent_DeviceLost value) deviceLost,
    required TResult Function(FfiEvent_TransferRequested value)
    transferRequested,
    required TResult Function(FfiEvent_TransferProgress value) transferProgress,
    required TResult Function(FfiEvent_TransferCompleted value)
    transferCompleted,
    required TResult Function(FfiEvent_TransferFailed value) transferFailed,
    required TResult Function(FfiEvent_Error value) error,
  }) {
    return transferRequested(this);
  }

  @override
  @optionalTypeArgs
  TResult? mapOrNull<TResult extends Object?>({
    TResult? Function(FfiEvent_DeviceDiscovered value)? deviceDiscovered,
    TResult? Function(FfiEvent_DeviceLost value)? deviceLost,
    TResult? Function(FfiEvent_TransferRequested value)? transferRequested,
    TResult? Function(FfiEvent_TransferProgress value)? transferProgress,
    TResult? Function(FfiEvent_TransferCompleted value)? transferCompleted,
    TResult? Function(FfiEvent_TransferFailed value)? transferFailed,
    TResult? Function(FfiEvent_Error value)? error,
  }) {
    return transferRequested?.call(this);
  }

  @override
  @optionalTypeArgs
  TResult maybeMap<TResult extends Object?>({
    TResult Function(FfiEvent_DeviceDiscovered value)? deviceDiscovered,
    TResult Function(FfiEvent_DeviceLost value)? deviceLost,
    TResult Function(FfiEvent_TransferRequested value)? transferRequested,
    TResult Function(FfiEvent_TransferProgress value)? transferProgress,
    TResult Function(FfiEvent_TransferCompleted value)? transferCompleted,
    TResult Function(FfiEvent_TransferFailed value)? transferFailed,
    TResult Function(FfiEvent_Error value)? error,
    required TResult orElse(),
  }) {
    if (transferRequested != null) {
      return transferRequested(this);
    }
    return orElse();
  }
}

abstract class FfiEvent_TransferRequested extends FfiEvent {
  const factory FfiEvent_TransferRequested({
    required final FfiTransferRequest request,
  }) = _$FfiEvent_TransferRequestedImpl;
  const FfiEvent_TransferRequested._() : super._();

  FfiTransferRequest get request;

  /// Create a copy of FfiEvent
  /// with the given fields replaced by the non-null parameter values.
  @JsonKey(includeFromJson: false, includeToJson: false)
  _$$FfiEvent_TransferRequestedImplCopyWith<_$FfiEvent_TransferRequestedImpl>
  get copyWith => throw _privateConstructorUsedError;
}

/// @nodoc
abstract class _$$FfiEvent_TransferProgressImplCopyWith<$Res> {
  factory _$$FfiEvent_TransferProgressImplCopyWith(
    _$FfiEvent_TransferProgressImpl value,
    $Res Function(_$FfiEvent_TransferProgressImpl) then,
  ) = __$$FfiEvent_TransferProgressImplCopyWithImpl<$Res>;
  @useResult
  $Res call({FfiTransferProgress progress});

  $FfiTransferProgressCopyWith<$Res> get progress;
}

/// @nodoc
class __$$FfiEvent_TransferProgressImplCopyWithImpl<$Res>
    extends _$FfiEventCopyWithImpl<$Res, _$FfiEvent_TransferProgressImpl>
    implements _$$FfiEvent_TransferProgressImplCopyWith<$Res> {
  __$$FfiEvent_TransferProgressImplCopyWithImpl(
    _$FfiEvent_TransferProgressImpl _value,
    $Res Function(_$FfiEvent_TransferProgressImpl) _then,
  ) : super(_value, _then);

  /// Create a copy of FfiEvent
  /// with the given fields replaced by the non-null parameter values.
  @pragma('vm:prefer-inline')
  @override
  $Res call({Object? progress = null}) {
    return _then(
      _$FfiEvent_TransferProgressImpl(
        progress: null == progress
            ? _value.progress
            : progress // ignore: cast_nullable_to_non_nullable
                  as FfiTransferProgress,
      ),
    );
  }

  /// Create a copy of FfiEvent
  /// with the given fields replaced by the non-null parameter values.
  @override
  @pragma('vm:prefer-inline')
  $FfiTransferProgressCopyWith<$Res> get progress {
    return $FfiTransferProgressCopyWith<$Res>(_value.progress, (value) {
      return _then(_value.copyWith(progress: value));
    });
  }
}

/// @nodoc

class _$FfiEvent_TransferProgressImpl extends FfiEvent_TransferProgress {
  const _$FfiEvent_TransferProgressImpl({required this.progress}) : super._();

  @override
  final FfiTransferProgress progress;

  @override
  String toString() {
    return 'FfiEvent.transferProgress(progress: $progress)';
  }

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is _$FfiEvent_TransferProgressImpl &&
            (identical(other.progress, progress) ||
                other.progress == progress));
  }

  @override
  int get hashCode => Object.hash(runtimeType, progress);

  /// Create a copy of FfiEvent
  /// with the given fields replaced by the non-null parameter values.
  @JsonKey(includeFromJson: false, includeToJson: false)
  @override
  @pragma('vm:prefer-inline')
  _$$FfiEvent_TransferProgressImplCopyWith<_$FfiEvent_TransferProgressImpl>
  get copyWith =>
      __$$FfiEvent_TransferProgressImplCopyWithImpl<
        _$FfiEvent_TransferProgressImpl
      >(this, _$identity);

  @override
  @optionalTypeArgs
  TResult when<TResult extends Object?>({
    required TResult Function(FfiDevice device) deviceDiscovered,
    required TResult Function(String deviceId) deviceLost,
    required TResult Function(FfiTransferRequest request) transferRequested,
    required TResult Function(FfiTransferProgress progress) transferProgress,
    required TResult Function(String transferId) transferCompleted,
    required TResult Function(String transferId, String error) transferFailed,
    required TResult Function(String message) error,
  }) {
    return transferProgress(progress);
  }

  @override
  @optionalTypeArgs
  TResult? whenOrNull<TResult extends Object?>({
    TResult? Function(FfiDevice device)? deviceDiscovered,
    TResult? Function(String deviceId)? deviceLost,
    TResult? Function(FfiTransferRequest request)? transferRequested,
    TResult? Function(FfiTransferProgress progress)? transferProgress,
    TResult? Function(String transferId)? transferCompleted,
    TResult? Function(String transferId, String error)? transferFailed,
    TResult? Function(String message)? error,
  }) {
    return transferProgress?.call(progress);
  }

  @override
  @optionalTypeArgs
  TResult maybeWhen<TResult extends Object?>({
    TResult Function(FfiDevice device)? deviceDiscovered,
    TResult Function(String deviceId)? deviceLost,
    TResult Function(FfiTransferRequest request)? transferRequested,
    TResult Function(FfiTransferProgress progress)? transferProgress,
    TResult Function(String transferId)? transferCompleted,
    TResult Function(String transferId, String error)? transferFailed,
    TResult Function(String message)? error,
    required TResult orElse(),
  }) {
    if (transferProgress != null) {
      return transferProgress(progress);
    }
    return orElse();
  }

  @override
  @optionalTypeArgs
  TResult map<TResult extends Object?>({
    required TResult Function(FfiEvent_DeviceDiscovered value) deviceDiscovered,
    required TResult Function(FfiEvent_DeviceLost value) deviceLost,
    required TResult Function(FfiEvent_TransferRequested value)
    transferRequested,
    required TResult Function(FfiEvent_TransferProgress value) transferProgress,
    required TResult Function(FfiEvent_TransferCompleted value)
    transferCompleted,
    required TResult Function(FfiEvent_TransferFailed value) transferFailed,
    required TResult Function(FfiEvent_Error value) error,
  }) {
    return transferProgress(this);
  }

  @override
  @optionalTypeArgs
  TResult? mapOrNull<TResult extends Object?>({
    TResult? Function(FfiEvent_DeviceDiscovered value)? deviceDiscovered,
    TResult? Function(FfiEvent_DeviceLost value)? deviceLost,
    TResult? Function(FfiEvent_TransferRequested value)? transferRequested,
    TResult? Function(FfiEvent_TransferProgress value)? transferProgress,
    TResult? Function(FfiEvent_TransferCompleted value)? transferCompleted,
    TResult? Function(FfiEvent_TransferFailed value)? transferFailed,
    TResult? Function(FfiEvent_Error value)? error,
  }) {
    return transferProgress?.call(this);
  }

  @override
  @optionalTypeArgs
  TResult maybeMap<TResult extends Object?>({
    TResult Function(FfiEvent_DeviceDiscovered value)? deviceDiscovered,
    TResult Function(FfiEvent_DeviceLost value)? deviceLost,
    TResult Function(FfiEvent_TransferRequested value)? transferRequested,
    TResult Function(FfiEvent_TransferProgress value)? transferProgress,
    TResult Function(FfiEvent_TransferCompleted value)? transferCompleted,
    TResult Function(FfiEvent_TransferFailed value)? transferFailed,
    TResult Function(FfiEvent_Error value)? error,
    required TResult orElse(),
  }) {
    if (transferProgress != null) {
      return transferProgress(this);
    }
    return orElse();
  }
}

abstract class FfiEvent_TransferProgress extends FfiEvent {
  const factory FfiEvent_TransferProgress({
    required final FfiTransferProgress progress,
  }) = _$FfiEvent_TransferProgressImpl;
  const FfiEvent_TransferProgress._() : super._();

  FfiTransferProgress get progress;

  /// Create a copy of FfiEvent
  /// with the given fields replaced by the non-null parameter values.
  @JsonKey(includeFromJson: false, includeToJson: false)
  _$$FfiEvent_TransferProgressImplCopyWith<_$FfiEvent_TransferProgressImpl>
  get copyWith => throw _privateConstructorUsedError;
}

/// @nodoc
abstract class _$$FfiEvent_TransferCompletedImplCopyWith<$Res> {
  factory _$$FfiEvent_TransferCompletedImplCopyWith(
    _$FfiEvent_TransferCompletedImpl value,
    $Res Function(_$FfiEvent_TransferCompletedImpl) then,
  ) = __$$FfiEvent_TransferCompletedImplCopyWithImpl<$Res>;
  @useResult
  $Res call({String transferId});
}

/// @nodoc
class __$$FfiEvent_TransferCompletedImplCopyWithImpl<$Res>
    extends _$FfiEventCopyWithImpl<$Res, _$FfiEvent_TransferCompletedImpl>
    implements _$$FfiEvent_TransferCompletedImplCopyWith<$Res> {
  __$$FfiEvent_TransferCompletedImplCopyWithImpl(
    _$FfiEvent_TransferCompletedImpl _value,
    $Res Function(_$FfiEvent_TransferCompletedImpl) _then,
  ) : super(_value, _then);

  /// Create a copy of FfiEvent
  /// with the given fields replaced by the non-null parameter values.
  @pragma('vm:prefer-inline')
  @override
  $Res call({Object? transferId = null}) {
    return _then(
      _$FfiEvent_TransferCompletedImpl(
        transferId: null == transferId
            ? _value.transferId
            : transferId // ignore: cast_nullable_to_non_nullable
                  as String,
      ),
    );
  }
}

/// @nodoc

class _$FfiEvent_TransferCompletedImpl extends FfiEvent_TransferCompleted {
  const _$FfiEvent_TransferCompletedImpl({required this.transferId})
    : super._();

  @override
  final String transferId;

  @override
  String toString() {
    return 'FfiEvent.transferCompleted(transferId: $transferId)';
  }

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is _$FfiEvent_TransferCompletedImpl &&
            (identical(other.transferId, transferId) ||
                other.transferId == transferId));
  }

  @override
  int get hashCode => Object.hash(runtimeType, transferId);

  /// Create a copy of FfiEvent
  /// with the given fields replaced by the non-null parameter values.
  @JsonKey(includeFromJson: false, includeToJson: false)
  @override
  @pragma('vm:prefer-inline')
  _$$FfiEvent_TransferCompletedImplCopyWith<_$FfiEvent_TransferCompletedImpl>
  get copyWith =>
      __$$FfiEvent_TransferCompletedImplCopyWithImpl<
        _$FfiEvent_TransferCompletedImpl
      >(this, _$identity);

  @override
  @optionalTypeArgs
  TResult when<TResult extends Object?>({
    required TResult Function(FfiDevice device) deviceDiscovered,
    required TResult Function(String deviceId) deviceLost,
    required TResult Function(FfiTransferRequest request) transferRequested,
    required TResult Function(FfiTransferProgress progress) transferProgress,
    required TResult Function(String transferId) transferCompleted,
    required TResult Function(String transferId, String error) transferFailed,
    required TResult Function(String message) error,
  }) {
    return transferCompleted(transferId);
  }

  @override
  @optionalTypeArgs
  TResult? whenOrNull<TResult extends Object?>({
    TResult? Function(FfiDevice device)? deviceDiscovered,
    TResult? Function(String deviceId)? deviceLost,
    TResult? Function(FfiTransferRequest request)? transferRequested,
    TResult? Function(FfiTransferProgress progress)? transferProgress,
    TResult? Function(String transferId)? transferCompleted,
    TResult? Function(String transferId, String error)? transferFailed,
    TResult? Function(String message)? error,
  }) {
    return transferCompleted?.call(transferId);
  }

  @override
  @optionalTypeArgs
  TResult maybeWhen<TResult extends Object?>({
    TResult Function(FfiDevice device)? deviceDiscovered,
    TResult Function(String deviceId)? deviceLost,
    TResult Function(FfiTransferRequest request)? transferRequested,
    TResult Function(FfiTransferProgress progress)? transferProgress,
    TResult Function(String transferId)? transferCompleted,
    TResult Function(String transferId, String error)? transferFailed,
    TResult Function(String message)? error,
    required TResult orElse(),
  }) {
    if (transferCompleted != null) {
      return transferCompleted(transferId);
    }
    return orElse();
  }

  @override
  @optionalTypeArgs
  TResult map<TResult extends Object?>({
    required TResult Function(FfiEvent_DeviceDiscovered value) deviceDiscovered,
    required TResult Function(FfiEvent_DeviceLost value) deviceLost,
    required TResult Function(FfiEvent_TransferRequested value)
    transferRequested,
    required TResult Function(FfiEvent_TransferProgress value) transferProgress,
    required TResult Function(FfiEvent_TransferCompleted value)
    transferCompleted,
    required TResult Function(FfiEvent_TransferFailed value) transferFailed,
    required TResult Function(FfiEvent_Error value) error,
  }) {
    return transferCompleted(this);
  }

  @override
  @optionalTypeArgs
  TResult? mapOrNull<TResult extends Object?>({
    TResult? Function(FfiEvent_DeviceDiscovered value)? deviceDiscovered,
    TResult? Function(FfiEvent_DeviceLost value)? deviceLost,
    TResult? Function(FfiEvent_TransferRequested value)? transferRequested,
    TResult? Function(FfiEvent_TransferProgress value)? transferProgress,
    TResult? Function(FfiEvent_TransferCompleted value)? transferCompleted,
    TResult? Function(FfiEvent_TransferFailed value)? transferFailed,
    TResult? Function(FfiEvent_Error value)? error,
  }) {
    return transferCompleted?.call(this);
  }

  @override
  @optionalTypeArgs
  TResult maybeMap<TResult extends Object?>({
    TResult Function(FfiEvent_DeviceDiscovered value)? deviceDiscovered,
    TResult Function(FfiEvent_DeviceLost value)? deviceLost,
    TResult Function(FfiEvent_TransferRequested value)? transferRequested,
    TResult Function(FfiEvent_TransferProgress value)? transferProgress,
    TResult Function(FfiEvent_TransferCompleted value)? transferCompleted,
    TResult Function(FfiEvent_TransferFailed value)? transferFailed,
    TResult Function(FfiEvent_Error value)? error,
    required TResult orElse(),
  }) {
    if (transferCompleted != null) {
      return transferCompleted(this);
    }
    return orElse();
  }
}

abstract class FfiEvent_TransferCompleted extends FfiEvent {
  const factory FfiEvent_TransferCompleted({required final String transferId}) =
      _$FfiEvent_TransferCompletedImpl;
  const FfiEvent_TransferCompleted._() : super._();

  String get transferId;

  /// Create a copy of FfiEvent
  /// with the given fields replaced by the non-null parameter values.
  @JsonKey(includeFromJson: false, includeToJson: false)
  _$$FfiEvent_TransferCompletedImplCopyWith<_$FfiEvent_TransferCompletedImpl>
  get copyWith => throw _privateConstructorUsedError;
}

/// @nodoc
abstract class _$$FfiEvent_TransferFailedImplCopyWith<$Res> {
  factory _$$FfiEvent_TransferFailedImplCopyWith(
    _$FfiEvent_TransferFailedImpl value,
    $Res Function(_$FfiEvent_TransferFailedImpl) then,
  ) = __$$FfiEvent_TransferFailedImplCopyWithImpl<$Res>;
  @useResult
  $Res call({String transferId, String error});
}

/// @nodoc
class __$$FfiEvent_TransferFailedImplCopyWithImpl<$Res>
    extends _$FfiEventCopyWithImpl<$Res, _$FfiEvent_TransferFailedImpl>
    implements _$$FfiEvent_TransferFailedImplCopyWith<$Res> {
  __$$FfiEvent_TransferFailedImplCopyWithImpl(
    _$FfiEvent_TransferFailedImpl _value,
    $Res Function(_$FfiEvent_TransferFailedImpl) _then,
  ) : super(_value, _then);

  /// Create a copy of FfiEvent
  /// with the given fields replaced by the non-null parameter values.
  @pragma('vm:prefer-inline')
  @override
  $Res call({Object? transferId = null, Object? error = null}) {
    return _then(
      _$FfiEvent_TransferFailedImpl(
        transferId: null == transferId
            ? _value.transferId
            : transferId // ignore: cast_nullable_to_non_nullable
                  as String,
        error: null == error
            ? _value.error
            : error // ignore: cast_nullable_to_non_nullable
                  as String,
      ),
    );
  }
}

/// @nodoc

class _$FfiEvent_TransferFailedImpl extends FfiEvent_TransferFailed {
  const _$FfiEvent_TransferFailedImpl({
    required this.transferId,
    required this.error,
  }) : super._();

  @override
  final String transferId;
  @override
  final String error;

  @override
  String toString() {
    return 'FfiEvent.transferFailed(transferId: $transferId, error: $error)';
  }

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is _$FfiEvent_TransferFailedImpl &&
            (identical(other.transferId, transferId) ||
                other.transferId == transferId) &&
            (identical(other.error, error) || other.error == error));
  }

  @override
  int get hashCode => Object.hash(runtimeType, transferId, error);

  /// Create a copy of FfiEvent
  /// with the given fields replaced by the non-null parameter values.
  @JsonKey(includeFromJson: false, includeToJson: false)
  @override
  @pragma('vm:prefer-inline')
  _$$FfiEvent_TransferFailedImplCopyWith<_$FfiEvent_TransferFailedImpl>
  get copyWith =>
      __$$FfiEvent_TransferFailedImplCopyWithImpl<
        _$FfiEvent_TransferFailedImpl
      >(this, _$identity);

  @override
  @optionalTypeArgs
  TResult when<TResult extends Object?>({
    required TResult Function(FfiDevice device) deviceDiscovered,
    required TResult Function(String deviceId) deviceLost,
    required TResult Function(FfiTransferRequest request) transferRequested,
    required TResult Function(FfiTransferProgress progress) transferProgress,
    required TResult Function(String transferId) transferCompleted,
    required TResult Function(String transferId, String error) transferFailed,
    required TResult Function(String message) error,
  }) {
    return transferFailed(transferId, this.error);
  }

  @override
  @optionalTypeArgs
  TResult? whenOrNull<TResult extends Object?>({
    TResult? Function(FfiDevice device)? deviceDiscovered,
    TResult? Function(String deviceId)? deviceLost,
    TResult? Function(FfiTransferRequest request)? transferRequested,
    TResult? Function(FfiTransferProgress progress)? transferProgress,
    TResult? Function(String transferId)? transferCompleted,
    TResult? Function(String transferId, String error)? transferFailed,
    TResult? Function(String message)? error,
  }) {
    return transferFailed?.call(transferId, this.error);
  }

  @override
  @optionalTypeArgs
  TResult maybeWhen<TResult extends Object?>({
    TResult Function(FfiDevice device)? deviceDiscovered,
    TResult Function(String deviceId)? deviceLost,
    TResult Function(FfiTransferRequest request)? transferRequested,
    TResult Function(FfiTransferProgress progress)? transferProgress,
    TResult Function(String transferId)? transferCompleted,
    TResult Function(String transferId, String error)? transferFailed,
    TResult Function(String message)? error,
    required TResult orElse(),
  }) {
    if (transferFailed != null) {
      return transferFailed(transferId, this.error);
    }
    return orElse();
  }

  @override
  @optionalTypeArgs
  TResult map<TResult extends Object?>({
    required TResult Function(FfiEvent_DeviceDiscovered value) deviceDiscovered,
    required TResult Function(FfiEvent_DeviceLost value) deviceLost,
    required TResult Function(FfiEvent_TransferRequested value)
    transferRequested,
    required TResult Function(FfiEvent_TransferProgress value) transferProgress,
    required TResult Function(FfiEvent_TransferCompleted value)
    transferCompleted,
    required TResult Function(FfiEvent_TransferFailed value) transferFailed,
    required TResult Function(FfiEvent_Error value) error,
  }) {
    return transferFailed(this);
  }

  @override
  @optionalTypeArgs
  TResult? mapOrNull<TResult extends Object?>({
    TResult? Function(FfiEvent_DeviceDiscovered value)? deviceDiscovered,
    TResult? Function(FfiEvent_DeviceLost value)? deviceLost,
    TResult? Function(FfiEvent_TransferRequested value)? transferRequested,
    TResult? Function(FfiEvent_TransferProgress value)? transferProgress,
    TResult? Function(FfiEvent_TransferCompleted value)? transferCompleted,
    TResult? Function(FfiEvent_TransferFailed value)? transferFailed,
    TResult? Function(FfiEvent_Error value)? error,
  }) {
    return transferFailed?.call(this);
  }

  @override
  @optionalTypeArgs
  TResult maybeMap<TResult extends Object?>({
    TResult Function(FfiEvent_DeviceDiscovered value)? deviceDiscovered,
    TResult Function(FfiEvent_DeviceLost value)? deviceLost,
    TResult Function(FfiEvent_TransferRequested value)? transferRequested,
    TResult Function(FfiEvent_TransferProgress value)? transferProgress,
    TResult Function(FfiEvent_TransferCompleted value)? transferCompleted,
    TResult Function(FfiEvent_TransferFailed value)? transferFailed,
    TResult Function(FfiEvent_Error value)? error,
    required TResult orElse(),
  }) {
    if (transferFailed != null) {
      return transferFailed(this);
    }
    return orElse();
  }
}

abstract class FfiEvent_TransferFailed extends FfiEvent {
  const factory FfiEvent_TransferFailed({
    required final String transferId,
    required final String error,
  }) = _$FfiEvent_TransferFailedImpl;
  const FfiEvent_TransferFailed._() : super._();

  String get transferId;
  String get error;

  /// Create a copy of FfiEvent
  /// with the given fields replaced by the non-null parameter values.
  @JsonKey(includeFromJson: false, includeToJson: false)
  _$$FfiEvent_TransferFailedImplCopyWith<_$FfiEvent_TransferFailedImpl>
  get copyWith => throw _privateConstructorUsedError;
}

/// @nodoc
abstract class _$$FfiEvent_ErrorImplCopyWith<$Res> {
  factory _$$FfiEvent_ErrorImplCopyWith(
    _$FfiEvent_ErrorImpl value,
    $Res Function(_$FfiEvent_ErrorImpl) then,
  ) = __$$FfiEvent_ErrorImplCopyWithImpl<$Res>;
  @useResult
  $Res call({String message});
}

/// @nodoc
class __$$FfiEvent_ErrorImplCopyWithImpl<$Res>
    extends _$FfiEventCopyWithImpl<$Res, _$FfiEvent_ErrorImpl>
    implements _$$FfiEvent_ErrorImplCopyWith<$Res> {
  __$$FfiEvent_ErrorImplCopyWithImpl(
    _$FfiEvent_ErrorImpl _value,
    $Res Function(_$FfiEvent_ErrorImpl) _then,
  ) : super(_value, _then);

  /// Create a copy of FfiEvent
  /// with the given fields replaced by the non-null parameter values.
  @pragma('vm:prefer-inline')
  @override
  $Res call({Object? message = null}) {
    return _then(
      _$FfiEvent_ErrorImpl(
        message: null == message
            ? _value.message
            : message // ignore: cast_nullable_to_non_nullable
                  as String,
      ),
    );
  }
}

/// @nodoc

class _$FfiEvent_ErrorImpl extends FfiEvent_Error {
  const _$FfiEvent_ErrorImpl({required this.message}) : super._();

  @override
  final String message;

  @override
  String toString() {
    return 'FfiEvent.error(message: $message)';
  }

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is _$FfiEvent_ErrorImpl &&
            (identical(other.message, message) || other.message == message));
  }

  @override
  int get hashCode => Object.hash(runtimeType, message);

  /// Create a copy of FfiEvent
  /// with the given fields replaced by the non-null parameter values.
  @JsonKey(includeFromJson: false, includeToJson: false)
  @override
  @pragma('vm:prefer-inline')
  _$$FfiEvent_ErrorImplCopyWith<_$FfiEvent_ErrorImpl> get copyWith =>
      __$$FfiEvent_ErrorImplCopyWithImpl<_$FfiEvent_ErrorImpl>(
        this,
        _$identity,
      );

  @override
  @optionalTypeArgs
  TResult when<TResult extends Object?>({
    required TResult Function(FfiDevice device) deviceDiscovered,
    required TResult Function(String deviceId) deviceLost,
    required TResult Function(FfiTransferRequest request) transferRequested,
    required TResult Function(FfiTransferProgress progress) transferProgress,
    required TResult Function(String transferId) transferCompleted,
    required TResult Function(String transferId, String error) transferFailed,
    required TResult Function(String message) error,
  }) {
    return error(message);
  }

  @override
  @optionalTypeArgs
  TResult? whenOrNull<TResult extends Object?>({
    TResult? Function(FfiDevice device)? deviceDiscovered,
    TResult? Function(String deviceId)? deviceLost,
    TResult? Function(FfiTransferRequest request)? transferRequested,
    TResult? Function(FfiTransferProgress progress)? transferProgress,
    TResult? Function(String transferId)? transferCompleted,
    TResult? Function(String transferId, String error)? transferFailed,
    TResult? Function(String message)? error,
  }) {
    return error?.call(message);
  }

  @override
  @optionalTypeArgs
  TResult maybeWhen<TResult extends Object?>({
    TResult Function(FfiDevice device)? deviceDiscovered,
    TResult Function(String deviceId)? deviceLost,
    TResult Function(FfiTransferRequest request)? transferRequested,
    TResult Function(FfiTransferProgress progress)? transferProgress,
    TResult Function(String transferId)? transferCompleted,
    TResult Function(String transferId, String error)? transferFailed,
    TResult Function(String message)? error,
    required TResult orElse(),
  }) {
    if (error != null) {
      return error(message);
    }
    return orElse();
  }

  @override
  @optionalTypeArgs
  TResult map<TResult extends Object?>({
    required TResult Function(FfiEvent_DeviceDiscovered value) deviceDiscovered,
    required TResult Function(FfiEvent_DeviceLost value) deviceLost,
    required TResult Function(FfiEvent_TransferRequested value)
    transferRequested,
    required TResult Function(FfiEvent_TransferProgress value) transferProgress,
    required TResult Function(FfiEvent_TransferCompleted value)
    transferCompleted,
    required TResult Function(FfiEvent_TransferFailed value) transferFailed,
    required TResult Function(FfiEvent_Error value) error,
  }) {
    return error(this);
  }

  @override
  @optionalTypeArgs
  TResult? mapOrNull<TResult extends Object?>({
    TResult? Function(FfiEvent_DeviceDiscovered value)? deviceDiscovered,
    TResult? Function(FfiEvent_DeviceLost value)? deviceLost,
    TResult? Function(FfiEvent_TransferRequested value)? transferRequested,
    TResult? Function(FfiEvent_TransferProgress value)? transferProgress,
    TResult? Function(FfiEvent_TransferCompleted value)? transferCompleted,
    TResult? Function(FfiEvent_TransferFailed value)? transferFailed,
    TResult? Function(FfiEvent_Error value)? error,
  }) {
    return error?.call(this);
  }

  @override
  @optionalTypeArgs
  TResult maybeMap<TResult extends Object?>({
    TResult Function(FfiEvent_DeviceDiscovered value)? deviceDiscovered,
    TResult Function(FfiEvent_DeviceLost value)? deviceLost,
    TResult Function(FfiEvent_TransferRequested value)? transferRequested,
    TResult Function(FfiEvent_TransferProgress value)? transferProgress,
    TResult Function(FfiEvent_TransferCompleted value)? transferCompleted,
    TResult Function(FfiEvent_TransferFailed value)? transferFailed,
    TResult Function(FfiEvent_Error value)? error,
    required TResult orElse(),
  }) {
    if (error != null) {
      return error(this);
    }
    return orElse();
  }
}

abstract class FfiEvent_Error extends FfiEvent {
  const factory FfiEvent_Error({required final String message}) =
      _$FfiEvent_ErrorImpl;
  const FfiEvent_Error._() : super._();

  String get message;

  /// Create a copy of FfiEvent
  /// with the given fields replaced by the non-null parameter values.
  @JsonKey(includeFromJson: false, includeToJson: false)
  _$$FfiEvent_ErrorImplCopyWith<_$FfiEvent_ErrorImpl> get copyWith =>
      throw _privateConstructorUsedError;
}

/// @nodoc
mixin _$FfiFileInfo {
  String get id => throw _privateConstructorUsedError;
  String get name => throw _privateConstructorUsedError;
  BigInt get size => throw _privateConstructorUsedError;
  String? get mimeType => throw _privateConstructorUsedError;

  /// Create a copy of FfiFileInfo
  /// with the given fields replaced by the non-null parameter values.
  @JsonKey(includeFromJson: false, includeToJson: false)
  $FfiFileInfoCopyWith<FfiFileInfo> get copyWith =>
      throw _privateConstructorUsedError;
}

/// @nodoc
abstract class $FfiFileInfoCopyWith<$Res> {
  factory $FfiFileInfoCopyWith(
    FfiFileInfo value,
    $Res Function(FfiFileInfo) then,
  ) = _$FfiFileInfoCopyWithImpl<$Res, FfiFileInfo>;
  @useResult
  $Res call({String id, String name, BigInt size, String? mimeType});
}

/// @nodoc
class _$FfiFileInfoCopyWithImpl<$Res, $Val extends FfiFileInfo>
    implements $FfiFileInfoCopyWith<$Res> {
  _$FfiFileInfoCopyWithImpl(this._value, this._then);

  // ignore: unused_field
  final $Val _value;
  // ignore: unused_field
  final $Res Function($Val) _then;

  /// Create a copy of FfiFileInfo
  /// with the given fields replaced by the non-null parameter values.
  @pragma('vm:prefer-inline')
  @override
  $Res call({
    Object? id = null,
    Object? name = null,
    Object? size = null,
    Object? mimeType = freezed,
  }) {
    return _then(
      _value.copyWith(
            id: null == id
                ? _value.id
                : id // ignore: cast_nullable_to_non_nullable
                      as String,
            name: null == name
                ? _value.name
                : name // ignore: cast_nullable_to_non_nullable
                      as String,
            size: null == size
                ? _value.size
                : size // ignore: cast_nullable_to_non_nullable
                      as BigInt,
            mimeType: freezed == mimeType
                ? _value.mimeType
                : mimeType // ignore: cast_nullable_to_non_nullable
                      as String?,
          )
          as $Val,
    );
  }
}

/// @nodoc
abstract class _$$FfiFileInfoImplCopyWith<$Res>
    implements $FfiFileInfoCopyWith<$Res> {
  factory _$$FfiFileInfoImplCopyWith(
    _$FfiFileInfoImpl value,
    $Res Function(_$FfiFileInfoImpl) then,
  ) = __$$FfiFileInfoImplCopyWithImpl<$Res>;
  @override
  @useResult
  $Res call({String id, String name, BigInt size, String? mimeType});
}

/// @nodoc
class __$$FfiFileInfoImplCopyWithImpl<$Res>
    extends _$FfiFileInfoCopyWithImpl<$Res, _$FfiFileInfoImpl>
    implements _$$FfiFileInfoImplCopyWith<$Res> {
  __$$FfiFileInfoImplCopyWithImpl(
    _$FfiFileInfoImpl _value,
    $Res Function(_$FfiFileInfoImpl) _then,
  ) : super(_value, _then);

  /// Create a copy of FfiFileInfo
  /// with the given fields replaced by the non-null parameter values.
  @pragma('vm:prefer-inline')
  @override
  $Res call({
    Object? id = null,
    Object? name = null,
    Object? size = null,
    Object? mimeType = freezed,
  }) {
    return _then(
      _$FfiFileInfoImpl(
        id: null == id
            ? _value.id
            : id // ignore: cast_nullable_to_non_nullable
                  as String,
        name: null == name
            ? _value.name
            : name // ignore: cast_nullable_to_non_nullable
                  as String,
        size: null == size
            ? _value.size
            : size // ignore: cast_nullable_to_non_nullable
                  as BigInt,
        mimeType: freezed == mimeType
            ? _value.mimeType
            : mimeType // ignore: cast_nullable_to_non_nullable
                  as String?,
      ),
    );
  }
}

/// @nodoc

class _$FfiFileInfoImpl implements _FfiFileInfo {
  const _$FfiFileInfoImpl({
    required this.id,
    required this.name,
    required this.size,
    this.mimeType,
  });

  @override
  final String id;
  @override
  final String name;
  @override
  final BigInt size;
  @override
  final String? mimeType;

  @override
  String toString() {
    return 'FfiFileInfo(id: $id, name: $name, size: $size, mimeType: $mimeType)';
  }

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is _$FfiFileInfoImpl &&
            (identical(other.id, id) || other.id == id) &&
            (identical(other.name, name) || other.name == name) &&
            (identical(other.size, size) || other.size == size) &&
            (identical(other.mimeType, mimeType) ||
                other.mimeType == mimeType));
  }

  @override
  int get hashCode => Object.hash(runtimeType, id, name, size, mimeType);

  /// Create a copy of FfiFileInfo
  /// with the given fields replaced by the non-null parameter values.
  @JsonKey(includeFromJson: false, includeToJson: false)
  @override
  @pragma('vm:prefer-inline')
  _$$FfiFileInfoImplCopyWith<_$FfiFileInfoImpl> get copyWith =>
      __$$FfiFileInfoImplCopyWithImpl<_$FfiFileInfoImpl>(this, _$identity);
}

abstract class _FfiFileInfo implements FfiFileInfo {
  const factory _FfiFileInfo({
    required final String id,
    required final String name,
    required final BigInt size,
    final String? mimeType,
  }) = _$FfiFileInfoImpl;

  @override
  String get id;
  @override
  String get name;
  @override
  BigInt get size;
  @override
  String? get mimeType;

  /// Create a copy of FfiFileInfo
  /// with the given fields replaced by the non-null parameter values.
  @override
  @JsonKey(includeFromJson: false, includeToJson: false)
  _$$FfiFileInfoImplCopyWith<_$FfiFileInfoImpl> get copyWith =>
      throw _privateConstructorUsedError;
}

/// @nodoc
mixin _$FfiLocalInfo {
  String get deviceName => throw _privateConstructorUsedError;
  String get saveDir => throw _privateConstructorUsedError;
  List<String> get protocols => throw _privateConstructorUsedError;

  /// Create a copy of FfiLocalInfo
  /// with the given fields replaced by the non-null parameter values.
  @JsonKey(includeFromJson: false, includeToJson: false)
  $FfiLocalInfoCopyWith<FfiLocalInfo> get copyWith =>
      throw _privateConstructorUsedError;
}

/// @nodoc
abstract class $FfiLocalInfoCopyWith<$Res> {
  factory $FfiLocalInfoCopyWith(
    FfiLocalInfo value,
    $Res Function(FfiLocalInfo) then,
  ) = _$FfiLocalInfoCopyWithImpl<$Res, FfiLocalInfo>;
  @useResult
  $Res call({String deviceName, String saveDir, List<String> protocols});
}

/// @nodoc
class _$FfiLocalInfoCopyWithImpl<$Res, $Val extends FfiLocalInfo>
    implements $FfiLocalInfoCopyWith<$Res> {
  _$FfiLocalInfoCopyWithImpl(this._value, this._then);

  // ignore: unused_field
  final $Val _value;
  // ignore: unused_field
  final $Res Function($Val) _then;

  /// Create a copy of FfiLocalInfo
  /// with the given fields replaced by the non-null parameter values.
  @pragma('vm:prefer-inline')
  @override
  $Res call({
    Object? deviceName = null,
    Object? saveDir = null,
    Object? protocols = null,
  }) {
    return _then(
      _value.copyWith(
            deviceName: null == deviceName
                ? _value.deviceName
                : deviceName // ignore: cast_nullable_to_non_nullable
                      as String,
            saveDir: null == saveDir
                ? _value.saveDir
                : saveDir // ignore: cast_nullable_to_non_nullable
                      as String,
            protocols: null == protocols
                ? _value.protocols
                : protocols // ignore: cast_nullable_to_non_nullable
                      as List<String>,
          )
          as $Val,
    );
  }
}

/// @nodoc
abstract class _$$FfiLocalInfoImplCopyWith<$Res>
    implements $FfiLocalInfoCopyWith<$Res> {
  factory _$$FfiLocalInfoImplCopyWith(
    _$FfiLocalInfoImpl value,
    $Res Function(_$FfiLocalInfoImpl) then,
  ) = __$$FfiLocalInfoImplCopyWithImpl<$Res>;
  @override
  @useResult
  $Res call({String deviceName, String saveDir, List<String> protocols});
}

/// @nodoc
class __$$FfiLocalInfoImplCopyWithImpl<$Res>
    extends _$FfiLocalInfoCopyWithImpl<$Res, _$FfiLocalInfoImpl>
    implements _$$FfiLocalInfoImplCopyWith<$Res> {
  __$$FfiLocalInfoImplCopyWithImpl(
    _$FfiLocalInfoImpl _value,
    $Res Function(_$FfiLocalInfoImpl) _then,
  ) : super(_value, _then);

  /// Create a copy of FfiLocalInfo
  /// with the given fields replaced by the non-null parameter values.
  @pragma('vm:prefer-inline')
  @override
  $Res call({
    Object? deviceName = null,
    Object? saveDir = null,
    Object? protocols = null,
  }) {
    return _then(
      _$FfiLocalInfoImpl(
        deviceName: null == deviceName
            ? _value.deviceName
            : deviceName // ignore: cast_nullable_to_non_nullable
                  as String,
        saveDir: null == saveDir
            ? _value.saveDir
            : saveDir // ignore: cast_nullable_to_non_nullable
                  as String,
        protocols: null == protocols
            ? _value._protocols
            : protocols // ignore: cast_nullable_to_non_nullable
                  as List<String>,
      ),
    );
  }
}

/// @nodoc

class _$FfiLocalInfoImpl implements _FfiLocalInfo {
  const _$FfiLocalInfoImpl({
    required this.deviceName,
    required this.saveDir,
    required final List<String> protocols,
  }) : _protocols = protocols;

  @override
  final String deviceName;
  @override
  final String saveDir;
  final List<String> _protocols;
  @override
  List<String> get protocols {
    if (_protocols is EqualUnmodifiableListView) return _protocols;
    // ignore: implicit_dynamic_type
    return EqualUnmodifiableListView(_protocols);
  }

  @override
  String toString() {
    return 'FfiLocalInfo(deviceName: $deviceName, saveDir: $saveDir, protocols: $protocols)';
  }

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is _$FfiLocalInfoImpl &&
            (identical(other.deviceName, deviceName) ||
                other.deviceName == deviceName) &&
            (identical(other.saveDir, saveDir) || other.saveDir == saveDir) &&
            const DeepCollectionEquality().equals(
              other._protocols,
              _protocols,
            ));
  }

  @override
  int get hashCode => Object.hash(
    runtimeType,
    deviceName,
    saveDir,
    const DeepCollectionEquality().hash(_protocols),
  );

  /// Create a copy of FfiLocalInfo
  /// with the given fields replaced by the non-null parameter values.
  @JsonKey(includeFromJson: false, includeToJson: false)
  @override
  @pragma('vm:prefer-inline')
  _$$FfiLocalInfoImplCopyWith<_$FfiLocalInfoImpl> get copyWith =>
      __$$FfiLocalInfoImplCopyWithImpl<_$FfiLocalInfoImpl>(this, _$identity);
}

abstract class _FfiLocalInfo implements FfiLocalInfo {
  const factory _FfiLocalInfo({
    required final String deviceName,
    required final String saveDir,
    required final List<String> protocols,
  }) = _$FfiLocalInfoImpl;

  @override
  String get deviceName;
  @override
  String get saveDir;
  @override
  List<String> get protocols;

  /// Create a copy of FfiLocalInfo
  /// with the given fields replaced by the non-null parameter values.
  @override
  @JsonKey(includeFromJson: false, includeToJson: false)
  _$$FfiLocalInfoImplCopyWith<_$FfiLocalInfoImpl> get copyWith =>
      throw _privateConstructorUsedError;
}

/// @nodoc
mixin _$FfiTransferProgress {
  String get transferId => throw _privateConstructorUsedError;
  String get fileName => throw _privateConstructorUsedError;
  BigInt get bytesSent => throw _privateConstructorUsedError;
  BigInt get totalBytes => throw _privateConstructorUsedError;
  double get progress => throw _privateConstructorUsedError;

  /// Create a copy of FfiTransferProgress
  /// with the given fields replaced by the non-null parameter values.
  @JsonKey(includeFromJson: false, includeToJson: false)
  $FfiTransferProgressCopyWith<FfiTransferProgress> get copyWith =>
      throw _privateConstructorUsedError;
}

/// @nodoc
abstract class $FfiTransferProgressCopyWith<$Res> {
  factory $FfiTransferProgressCopyWith(
    FfiTransferProgress value,
    $Res Function(FfiTransferProgress) then,
  ) = _$FfiTransferProgressCopyWithImpl<$Res, FfiTransferProgress>;
  @useResult
  $Res call({
    String transferId,
    String fileName,
    BigInt bytesSent,
    BigInt totalBytes,
    double progress,
  });
}

/// @nodoc
class _$FfiTransferProgressCopyWithImpl<$Res, $Val extends FfiTransferProgress>
    implements $FfiTransferProgressCopyWith<$Res> {
  _$FfiTransferProgressCopyWithImpl(this._value, this._then);

  // ignore: unused_field
  final $Val _value;
  // ignore: unused_field
  final $Res Function($Val) _then;

  /// Create a copy of FfiTransferProgress
  /// with the given fields replaced by the non-null parameter values.
  @pragma('vm:prefer-inline')
  @override
  $Res call({
    Object? transferId = null,
    Object? fileName = null,
    Object? bytesSent = null,
    Object? totalBytes = null,
    Object? progress = null,
  }) {
    return _then(
      _value.copyWith(
            transferId: null == transferId
                ? _value.transferId
                : transferId // ignore: cast_nullable_to_non_nullable
                      as String,
            fileName: null == fileName
                ? _value.fileName
                : fileName // ignore: cast_nullable_to_non_nullable
                      as String,
            bytesSent: null == bytesSent
                ? _value.bytesSent
                : bytesSent // ignore: cast_nullable_to_non_nullable
                      as BigInt,
            totalBytes: null == totalBytes
                ? _value.totalBytes
                : totalBytes // ignore: cast_nullable_to_non_nullable
                      as BigInt,
            progress: null == progress
                ? _value.progress
                : progress // ignore: cast_nullable_to_non_nullable
                      as double,
          )
          as $Val,
    );
  }
}

/// @nodoc
abstract class _$$FfiTransferProgressImplCopyWith<$Res>
    implements $FfiTransferProgressCopyWith<$Res> {
  factory _$$FfiTransferProgressImplCopyWith(
    _$FfiTransferProgressImpl value,
    $Res Function(_$FfiTransferProgressImpl) then,
  ) = __$$FfiTransferProgressImplCopyWithImpl<$Res>;
  @override
  @useResult
  $Res call({
    String transferId,
    String fileName,
    BigInt bytesSent,
    BigInt totalBytes,
    double progress,
  });
}

/// @nodoc
class __$$FfiTransferProgressImplCopyWithImpl<$Res>
    extends _$FfiTransferProgressCopyWithImpl<$Res, _$FfiTransferProgressImpl>
    implements _$$FfiTransferProgressImplCopyWith<$Res> {
  __$$FfiTransferProgressImplCopyWithImpl(
    _$FfiTransferProgressImpl _value,
    $Res Function(_$FfiTransferProgressImpl) _then,
  ) : super(_value, _then);

  /// Create a copy of FfiTransferProgress
  /// with the given fields replaced by the non-null parameter values.
  @pragma('vm:prefer-inline')
  @override
  $Res call({
    Object? transferId = null,
    Object? fileName = null,
    Object? bytesSent = null,
    Object? totalBytes = null,
    Object? progress = null,
  }) {
    return _then(
      _$FfiTransferProgressImpl(
        transferId: null == transferId
            ? _value.transferId
            : transferId // ignore: cast_nullable_to_non_nullable
                  as String,
        fileName: null == fileName
            ? _value.fileName
            : fileName // ignore: cast_nullable_to_non_nullable
                  as String,
        bytesSent: null == bytesSent
            ? _value.bytesSent
            : bytesSent // ignore: cast_nullable_to_non_nullable
                  as BigInt,
        totalBytes: null == totalBytes
            ? _value.totalBytes
            : totalBytes // ignore: cast_nullable_to_non_nullable
                  as BigInt,
        progress: null == progress
            ? _value.progress
            : progress // ignore: cast_nullable_to_non_nullable
                  as double,
      ),
    );
  }
}

/// @nodoc

class _$FfiTransferProgressImpl implements _FfiTransferProgress {
  const _$FfiTransferProgressImpl({
    required this.transferId,
    required this.fileName,
    required this.bytesSent,
    required this.totalBytes,
    required this.progress,
  });

  @override
  final String transferId;
  @override
  final String fileName;
  @override
  final BigInt bytesSent;
  @override
  final BigInt totalBytes;
  @override
  final double progress;

  @override
  String toString() {
    return 'FfiTransferProgress(transferId: $transferId, fileName: $fileName, bytesSent: $bytesSent, totalBytes: $totalBytes, progress: $progress)';
  }

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is _$FfiTransferProgressImpl &&
            (identical(other.transferId, transferId) ||
                other.transferId == transferId) &&
            (identical(other.fileName, fileName) ||
                other.fileName == fileName) &&
            (identical(other.bytesSent, bytesSent) ||
                other.bytesSent == bytesSent) &&
            (identical(other.totalBytes, totalBytes) ||
                other.totalBytes == totalBytes) &&
            (identical(other.progress, progress) ||
                other.progress == progress));
  }

  @override
  int get hashCode => Object.hash(
    runtimeType,
    transferId,
    fileName,
    bytesSent,
    totalBytes,
    progress,
  );

  /// Create a copy of FfiTransferProgress
  /// with the given fields replaced by the non-null parameter values.
  @JsonKey(includeFromJson: false, includeToJson: false)
  @override
  @pragma('vm:prefer-inline')
  _$$FfiTransferProgressImplCopyWith<_$FfiTransferProgressImpl> get copyWith =>
      __$$FfiTransferProgressImplCopyWithImpl<_$FfiTransferProgressImpl>(
        this,
        _$identity,
      );
}

abstract class _FfiTransferProgress implements FfiTransferProgress {
  const factory _FfiTransferProgress({
    required final String transferId,
    required final String fileName,
    required final BigInt bytesSent,
    required final BigInt totalBytes,
    required final double progress,
  }) = _$FfiTransferProgressImpl;

  @override
  String get transferId;
  @override
  String get fileName;
  @override
  BigInt get bytesSent;
  @override
  BigInt get totalBytes;
  @override
  double get progress;

  /// Create a copy of FfiTransferProgress
  /// with the given fields replaced by the non-null parameter values.
  @override
  @JsonKey(includeFromJson: false, includeToJson: false)
  _$$FfiTransferProgressImplCopyWith<_$FfiTransferProgressImpl> get copyWith =>
      throw _privateConstructorUsedError;
}

/// @nodoc
mixin _$FfiTransferRequest {
  String get requestId => throw _privateConstructorUsedError;
  FfiDevice get fromDevice => throw _privateConstructorUsedError;
  List<FfiFileInfo> get files => throw _privateConstructorUsedError;
  BigInt get totalSize => throw _privateConstructorUsedError;

  /// Create a copy of FfiTransferRequest
  /// with the given fields replaced by the non-null parameter values.
  @JsonKey(includeFromJson: false, includeToJson: false)
  $FfiTransferRequestCopyWith<FfiTransferRequest> get copyWith =>
      throw _privateConstructorUsedError;
}

/// @nodoc
abstract class $FfiTransferRequestCopyWith<$Res> {
  factory $FfiTransferRequestCopyWith(
    FfiTransferRequest value,
    $Res Function(FfiTransferRequest) then,
  ) = _$FfiTransferRequestCopyWithImpl<$Res, FfiTransferRequest>;
  @useResult
  $Res call({
    String requestId,
    FfiDevice fromDevice,
    List<FfiFileInfo> files,
    BigInt totalSize,
  });

  $FfiDeviceCopyWith<$Res> get fromDevice;
}

/// @nodoc
class _$FfiTransferRequestCopyWithImpl<$Res, $Val extends FfiTransferRequest>
    implements $FfiTransferRequestCopyWith<$Res> {
  _$FfiTransferRequestCopyWithImpl(this._value, this._then);

  // ignore: unused_field
  final $Val _value;
  // ignore: unused_field
  final $Res Function($Val) _then;

  /// Create a copy of FfiTransferRequest
  /// with the given fields replaced by the non-null parameter values.
  @pragma('vm:prefer-inline')
  @override
  $Res call({
    Object? requestId = null,
    Object? fromDevice = null,
    Object? files = null,
    Object? totalSize = null,
  }) {
    return _then(
      _value.copyWith(
            requestId: null == requestId
                ? _value.requestId
                : requestId // ignore: cast_nullable_to_non_nullable
                      as String,
            fromDevice: null == fromDevice
                ? _value.fromDevice
                : fromDevice // ignore: cast_nullable_to_non_nullable
                      as FfiDevice,
            files: null == files
                ? _value.files
                : files // ignore: cast_nullable_to_non_nullable
                      as List<FfiFileInfo>,
            totalSize: null == totalSize
                ? _value.totalSize
                : totalSize // ignore: cast_nullable_to_non_nullable
                      as BigInt,
          )
          as $Val,
    );
  }

  /// Create a copy of FfiTransferRequest
  /// with the given fields replaced by the non-null parameter values.
  @override
  @pragma('vm:prefer-inline')
  $FfiDeviceCopyWith<$Res> get fromDevice {
    return $FfiDeviceCopyWith<$Res>(_value.fromDevice, (value) {
      return _then(_value.copyWith(fromDevice: value) as $Val);
    });
  }
}

/// @nodoc
abstract class _$$FfiTransferRequestImplCopyWith<$Res>
    implements $FfiTransferRequestCopyWith<$Res> {
  factory _$$FfiTransferRequestImplCopyWith(
    _$FfiTransferRequestImpl value,
    $Res Function(_$FfiTransferRequestImpl) then,
  ) = __$$FfiTransferRequestImplCopyWithImpl<$Res>;
  @override
  @useResult
  $Res call({
    String requestId,
    FfiDevice fromDevice,
    List<FfiFileInfo> files,
    BigInt totalSize,
  });

  @override
  $FfiDeviceCopyWith<$Res> get fromDevice;
}

/// @nodoc
class __$$FfiTransferRequestImplCopyWithImpl<$Res>
    extends _$FfiTransferRequestCopyWithImpl<$Res, _$FfiTransferRequestImpl>
    implements _$$FfiTransferRequestImplCopyWith<$Res> {
  __$$FfiTransferRequestImplCopyWithImpl(
    _$FfiTransferRequestImpl _value,
    $Res Function(_$FfiTransferRequestImpl) _then,
  ) : super(_value, _then);

  /// Create a copy of FfiTransferRequest
  /// with the given fields replaced by the non-null parameter values.
  @pragma('vm:prefer-inline')
  @override
  $Res call({
    Object? requestId = null,
    Object? fromDevice = null,
    Object? files = null,
    Object? totalSize = null,
  }) {
    return _then(
      _$FfiTransferRequestImpl(
        requestId: null == requestId
            ? _value.requestId
            : requestId // ignore: cast_nullable_to_non_nullable
                  as String,
        fromDevice: null == fromDevice
            ? _value.fromDevice
            : fromDevice // ignore: cast_nullable_to_non_nullable
                  as FfiDevice,
        files: null == files
            ? _value._files
            : files // ignore: cast_nullable_to_non_nullable
                  as List<FfiFileInfo>,
        totalSize: null == totalSize
            ? _value.totalSize
            : totalSize // ignore: cast_nullable_to_non_nullable
                  as BigInt,
      ),
    );
  }
}

/// @nodoc

class _$FfiTransferRequestImpl implements _FfiTransferRequest {
  const _$FfiTransferRequestImpl({
    required this.requestId,
    required this.fromDevice,
    required final List<FfiFileInfo> files,
    required this.totalSize,
  }) : _files = files;

  @override
  final String requestId;
  @override
  final FfiDevice fromDevice;
  final List<FfiFileInfo> _files;
  @override
  List<FfiFileInfo> get files {
    if (_files is EqualUnmodifiableListView) return _files;
    // ignore: implicit_dynamic_type
    return EqualUnmodifiableListView(_files);
  }

  @override
  final BigInt totalSize;

  @override
  String toString() {
    return 'FfiTransferRequest(requestId: $requestId, fromDevice: $fromDevice, files: $files, totalSize: $totalSize)';
  }

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is _$FfiTransferRequestImpl &&
            (identical(other.requestId, requestId) ||
                other.requestId == requestId) &&
            (identical(other.fromDevice, fromDevice) ||
                other.fromDevice == fromDevice) &&
            const DeepCollectionEquality().equals(other._files, _files) &&
            (identical(other.totalSize, totalSize) ||
                other.totalSize == totalSize));
  }

  @override
  int get hashCode => Object.hash(
    runtimeType,
    requestId,
    fromDevice,
    const DeepCollectionEquality().hash(_files),
    totalSize,
  );

  /// Create a copy of FfiTransferRequest
  /// with the given fields replaced by the non-null parameter values.
  @JsonKey(includeFromJson: false, includeToJson: false)
  @override
  @pragma('vm:prefer-inline')
  _$$FfiTransferRequestImplCopyWith<_$FfiTransferRequestImpl> get copyWith =>
      __$$FfiTransferRequestImplCopyWithImpl<_$FfiTransferRequestImpl>(
        this,
        _$identity,
      );
}

abstract class _FfiTransferRequest implements FfiTransferRequest {
  const factory _FfiTransferRequest({
    required final String requestId,
    required final FfiDevice fromDevice,
    required final List<FfiFileInfo> files,
    required final BigInt totalSize,
  }) = _$FfiTransferRequestImpl;

  @override
  String get requestId;
  @override
  FfiDevice get fromDevice;
  @override
  List<FfiFileInfo> get files;
  @override
  BigInt get totalSize;

  /// Create a copy of FfiTransferRequest
  /// with the given fields replaced by the non-null parameter values.
  @override
  @JsonKey(includeFromJson: false, includeToJson: false)
  _$$FfiTransferRequestImplCopyWith<_$FfiTransferRequestImpl> get copyWith =>
      throw _privateConstructorUsedError;
}
