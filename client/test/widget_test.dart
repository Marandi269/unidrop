import 'package:flutter/material.dart';
import 'package:flutter_test/flutter_test.dart';

import 'package:unidrop_app/main.dart';

void main() {
  testWidgets('App builds correctly', (WidgetTester tester) async {
    await tester.pumpWidget(const UniDropApp());
    expect(find.text('UniDrop'), findsOneWidget);
  });
}
