import 'package:flutter/material.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';

import '../providers/device_provider.dart';

class SettingsScreen extends ConsumerWidget {
  const SettingsScreen({super.key});

  @override
  Widget build(BuildContext context, WidgetRef ref) {
    final initAsync = ref.watch(initProvider);

    return Scaffold(
      appBar: AppBar(
        title: const Text('设置'),
      ),
      body: initAsync.when(
        data: (localInfo) => ListView(
          children: [
            ListTile(
              leading: const Icon(Icons.person),
              title: const Text('设备名称'),
              subtitle: Text(localInfo.alias),
            ),
            ListTile(
              leading: const Icon(Icons.devices),
              title: const Text('设备型号'),
              subtitle: Text(localInfo.deviceModel),
            ),
            ListTile(
              leading: const Icon(Icons.fingerprint),
              title: const Text('设备指纹'),
              subtitle: Text(localInfo.fingerprint),
            ),
            ListTile(
              leading: const Icon(Icons.router),
              title: const Text('端口'),
              subtitle: Text('${localInfo.port}'),
            ),
            const Divider(),
            ListTile(
              leading: const Icon(Icons.info_outline),
              title: const Text('关于'),
              subtitle: const Text('UniDrop v0.1.0'),
              onTap: () {
                showAboutDialog(
                  context: context,
                  applicationName: 'UniDrop',
                  applicationVersion: '0.1.0',
                  applicationLegalese: '© 2024 UniDrop',
                  children: [
                    const SizedBox(height: 16),
                    const Text('跨平台本地文件共享应用'),
                    const Text('兼容 LocalSend 协议'),
                  ],
                );
              },
            ),
          ],
        ),
        loading: () => const Center(child: CircularProgressIndicator()),
        error: (e, _) => Center(child: Text('错误: $e')),
      ),
    );
  }
}
