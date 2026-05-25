import 'package:bucket_list/dto/bucketDTO.dart';
import 'package:bucket_list/dto/todoListDTO.dart';

class BucketService {
  BucketService._();

  static final BucketService instance = BucketService._();

  final List<BucketDTO> _lists = [
    BucketDTO(
      id: 0,
      name: "Bucket 1",
      todoLists: [
        TodoListDTO(id: 1, name: "Todo list 1"),
        TodoListDTO(id: 2, name: "Todo list 2"),
        TodoListDTO(id: 3, name: "Todo list 3"),
      ],
    ),
    BucketDTO(
      id: 1,
      name: "Bucket 2",
      todoLists: [
        TodoListDTO(id: 4, name: "Todo list 1"),
        TodoListDTO(id: 5, name: "Todo list 2"),
        TodoListDTO(id: 6, name: "Todo list 3"),
      ],
    ),
    BucketDTO(
      id: 2,
      name: "Bucket 3",
      todoLists: [
        TodoListDTO(id: 7, name: "Todo list 1"),
        TodoListDTO(id: 8, name: "Todo list 2"),
        TodoListDTO(id: 9, name: "Todo list 3"),
      ],
    ),
  ];

  List<BucketDTO> getAll() {
    return _lists;
  }

  void addBucket(String name) {
    _lists.add(BucketDTO(id: _lists.length, name: name));
  }

  void addTodoList(int bucketId, String name) {
    _lists
        .firstWhere((e) => e.id == bucketId)
        .todoLists
        .add(TodoListDTO(id: _lists.length, name: name));
  }

  void delete(int id) {
    _lists.removeWhere((e) => e.id == id);
  }
}
