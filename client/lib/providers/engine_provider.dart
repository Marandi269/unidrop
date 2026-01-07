// UniDrop Engine Provider - 基于 Rust FFI
import 'dart:async';

import 'package:flutter_riverpod/flutter_riverpod.dart';

import '../src/rust/api.dart' as rust;
import '../src/rust/init.dart';

/// Engine 初始化状态
final engineInitProvider = FutureProvider<rust.FfiLocalInfo>((ref) async {
  print('[UniDrop] 开始初始化 Rust FFI...');
  try {
    // 初始化 Rust FFI
    await initRustLib();
    print('[UniDrop] Rust FFI 初始化完成');

    // 初始化引擎
    print('[UniDrop] 初始化引擎...');
    final info = rust.initEngine();
    print('[UniDrop] 引擎初始化完成: ${info.deviceName}');

    // 启动引擎
    print('[UniDrop] 启动引擎...');
    await rust.startEngine();
    print('[UniDrop] 引擎已启动');

    // 订阅事件
    print('[UniDrop] 订阅事件流...');
    await rust.subscribeEvents(callback: (event) async {
      print('[UniDrop] 收到事件: $event');
      ref.read(eventStreamProvider.notifier).emit(event);
    });
    print('[UniDrop] 事件订阅完成');

    // 触发设备扫描
    print('[UniDrop] 开始扫描设备...');
    await rust.scanDevices();
    print('[UniDrop] 设备扫描已触发');

    return info;
  } catch (e, stack) {
    print('[UniDrop] 初始化失败: $e');
    print('[UniDrop] 堆栈: $stack');
    rethrow;
  }
});

/// 事件流
class EventStreamNotifier extends StateNotifier<rust.FfiEvent?> {
  final StreamController<rust.FfiEvent> _controller =
      StreamController.broadcast();

  EventStreamNotifier() : super(null);

  @override
  Stream<rust.FfiEvent> get stream => _controller.stream;

  void emit(rust.FfiEvent event) {
    state = event;
    _controller.add(event);
  }

  @override
  void dispose() {
    _controller.close();
    super.dispose();
  }
}

final eventStreamProvider =
    StateNotifierProvider<EventStreamNotifier, rust.FfiEvent?>((ref) {
  return EventStreamNotifier();
});

/// 设备列表 - 从 Rust 引擎获取
class DeviceListNotifier extends StateNotifier<List<rust.FfiDevice>> {
  final Ref _ref;
  bool _initialized = false;

  DeviceListNotifier(this._ref) : super([]) {
    // 监听引擎初始化状态
    _ref.listen(engineInitProvider, (prev, next) {
      next.whenData((_) {
        _initialized = true;
      });
    });

    // 监听事件流
    _ref.listen(eventStreamProvider, (prev, next) {
      if (next == null) return;

      next.when(
        deviceDiscovered: (device) {
          // 添加新设备
          if (!state.any((d) => d.id == device.id)) {
            state = [...state, device];
          }
        },
        deviceLost: (deviceId) {
          // 移除设备
          state = state.where((d) => d.id != deviceId).toList();
        },
        transferRequested: (request) {},
        transferProgress: (progress) {},
        transferCompleted: (transferId) {},
        transferFailed: (transferId, error) {},
        error: (message) {},
      );
    });
  }

  Future<void> refresh() async {
    // 确保引擎已初始化
    if (!_initialized) return;

    // 从 Rust 获取最新设备列表
    final devices = await rust.getDevices();
    state = devices;

    // 触发扫描
    await rust.scanDevices();
  }
}

final deviceListProvider =
    StateNotifierProvider<DeviceListNotifier, List<rust.FfiDevice>>((ref) {
  // 确保引擎已初始化
  ref.watch(engineInitProvider);
  return DeviceListNotifier(ref);
});

/// 传输请求队列
class TransferRequestNotifier
    extends StateNotifier<List<rust.FfiTransferRequest>> {
  final Ref _ref;

  TransferRequestNotifier(this._ref) : super([]) {
    _ref.listen(eventStreamProvider, (prev, next) {
      if (next == null) return;

      next.maybeWhen(
        transferRequested: (request) {
          state = [...state, request];
        },
        orElse: () {},
      );
    });
  }

  Future<void> accept(rust.FfiTransferRequest request) async {
    await rust.acceptTransfer(
      requestId: request.requestId,
      protocol: request.fromDevice.protocol,
    );
    state = state.where((r) => r.requestId != request.requestId).toList();
  }

  Future<void> reject(rust.FfiTransferRequest request) async {
    await rust.rejectTransfer(
      requestId: request.requestId,
      protocol: request.fromDevice.protocol,
    );
    state = state.where((r) => r.requestId != request.requestId).toList();
  }
}

final transferRequestProvider = StateNotifierProvider<TransferRequestNotifier,
    List<rust.FfiTransferRequest>>((ref) {
  return TransferRequestNotifier(ref);
});

/// 发送文件
Future<String> sendFiles(
  WidgetRef ref,
  rust.FfiDevice device,
  List<String> filePaths,
) async {
  return await rust.sendFiles(
    deviceId: device.id,
    filePaths: filePaths,
  );
}

/// 协议列表
final protocolsProvider = Provider<List<String>>((ref) {
  final initAsync = ref.watch(engineInitProvider);
  return initAsync.when(
    data: (info) => info.protocols,
    loading: () => [],
    error: (_, __) => [],
  );
});

/// 引擎运行状态
final engineRunningProvider = Provider<bool>((ref) {
  return rust.isEngineRunning();
});
