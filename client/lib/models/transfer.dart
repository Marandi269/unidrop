import 'device.dart';

/// 传输状态
enum TransferStatus {
  pending,
  waiting,
  transferring,
  completed,
  failed,
  cancelled,
}

/// 传输方向
enum TransferDirection {
  send,
  receive,
}

/// 传输文件信息
class TransferFile {
  final String id;
  final String fileName;
  final int size;
  final String? fileType;
  String? token;
  int bytesTransferred;
  TransferStatus status;

  TransferFile({
    required this.id,
    required this.fileName,
    required this.size,
    this.fileType,
    this.token,
    this.bytesTransferred = 0,
    this.status = TransferStatus.pending,
  });

  double get progress => size > 0 ? bytesTransferred / size : 0;

  factory TransferFile.fromJson(Map<String, dynamic> json) {
    return TransferFile(
      id: json['id'] ?? '',
      fileName: json['fileName'] ?? '',
      size: json['size'] ?? 0,
      fileType: json['fileType'],
      token: json['token'],
    );
  }

  Map<String, dynamic> toJson() {
    return {
      'id': id,
      'fileName': fileName,
      'size': size,
      'fileType': fileType,
    };
  }
}

/// 传输会话
class TransferSession {
  final String sessionId;
  final Device peer;
  final TransferDirection direction;
  final List<TransferFile> files;
  TransferStatus status;
  DateTime startTime;
  DateTime? endTime;

  TransferSession({
    required this.sessionId,
    required this.peer,
    required this.direction,
    required this.files,
    this.status = TransferStatus.pending,
    DateTime? startTime,
    this.endTime,
  }) : startTime = startTime ?? DateTime.now();

  int get totalSize => files.fold(0, (sum, f) => sum + f.size);

  int get bytesTransferred => files.fold(0, (sum, f) => sum + f.bytesTransferred);

  double get progress => totalSize > 0 ? bytesTransferred / totalSize : 0;

  bool get isComplete => status == TransferStatus.completed;
  bool get isFailed => status == TransferStatus.failed;
  bool get isActive =>
      status == TransferStatus.transferring || status == TransferStatus.waiting;
}
