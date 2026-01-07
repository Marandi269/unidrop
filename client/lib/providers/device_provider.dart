import 'dart:io';

import 'package:flutter_riverpod/flutter_riverpod.dart';

import '../models/device.dart';
import '../models/transfer.dart';
import '../services/localsend_service.dart';

/// LocalSend 服务单例
final localSendServiceProvider = Provider<LocalSendService>((ref) {
  final service = LocalSendService();
  ref.onDispose(() => service.dispose());
  return service;
});

/// 设备列表状态
class DeviceListNotifier extends StateNotifier<List<Device>> {
  final LocalSendService _service;

  DeviceListNotifier(this._service) : super([]) {
    _service.deviceStream.listen((device) {
      if (!state.any((d) => d.fingerprint == device.fingerprint)) {
        state = [...state, device];
      }
    });
  }

  void clear() {
    state = [];
  }

  void addDevice(Device device) {
    if (!state.any((d) => d.fingerprint == device.fingerprint)) {
      state = [...state, device];
    }
  }
}

final deviceListProvider =
    StateNotifierProvider<DeviceListNotifier, List<Device>>((ref) {
  final service = ref.watch(localSendServiceProvider);
  return DeviceListNotifier(service);
});

/// 发现状态
class DiscoveryNotifier extends StateNotifier<bool> {
  final LocalSendService _service;

  DiscoveryNotifier(this._service) : super(false);

  Future<void> start() async {
    if (state) return;
    state = true;
    await _service.startDiscovery();
  }

  void stop() {
    _service.stopDiscovery();
    state = false;
  }

  Future<void> refresh() async {
    await _service.sendAnnouncement();
  }
}

final discoveryProvider =
    StateNotifierProvider<DiscoveryNotifier, bool>((ref) {
  final service = ref.watch(localSendServiceProvider);
  return DiscoveryNotifier(service);
});

/// 初始化状态
final initProvider = FutureProvider<LocalDeviceInfo>((ref) async {
  final service = ref.watch(localSendServiceProvider);
  await service.init();
  return service.localInfo;
});

/// 传输列表
class TransferListNotifier extends StateNotifier<List<TransferSession>> {
  TransferListNotifier() : super([]);

  void add(TransferSession session) {
    state = [...state, session];
  }

  void update(String sessionId, TransferSession session) {
    state = [
      for (final s in state)
        if (s.sessionId == sessionId) session else s,
    ];
  }

  void remove(String sessionId) {
    state = state.where((s) => s.sessionId != sessionId).toList();
  }
}

final transferListProvider =
    StateNotifierProvider<TransferListNotifier, List<TransferSession>>((ref) {
  return TransferListNotifier();
});

/// 发送文件
final sendFilesProvider =
    FutureProvider.family<TransferSession, ({Device target, List<File> files})>(
        (ref, params) async {
  final service = ref.watch(localSendServiceProvider);
  final session = await service.sendFiles(params.target, params.files);
  ref.read(transferListProvider.notifier).add(session);
  return session;
});
