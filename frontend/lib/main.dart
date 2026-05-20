import 'package:flutter/material.dart';
import 'views/homeView.dart';

void main() {
  runApp(const App());
}

class App extends StatelessWidget {
  const App({super.key});

  @override
  Widget build(BuildContext context) {
    return MaterialApp(
      title: 'Bucket List',
      theme: ThemeData(colorScheme: .fromSeed(seedColor: Colors.white)),
      home: const HomeView(),
    );
  }
}
