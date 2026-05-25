import 'package:bucket_list/dto/todoListDTO.dart';

class BucketDTO {
  final int id;
  final String name;

  List<TodoListDTO> todoLists = [];

  BucketDTO({required this.id, required this.name, this.todoLists = const []});
}
