class TaskDTO {
  final int id;
  final String title;
  bool isDone;

  TaskDTO({required this.id, required this.title, this.isDone = false});
}
