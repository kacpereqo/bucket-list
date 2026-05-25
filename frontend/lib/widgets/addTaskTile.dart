import 'package:flutter/material.dart';

class AddTaskTile extends StatelessWidget {
  final VoidCallback onTap;

  const AddTaskTile({super.key, required this.onTap});

  @override
  Widget build(BuildContext context) {
    return Card(
      margin: const EdgeInsets.symmetric(horizontal: 8, vertical: 4),
      shape: RoundedRectangleBorder(
        borderRadius: BorderRadius.circular(12),
        side: BorderSide(color: Theme.of(context).colorScheme.outline),
      ),
      child: ListTile(
        title: const Text("Create new"),
        leading: Icon(Icons.add),
        onTap: onTap,
      ),
    );
  }
}
