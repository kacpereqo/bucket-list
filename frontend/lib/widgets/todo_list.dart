import 'package:bucket_list/dto/todoListDTO.dart';
import 'package:flutter/material.dart';

class TodoList extends StatefulWidget {
  final TodoListDTO todolist;

  const TodoList({super.key, required this.todolist});

  @override
  State<TodoList> createState() => _TodoListState();
}

class _TodoListState extends State<TodoList> {
  @override
  Widget build(BuildContext context) {
    if (widget.todolist.tasks.isEmpty) {
      return const Center(child: Text("No tasks"));
    }

    return ListView.builder(
      itemCount: widget.todolist.tasks.length,
      itemBuilder: (ctx, index) {
        return ListTile(
          title: Text(widget.todolist.tasks[index].title),
          trailing: Checkbox(
            value: widget.todolist.tasks[index].isDone,
            onChanged: (val) {
              setState(() {
                widget.todolist.tasks[index].isDone = val!;
              });
            },
          ),
        );
      },
    );
  }
}
