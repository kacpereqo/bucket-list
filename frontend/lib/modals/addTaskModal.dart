import 'package:flutter/material.dart';

class AddTaskModal extends StatefulWidget {
  final Function(String title, String description) onSave;

  const AddTaskModal({super.key, required this.onSave});

  @override
  State<AddTaskModal> createState() => _AddTaskModalState();
}

class _AddTaskModalState extends State<AddTaskModal> {
  final TextEditingController _titleController = TextEditingController();
  final TextEditingController _descController = TextEditingController();

  @override
  void dispose() {
    _titleController.dispose();
    _descController.dispose();
    super.dispose();
  }

  @override
  Widget build(BuildContext context) {
    return Padding(
      // Important: This pushes the modal up when the keyboard appears
      padding: EdgeInsets.only(
        bottom: MediaQuery.of(context).viewInsets.bottom,
        left: 20,
        right: 20,
        top: 20,
      ),
      child: Column(
        mainAxisSize: MainAxisSize.min, // Fits the content
        crossAxisAlignment: CrossAxisAlignment.start,
        children: [
          const Text(
            "New Task",
            style: TextStyle(fontSize: 22, fontWeight: FontWeight.bold),
          ),
          const SizedBox(height: 20),

          TextField(
            controller: _titleController,
            autofocus: true,
            decoration: const InputDecoration(
              labelText: "Title",
              hintText: "What needs to be done?",
              border: OutlineInputBorder(),
            ),
          ),
          const SizedBox(height: 15),

          TextField(
            controller: _descController,
            maxLines: 3, // Allows for a longer description
            decoration: const InputDecoration(
              labelText: "Description",
              hintText: "Add some details...",
              border: OutlineInputBorder(),
            ),
          ),
          const SizedBox(height: 20),

          SizedBox(
            height: 50,
            width: double.infinity,
            child: ElevatedButton(
              style: ElevatedButton.styleFrom(
                backgroundColor: Theme.of(context).colorScheme.primary,
                foregroundColor: Theme.of(context)
                    .colorScheme
                    .onError, // Text color that looks good on error color
              ),
              onPressed: () {
                final title = _titleController.text.trim();
                final desc = _descController.text.trim();

                if (title.isNotEmpty) {
                  widget.onSave(title, desc);
                  Navigator.pop(context); // Close the bottom sheet
                }
              },
              child: const Text("Create Task"),
            ),
          ),
          const SizedBox(height: 20),
        ],
      ),
    );
  }
}
