/// 设备类型
enum DeviceType {
  mobile,
  desktop,
  web,
  headless,
  server,
}

/// 发现的设备
class Device {
  final String id;
  final String alias;
  final String ip;
  final int port;
  final String fingerprint;
  final DeviceType deviceType;
  final String? deviceModel;
  final String version;
  final bool download;

  const Device({
    required this.id,
    required this.alias,
    required this.ip,
    required this.port,
    required this.fingerprint,
    required this.deviceType,
    this.deviceModel,
    required this.version,
    this.download = false,
  });

  factory Device.fromJson(Map<String, dynamic> json) {
    return Device(
      id: json['fingerprint'] ?? '',
      alias: json['alias'] ?? 'Unknown',
      ip: json['ip'] ?? '',
      port: json['port'] ?? 53317,
      fingerprint: json['fingerprint'] ?? '',
      deviceType: _parseDeviceType(json['deviceType']),
      deviceModel: json['deviceModel'],
      version: json['version'] ?? '2.0',
      download: json['download'] ?? false,
    );
  }

  Map<String, dynamic> toJson() {
    return {
      'alias': alias,
      'ip': ip,
      'port': port,
      'fingerprint': fingerprint,
      'deviceType': deviceType.name,
      'deviceModel': deviceModel,
      'version': version,
      'download': download,
    };
  }

  String get address => '$ip:$port';

  static DeviceType _parseDeviceType(String? type) {
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

  @override
  bool operator ==(Object other) =>
      identical(this, other) ||
      other is Device &&
          runtimeType == other.runtimeType &&
          fingerprint == other.fingerprint;

  @override
  int get hashCode => fingerprint.hashCode;
}
