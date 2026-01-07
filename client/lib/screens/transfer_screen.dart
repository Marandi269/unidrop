import 'package:flutter/material.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';

import '../models/transfer.dart';
import '../providers/device_provider.dart';

class TransferScreen extends ConsumerWidget {
  const TransferScreen({super.key});

  @override
  Widget build(BuildContext context, WidgetRef ref) {
    final transfers = ref.watch(transferListProvider);

    return Scaffold(
      appBar: AppBar(
        title: const Text('传输记录'),
      ),
      body: transfers.isEmpty
          ? Center(
              child: Column(
                mainAxisAlignment: MainAxisAlignment.center,
                children: [
                  Icon(
                    Icons.history,
                    size: 64,
                    color: Theme.of(context).colorScheme.outline,
                  ),
                  const SizedBox(height: 16),
                  Text(
                    '暂无传输记录',
                    style: TextStyle(
                      color: Theme.of(context).colorScheme.outline,
                    ),
                  ),
                ],
              ),
            )
          : ListView.builder(
              itemCount: transfers.length,
              itemBuilder: (context, index) {
                final transfer = transfers[index];
                return _TransferCard(transfer: transfer);
              },
            ),
    );
  }
}

class _TransferCard extends StatelessWidget {
  final TransferSession transfer;

  const _TransferCard({required this.transfer});

  @override
  Widget build(BuildContext context) {
    return Card(
      margin: const EdgeInsets.symmetric(horizontal: 16, vertical: 8),
      child: Padding(
        padding: const EdgeInsets.all(16),
        child: Column(
          crossAxisAlignment: CrossAxisAlignment.start,
          children: [
            Row(
              children: [
                Icon(
                  transfer.direction == TransferDirection.send
                      ? Icons.upload
                      : Icons.download,
                  color: Theme.of(context).colorScheme.primary,
                ),
                const SizedBox(width: 8),
                Expanded(
                  child: Text(
                    transfer.peer.alias,
                    style: Theme.of(context).textTheme.titleMedium,
                  ),
                ),
                _StatusChip(status: transfer.status),
              ],
            ),
            const SizedBox(height: 8),
            Text(
              '${transfer.files.length} 个文件',
              style: Theme.of(context).textTheme.bodySmall,
            ),
            if (transfer.isActive) ...[
              const SizedBox(height: 8),
              LinearProgressIndicator(value: transfer.progress),
            ],
          ],
        ),
      ),
    );
  }
}

class _StatusChip extends StatelessWidget {
  final TransferStatus status;

  const _StatusChip({required this.status});

  @override
  Widget build(BuildContext context) {
    final (color, text) = switch (status) {
      TransferStatus.pending => (Colors.grey, '等待'),
      TransferStatus.waiting => (Colors.orange, '等待中'),
      TransferStatus.transferring => (Colors.blue, '传输中'),
      TransferStatus.completed => (Colors.green, '完成'),
      TransferStatus.failed => (Colors.red, '失败'),
      TransferStatus.cancelled => (Colors.grey, '已取消'),
    };

    return Container(
      padding: const EdgeInsets.symmetric(horizontal: 8, vertical: 4),
      decoration: BoxDecoration(
        color: color.withOpacity(0.1),
        borderRadius: BorderRadius.circular(12),
      ),
      child: Text(
        text,
        style: TextStyle(
          color: color,
          fontSize: 12,
          fontWeight: FontWeight.bold,
        ),
      ),
    );
  }
}
