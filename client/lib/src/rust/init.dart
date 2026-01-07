// Rust FFI 初始化
import 'dart:ffi';
import 'dart:io';
import 'package:flutter_rust_bridge/flutter_rust_bridge_for_generated.dart';
import 'frb_generated.dart';

/// 初始化 Rust FFI 库
Future<void> initRustLib() async {
  await RustLib.init(
    externalLibrary: ExternalLibrary.open(_getLibPath()),
  );
}

String _getLibPath() {
  if (Platform.isMacOS) {
    // macOS: 从应用包内加载 (sandbox 要求)
    final execPath = Platform.resolvedExecutable;
    final appBundlePath = execPath.substring(0, execPath.lastIndexOf('/MacOS/'));
    return '$appBundlePath/Frameworks/libunidrop_ffi.dylib';
  } else if (Platform.isLinux) {
    return 'libunidrop_ffi.so';
  } else if (Platform.isWindows) {
    return 'unidrop_ffi.dll';
  }

  throw UnsupportedError('Unsupported platform');
}
