import 'dart:async';
import 'dart:convert';
import 'dart:io';

import 'package:device_info_plus/device_info_plus.dart';
import 'package:dio/dio.dart';
import 'package:dio/io.dart';
import 'package:uuid/uuid.dart';

import '../models/device.dart';
import '../models/transfer.dart';

/// LocalSend 协议常量
class LocalSendConstants {
  static const int defaultPort = 53317;
  static const String multicastGroup = '224.0.0.167';
  static const String protocolVersion = '2.1';
}

/// 本地设备信息
class LocalDeviceInfo {
  final String alias;
  final String fingerprint;
  final String deviceModel;
  final DeviceType deviceType;
  final int port;
  final String protocol;

  const LocalDeviceInfo({
    required this.alias,
    required this.fingerprint,
    required this.deviceModel,
    required this.deviceType,
    required this.port,
    this.protocol = 'https',
  });

  Map<String, dynamic> toJson() {
    return {
      'alias': alias,
      'version': LocalSendConstants.protocolVersion,
      'deviceModel': deviceModel,
      'deviceType': deviceType.name,
      'fingerprint': fingerprint,
      'port': port,
      'protocol': protocol,
      'download': false,
    };
  }
}

/// LocalSend 服务
class LocalSendService {
  late final Dio _dio;
  LocalDeviceInfo? _localInfo;
  final _uuid = const Uuid();
  bool _isInitialized = false;
  Completer<void>? _initCompleter;

  RawDatagramSocket? _multicastSocket;
  final _deviceController = StreamController<Device>.broadcast();
  final _devices = <String, Device>{};

  Stream<Device> get deviceStream => _deviceController.stream;
  List<Device> get devices => _devices.values.toList();

  LocalSendService() {
    _dio = Dio();
    // 忽略自签名证书
    (_dio.httpClientAdapter as IOHttpClientAdapter).createHttpClient = () {
      final client = HttpClient();
      client.badCertificateCallback = (cert, host, port) => true;
      return client;
    };
  }

  /// 初始化服务
  Future<void> init() async {
    if (_isInitialized) return;
    if (_initCompleter != null) {
      await _initCompleter!.future;
      return;
    }

    _initCompleter = Completer<void>();
    _localInfo = await _getLocalDeviceInfo();
    _isInitialized = true;
    _initCompleter!.complete();
  }

  /// 获取本地设备信息
  Future<LocalDeviceInfo> _getLocalDeviceInfo() async {
    final deviceInfo = DeviceInfoPlugin();
    String alias = 'UniDrop';
    String deviceModel = 'Unknown';
    DeviceType deviceType = DeviceType.desktop;

    if (Platform.isAndroid) {
      final info = await deviceInfo.androidInfo;
      alias = info.model;
      deviceModel = '${info.manufacturer} ${info.model}';
      deviceType = DeviceType.mobile;
    } else if (Platform.isIOS) {
      final info = await deviceInfo.iosInfo;
      alias = info.name;
      deviceModel = info.utsname.machine;
      deviceType = DeviceType.mobile;
    } else if (Platform.isMacOS) {
      final info = await deviceInfo.macOsInfo;
      alias = info.computerName;
      deviceModel = info.model;
      deviceType = DeviceType.desktop;
    } else if (Platform.isWindows) {
      final info = await deviceInfo.windowsInfo;
      alias = info.computerName;
      deviceModel = 'Windows PC';
      deviceType = DeviceType.desktop;
    } else if (Platform.isLinux) {
      final info = await deviceInfo.linuxInfo;
      alias = info.prettyName;
      deviceModel = 'Linux PC';
      deviceType = DeviceType.desktop;
    }

    return LocalDeviceInfo(
      alias: alias,
      fingerprint: _uuid.v4().replaceAll('-', '').toUpperCase().substring(0, 32),
      deviceModel: deviceModel,
      deviceType: deviceType,
      port: LocalSendConstants.defaultPort,
    );
  }

  LocalDeviceInfo get localInfo => _localInfo!;

  /// 开始设备发现
  Future<void> startDiscovery() async {
    await init(); // 确保初始化完成
    await _startMulticastListener();
    await sendAnnouncement();
  }

