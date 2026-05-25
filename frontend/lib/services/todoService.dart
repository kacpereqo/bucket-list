import 'package:bucket_list/dto/todoListDTO.dart';

class TodoService {
  TodoService._();

  static final TodoService instance = TodoService._();

  final List<TodoListDTO> _lists = [
    TodoListDTO(id: 0, name: "todolist 1"),
    TodoListDTO(id: 1, name: "todolist 2"),
    TodoListDTO(id: 2, name: "todolist 3"),
  ];

  List<TodoListDTO> getAll() {
    return _lists;
  }

  void add(String name) {
    _lists.add(TodoListDTO(id: _lists.length, name: name));
  }

  void delete(int id) {
    _lists.removeWhere((e) => e.id == id);
  }
}
