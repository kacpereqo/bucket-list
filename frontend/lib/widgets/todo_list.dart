import 'package:bucket_list/dto/todoListDTO.dart';
import 'package:bucket_list/services/bucketService.dart';
import 'package:bucket_list/widgets/addTaskTile.dart';
import 'package:bucket_list/widgets/task.dart';
import 'package:flutter/material.dart';
import 'package:bucket_list/modals/addTaskModal.dart';

class TodoList extends StatefulWidget {
  final TodoListDTO todolist;

  const TodoList({super.key, required this.todolist});

  @override
  State<TodoList> createState() => _TodoListState();
}

class _TodoListState extends State<TodoList> {
  final BucketService bucketService = BucketService.instance;

  void onAddTaskTilePressed() {
    showModalBottomSheet(
      context: context,
      isScrollControlled: true,
      backgroundColor: Colors.white,
      shape: const RoundedRectangleBorder(
        borderRadius: BorderRadius.vertical(top: Radius.circular(25)),
      ),
      builder: (context) {
        return AddTaskModal(
          onSave: (title, description) {
            bucketService.addTask(widget.todolist.id, title, description);
          },
        );
      },
    );
  }

  @override
  Widget build(BuildContext context) {
    return Column(
      children: [
        AddTaskTile(onTap: onAddTaskTilePressed),
        Expanded(
          child: ListView.builder(
            itemCount: widget.todolist.tasks.length,
            itemBuilder: (ctx, index) {
              return Task(task: widget.todolist.tasks[index]);
            },
          ),
        ),
      ],
    );
  }
}
