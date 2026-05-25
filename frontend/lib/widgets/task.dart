import 'package:bucket_list/dto/TaskDTO.dart';
import 'package:bucket_list/services/bucketService.dart';
import 'package:flutter/material.dart';
import 'package:bucket_list/modals/addTaskModal.dart';

class Task extends StatefulWidget {
  final TaskDTO task;

  const Task({super.key, required this.task});

  @override
  State<Task> createState() => _TaskState();
}

class _TaskState extends State<Task> {
  BucketService bucketService = BucketService.instance;

  void _showOptions(BuildContext context) {
    showModalBottomSheet(
      context: context,
      shape: const RoundedRectangleBorder(
        borderRadius: BorderRadius.vertical(top: Radius.circular(20)),
      ),
      builder: (context) {
        return Wrap(
          children: [
            const Padding(
              padding: EdgeInsets.all(16.0),
              child: Text(
                "Task Options",
                style: TextStyle(fontWeight: FontWeight.bold, fontSize: 18),
              ),
            ),
            ListTile(
              leading: const Icon(Icons.delete, color: Colors.red),
              title: const Text(
                "Delete Task",
                style: TextStyle(color: Colors.red),
              ),
              onTap: () {
                bucketService.deleteTask(widget.task.id);
                Navigator.pop(context);
              },
            ),
            const SizedBox(height: 20),
          ],
        );
      },
    );
  }

  void _showEditModal() {
    showModalBottomSheet(
      context: context,
      isScrollControlled: true,
      builder: (context) => AddTaskModal(onSave: (newTitle, newDesc) {}),
    );
  }

  @override
  Widget build(BuildContext context) {
    final isDone = widget.task.isDone;

    return Card(
      margin: const EdgeInsets.symmetric(horizontal: 8, vertical: 4),
      clipBehavior: Clip.antiAlias,
      shape: RoundedRectangleBorder(
        borderRadius: BorderRadius.circular(12),
        side: BorderSide(color: isDone ? Colors.green : Colors.grey.shade300),
      ),
      child: ListTile(
        onLongPress: () => _showOptions(context),
        onTap: () {
          bucketService.toogleTaskIsDone(widget.task.id);
          setState(() => widget.task.isDone = !widget.task.isDone);
        },
        tileColor: isDone
            ? Theme.of(context).colorScheme.surfaceContainerHighest
            : null,
        title: Text(
          widget.task.title,
          style: TextStyle(
            fontWeight: FontWeight.bold,
            decoration: isDone ? TextDecoration.lineThrough : null,
          ),
        ),
        subtitle: Text(
          widget.task.description,
          maxLines: 2,
          overflow: TextOverflow.ellipsis,
          style: TextStyle(fontSize: 12, color: Colors.grey.shade600),
        ),
        trailing: Checkbox(
          value: isDone,
          onChanged: (val) {
            bucketService.toogleTaskIsDone(widget.task.id);
            setState(() => widget.task.isDone = val!);
          },
        ),
      ),
    );
  }
}