  /// 停止设备发现
  void stopDiscovery() {
    _multicastSocket?.close();
    _multicastSocket = null;
  }

  /// 启动 multicast 监听
  Future<void> _startMulticastListener() async {
    try {
      _multicastSocket = await RawDatagramSocket.bind(
        InternetAddress.anyIPv4,
        LocalSendConstants.defaultPort,
        reuseAddress: true,
        reusePort: true,
      );

      final multicastAddr = InternetAddress(LocalSendConstants.multicastGroup);
      _multicastSocket!.joinMulticast(multicastAddr);

      _multicastSocket!.listen((event) {
        if (event == RawSocketEvent.read) {
          final datagram = _multicastSocket!.receive();
          if (datagram != null) {
            _handleMulticastMessage(datagram);
          }
        }
      });
    } catch (e) {
      print('Failed to start multicast listener: $e');
    }
  }

  /// 处理 multicast 消息
  void _handleMulticastMessage(Datagram datagram) {
    try {
      final json = jsonDecode(utf8.decode(datagram.data));
      final fingerprint = json['fingerprint'] as String?;

      // 跳过自己
      if (fingerprint == _localInfo?.fingerprint) return;

      final device = Device(
        id: fingerprint ?? '',
        alias: json['alias'] ?? 'Unknown',
        ip: datagram.address.address,
        port: json['port'] ?? LocalSendConstants.defaultPort,
        fingerprint: fingerprint ?? '',
        deviceType: _parseDeviceType(json['deviceType']),
        deviceModel: json['deviceModel'],
        version: json['version'] ?? '2.0',
        download: json['download'] ?? false,
      );

      final isNew = !_devices.containsKey(device.fingerprint);
      _devices[device.fingerprint] = device;

      if (isNew) {
        _deviceController.add(device);
      }

      // 如果是公告，回复
      final isAnnouncement =
          (json['announcement'] == true) || (json['announce'] == true);
      if (isAnnouncement) {
        _sendMulticastResponse();
      }
    } catch (e) {
      print('Failed to parse multicast message: $e');
    }
  }

  /// 发送 multicast 公告
  Future<void> sendAnnouncement() async {
    await init(); // 确保初始化完成
    final message = {
      ..._localInfo!.toJson(),
      'announcement': true,
      'announce': true,
    };

    final data = utf8.encode(jsonEncode(message));
    final addr = InternetAddress(LocalSendConstants.multicastGroup);

    // 发送多次
    for (final delay in [100, 500, 2000]) {
      await Future.delayed(Duration(milliseconds: delay));
      try {
        final socket = await RawDatagramSocket.bind(InternetAddress.anyIPv4, 0);
        socket.send(data, addr, LocalSendConstants.defaultPort);
        socket.close();
      } catch (e) {
        print('Failed to send announcement: $e');
      }
    }
  }

  /// 发送 multicast 响应
  void _sendMulticastResponse() async {
    if (_localInfo == null) return;
    final message = {
      ..._localInfo!.toJson(),
      'announcement': false,
      'announce': false,
    };

    try {
      final socket = await RawDatagramSocket.bind(InternetAddress.anyIPv4, 0);
      final data = utf8.encode(jsonEncode(message));
      final addr = InternetAddress(LocalSendConstants.multicastGroup);
      socket.send(data, addr, LocalSendConstants.defaultPort);
      socket.close();
    } catch (e) {
      print('Failed to send response: $e');
    }
  }

  /// 获取设备信息
  Future<Device?> getDeviceInfo(String ip, int port) async {
    try {
      final response = await _dio.get(
        'https://$ip:$port/api/localsend/v2/info',
        options: Options(receiveTimeout: const Duration(seconds: 5)),
      );

      if (response.statusCode == 200) {
        final json = response.data as Map<String, dynamic>;
        return Device(
          id: json['fingerprint'] ?? '',
          alias: json['alias'] ?? 'Unknown',
          ip: ip,
          port: json['port'] ?? port,
          fingerprint: json['fingerprint'] ?? '',
          deviceType: _parseDeviceType(json['deviceType']),
          deviceModel: json['deviceModel'],
          version: json['version'] ?? '2.0',
          download: json['download'] ?? false,
        );
      }
    } catch (e) {
      print('Failed to get device info: $e');
    }
    return null;
  }

