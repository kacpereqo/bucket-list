import 'package:bucket_list/dto/bucketDTO.dart';
import 'package:bucket_list/dto/todoListDTO.dart';
import 'package:bucket_list/dto/TaskDTO.dart';
import 'package:flutter/material.dart';

class BucketService extends ChangeNotifier {
  BucketService._();

  static final BucketService instance = BucketService._();

  List<BucketDTO> _lists = [
    BucketDTO(
      id: 0,
      name: "Bucket 1",
      todoLists: [
        TodoListDTO(
          id: 1,
          name: "Todo list 1",
          tasks: [
            TaskDTO(id: 1, title: "Task 1"),
            TaskDTO(id: 2, title: "Task 2"),
            TaskDTO(id: 3, title: "Task 3"),
          ],
        ),
        TodoListDTO(id: 2, name: "Todo list 2"),
        TodoListDTO(id: 3, name: "Todo list 3"),
      ],
    ),
  ];

  List<BucketDTO> getAll() {
    return _lists;
  }

  void addBucket(String name) {
    _lists.add(BucketDTO(id: _lists.length, name: name));

    notifyListeners();
  }

  void addTodoList(int bucketId, String name) {
    _lists
        .firstWhere((e) => e.id == bucketId)
        .todoLists
        .add(
          TodoListDTO(id: DateTime.now().millisecondsSinceEpoch, name: name),
        );

    notifyListeners();
  }

  void addTask(int todoListId, String title, String description) {
    for (var bucket in _lists) {
      for (var list in bucket.todoLists) {
        if (list.id == todoListId) {
          list.tasks.add(
            TaskDTO(
              id: DateTime.now().millisecondsSinceEpoch,
              title: title,
              description: description,
            ),
          );

          notifyListeners();
          return;
        }
      }
    }
  }

  void deleteTask(int taskId) {
    for (var bucket in _lists) {
      for (var list in bucket.todoLists) {
        list.tasks.removeWhere((task) => task.id == taskId);
      }
    }
    notifyListeners();
  }

  void delete(int id) {
    _lists.removeWhere((e) => e.id == id);

    notifyListeners();
  }

  void toogleTaskIsDone(int taskId) {
    notifyListeners();
  }
}
