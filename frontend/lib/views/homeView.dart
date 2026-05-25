import 'package:bucket_list/services/bucketService.dart';
import 'package:bucket_list/views/BucketView.dart';
import 'package:flutter/material.dart';

class HomeView extends StatefulWidget {
  const HomeView({super.key});

  @override
  State<HomeView> createState() => _HomeViewState();
}

class _HomeViewState extends State<HomeView> {
  final BucketService bucketService = BucketService.instance;
  int selectedIndex = 0;

  @override
  Widget build(BuildContext context) {
    return ListenableBuilder(
      listenable: bucketService,
      builder: (context, child) {
        final buckets = bucketService.getAll().toList();

        if (selectedIndex >= buckets.length && buckets.isNotEmpty) {
          selectedIndex = buckets.length - 1;
        }

        final selectedBucket = buckets.isEmpty ? null : buckets[selectedIndex];
        final currentBucketName = selectedBucket == null
            ? "No Buckets"
            : "${selectedBucket.name} bucket";

        return Scaffold(
          appBar: AppBar(
            backgroundColor: Theme.of(context).colorScheme.primaryContainer,
            title: Text(
              currentBucketName,
              style: TextStyle(
                color: Theme.of(context).colorScheme.onPrimaryContainer,
                fontWeight: FontWeight.bold,
              ),
            ),
          ),
          drawer: Drawer(
            child: Column(
              children: [
                const DrawerHeader(child: Center(child: Text("My Buckets"))),
                Expanded(
                  child: ListView.builder(
                    itemCount: buckets.length,
                    itemBuilder: (context, index) {
                      final item = buckets[index];
                      return ListTile(
                        title: Text(item.name),
                        selected: index == selectedIndex,
                        onTap: () {
                          setState(() {
                            selectedIndex = index;
                          });
                          Navigator.pop(context);
                        },
                      );
                    },
                  ),
                ),
                ListTile(
                  leading: const Icon(Icons.add),
                  title: const Text("Create New Bucket"),
                  onTap: () {},
                ),
              ],
            ),
          ),
          body: selectedBucket == null
              ? const Center(child: Text("No buckets available"))
              : BucketView(bucket: selectedBucket),
        );
      },
    );
  }
}