  /// 发送文件
  Future<TransferSession> sendFiles(Device target, List<File> files) async {
    final sessionId = _uuid.v4();
    final transferFiles = <TransferFile>[];

    // 准备文件信息
    final filesJson = <String, dynamic>{};
    for (int i = 0; i < files.length; i++) {
      final file = files[i];
      final fileId = 'file$i';
      final stat = await file.stat();

      transferFiles.add(TransferFile(
        id: fileId,
        fileName: file.path.split('/').last,
        size: stat.size,
        fileType: _getMimeType(file.path),
      ));

      filesJson[fileId] = {
        'id': fileId,
        'fileName': file.path.split('/').last,
        'size': stat.size,
        'fileType': _getMimeType(file.path),
      };
    }

    // 发送 prepare-upload 请求
    final prepareResponse = await _dio.post(
      'https://${target.ip}:${target.port}/api/localsend/v2/prepare-upload',
      data: {
        'info': _localInfo!.toJson(),
        'files': filesJson,
      },
      options: Options(
        contentType: 'application/json',
        receiveTimeout: const Duration(seconds: 30),
      ),
    );

    if (prepareResponse.statusCode != 200) {
      throw Exception('Failed to prepare upload: ${prepareResponse.statusCode}');
    }

    final responseData = prepareResponse.data as Map<String, dynamic>;
    final serverSessionId = responseData['sessionId'] as String;
    final fileTokens = responseData['files'] as Map<String, dynamic>;

    // 更新文件 token
    for (final tf in transferFiles) {
      tf.token = fileTokens[tf.id] as String?;
    }

    final session = TransferSession(
      sessionId: serverSessionId,
      peer: target,
      direction: TransferDirection.send,
      files: transferFiles,
      status: TransferStatus.transferring,
    );

    // 上传每个文件
    for (int i = 0; i < files.length; i++) {
      final file = files[i];
      final tf = transferFiles[i];

      if (tf.token == null) continue;

      try {
        final formData = FormData.fromMap({
          'file': await MultipartFile.fromFile(file.path, filename: tf.fileName),
        });

        await _dio.post(
          'https://${target.ip}:${target.port}/api/localsend/v2/upload'
          '?sessionId=$serverSessionId&fileId=${tf.id}&token=${tf.token}',
          data: formData,
          onSendProgress: (sent, total) {
            tf.bytesTransferred = sent;
          },
        );

        tf.status = TransferStatus.completed;
      } catch (e) {
        tf.status = TransferStatus.failed;
        print('Failed to upload ${tf.fileName}: $e');
      }
    }

    session.status = transferFiles.every((f) => f.status == TransferStatus.completed)
        ? TransferStatus.completed
        : TransferStatus.failed;
    session.endTime = DateTime.now();

    return session;
  }

  DeviceType _parseDeviceType(String? type) {
    switch (type) {
      case 'mobile':
        return DeviceType.mobile;
      case 'desktop':
        return DeviceType.desktop;
      case 'web':
        return DeviceType.web;
      case 'headless':
        return DeviceType.headless;
      case 'server':
        return DeviceType.server;
      default:
        return DeviceType.desktop;
    }
  }

  String _getMimeType(String path) {
    final ext = path.split('.').last.toLowerCase();
    switch (ext) {
      case 'txt':
        return 'text/plain';
      case 'pdf':
        return 'application/pdf';
      case 'jpg':
      case 'jpeg':
        return 'image/jpeg';
      case 'png':
        return 'image/png';
      case 'gif':
        return 'image/gif';
      case 'mp4':
        return 'video/mp4';
      case 'mp3':
        return 'audio/mpeg';
      case 'zip':
        return 'application/zip';
      default:
        return 'application/octet-stream';
    }
  }

  void dispose() {
    stopDiscovery();
    _deviceController.close();
  }
}
