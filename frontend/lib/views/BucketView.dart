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
  late TabController _controller;

  List<TodoListDTO> todoLists = [
    TodoListDTO(id: 0, name: "Bucket 1"),
    TodoListDTO(id: 1, name: "Bucket 2"),
    TodoListDTO(id: 2, name: "Bucket 3"),
  ];

  @override
  void initState() {
    super.initState();

    _controller = TabController(length: todoLists.length + 1, vsync: this);

    _controller.addListener(() {
      // "+" tab clicked
      if (_controller.index == todoLists.length) {
        // go back to previous tab
        _controller.index = _controller.previousIndex;

        showDialog(
          context: context,
          builder: (_) => AlertDialog(
            title: const Text("Add bucket"),
            content: const Text("Popup content here"),
            actions: [
              TextButton(
                onPressed: () => Navigator.pop(context),
                child: const Text("Close"),
              ),
            ],
          ),
        );
      }
    });
  }

  @override
  void dispose() {
    _controller.dispose();
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
          tabs: [
            ...todoLists.map((e) => Tab(text: e.name)),
            const Tab(icon: Icon(Icons.add, size: 20)),
          ],
        ),

        Expanded(
          child: TabBarView(
            controller: _controller,
            children: todoLists.map((e) {
              return Center(child: Text(e.name));
            }).toList(),
          ),
        ),
      ],
    );
  }
}
