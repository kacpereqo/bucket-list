class TaskDTO {
  final int id;
  final String title;
  final String description;
  bool isDone;

  TaskDTO({
    required this.id,
    required this.title,
    this.description = "",
    this.isDone = false,
  });
}
