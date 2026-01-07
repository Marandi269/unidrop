import 'package:flutter/material.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';

import 'router.dart';

void main() {
  WidgetsFlutterBinding.ensureInitialized();
  runApp(const ProviderScope(child: UniDropApp()));
}

/// Mintie Tulipan Violet 色系
class AppColors {
  static const Color mint = Color(0xFFACF2D5);
  static const Color mintLight = Color(0xFFAAE4CE);
  static const Color teal = Color(0xFFA8D6C7);
  static const Color tealMuted = Color(0xFFA6C8C1);
  static const Color sage = Color(0xFFA4BBBA);
  static const Color neutral = Color(0xFFA2ADB4);
  static const Color lavender = Color(0xFFA09FAD);
  static const Color purple = Color(0xFF9E92A6);
  static const Color violet = Color(0xFF9C84A0);
  static const Color violetDeep = Color(0xFF9A7699);
  static const Color violetDark = Color(0xFF986993);
}

class UniDropApp extends StatelessWidget {
  const UniDropApp({super.key});

  @override
  Widget build(BuildContext context) {
    return MaterialApp.router(
      title: 'UniDrop',
      debugShowCheckedModeBanner: false,
      routerConfig: router,
      theme: ThemeData(
        colorScheme: ColorScheme.light(
          primary: AppColors.violet,
          onPrimary: Colors.white,
          primaryContainer: AppColors.mint,
          onPrimaryContainer: AppColors.violetDark,
          secondary: AppColors.teal,
          onSecondary: Colors.white,
          secondaryContainer: AppColors.mintLight,
          onSecondaryContainer: AppColors.violetDeep,
          tertiary: AppColors.lavender,
          surface: Colors.white,
          onSurface: const Color(0xFF1C1B1F),
          surfaceContainerHighest: AppColors.mint.withOpacity(0.3),
          outline: AppColors.neutral,
        ),
        useMaterial3: true,
        appBarTheme: const AppBarTheme(
          backgroundColor: AppColors.mint,
          foregroundColor: AppColors.violetDark,
          elevation: 0,
        ),
        floatingActionButtonTheme: const FloatingActionButtonThemeData(
          backgroundColor: AppColors.violet,
          foregroundColor: Colors.white,
        ),
        cardTheme: CardThemeData(
          color: Colors.white,
          elevation: 2,
          shape: RoundedRectangleBorder(
            borderRadius: BorderRadius.circular(16),
          ),
        ),
      ),
      darkTheme: ThemeData(
        colorScheme: ColorScheme.dark(
          primary: AppColors.mint,
          onPrimary: AppColors.violetDark,
          primaryContainer: AppColors.violetDeep,
          onPrimaryContainer: AppColors.mint,
          secondary: AppColors.teal,
          onSecondary: AppColors.violetDark,
          secondaryContainer: AppColors.purple,
          onSecondaryContainer: AppColors.mintLight,
          tertiary: AppColors.lavender,
          surface: const Color(0xFF1C1B1F),
          onSurface: Colors.white,
          surfaceContainerHighest: AppColors.violetDark.withOpacity(0.3),
          outline: AppColors.lavender,
        ),
        useMaterial3: true,
        appBarTheme: AppBarTheme(
          backgroundColor: AppColors.violetDark.withOpacity(0.8),
          foregroundColor: AppColors.mint,
          elevation: 0,
        ),
        floatingActionButtonTheme: const FloatingActionButtonThemeData(
          backgroundColor: AppColors.mint,
          foregroundColor: AppColors.violetDark,
        ),
        cardTheme: CardThemeData(
          color: const Color(0xFF2D2D2D),
          elevation: 2,
          shape: RoundedRectangleBorder(
            borderRadius: BorderRadius.circular(16),
          ),
        ),
      ),
      themeMode: ThemeMode.system,
    );
  }
}
