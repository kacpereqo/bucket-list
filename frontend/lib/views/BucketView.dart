import 'package:bucket_list/services/todoService.dart';
import 'package:flutter/material.dart';
import 'package:bucket_list/dto/todoListDTO.dart';

class BucketView extends StatefulWidget {
  final int id;

  const BucketView({super.key, required this.id});

  @override
  State<BucketView> createState() => _BucketViewState();
}

class _BucketViewState extends State<BucketView>
    with SingleTickerProviderStateMixin {
  final TodoService todoService = TodoService.instance;
  final TextEditingController nameController = TextEditingController();

  List<TodoListDTO> todoLists = [];

  TabController? _controller;

  void _recreateController() {
    final old = _controller;

    _controller = TabController(length: todoLists.length + 1, vsync: this);

    old?.dispose();

    setState(() {});
  }

  void fetchData() {
    setState(() {
      todoLists = todoService.getAll();
      _recreateController();
    });
  }

  void showAddDialog() {
    nameController.clear();

    showDialog(
      context: context,
      builder: (_) => AlertDialog(
        title: const Text("Add Todo list"),
        content: TextField(
          controller: nameController,
          decoration: const InputDecoration(
            labelText: "Todo list name",
            border: OutlineInputBorder(),
          ),
        ),
        actions: [
          TextButton(
            onPressed: () => Navigator.pop(context),
            child: const Text("Close"),
          ),
          TextButton(
            onPressed: () {
              todoService.add(nameController.text);
              fetchData();
              Navigator.pop(context);
            },
            child: const Text("Add"),
          ),
        ],
      ),
    );
  }

  @override
  void initState() {
    super.initState();

    todoLists = todoService.getAll();
    _recreateController();
  }

  @override
  void dispose() {
    _controller?.dispose();
    super.dispose();
  }

  @override
  Widget build(BuildContext context) {
    return Column(
      children: [
        TabBar(
          controller: _controller,
          isScrollable: true,
          labelColor: Colors.blue,
          onTap: (index) {
            if (index == todoLists.length) {
              _controller?.animateTo(todoLists.length - 1);

              showAddDialog();
            }
          },
          tabs: [
            ...todoLists.map((e) => Tab(text: e.name)),
            const Tab(icon: Icon(Icons.add, size: 20)),
          ],
        ),

        Expanded(
          child: TabBarView(
            controller: _controller,
            children:
                todoLists.map((e) {
                  return Center(child: Text(e.name));
                }).toList() +
                [Center()],
          ),
        ),
      ],
    );
  }
}
