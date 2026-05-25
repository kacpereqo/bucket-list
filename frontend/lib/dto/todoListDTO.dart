import 'TaskDTO.dart';

class TodoListDTO {
  final int id;
  final String name;
  List<TaskDTO> tasks = [];

  TodoListDTO({required this.id, required this.name, List<TaskDTO>? tasks})
    : tasks = tasks ?? [];
}
