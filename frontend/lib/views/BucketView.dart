import 'package:bucket_list/dto/bucketDTO.dart';
import 'package:bucket_list/services/bucketService.dart';
import 'package:bucket_list/widgets/todo_list.dart';
import 'package:flutter/material.dart';

class BucketView extends StatefulWidget {
  final BucketDTO bucket;
  const BucketView({super.key, required this.bucket});

  @override
  State<BucketView> createState() => _BucketViewState();
}

class _BucketViewState extends State<BucketView> with TickerProviderStateMixin {
  final TextEditingController nameController = TextEditingController();

  TabController? _controller;

  BucketService bucketService = BucketService.instance;
  void _recreateController() {
    final newController = TabController(
      length: widget.bucket.todoLists.length + 1,
      vsync: this,
    );

    final oldController = _controller;
    _controller = newController;

    setState(() {
      oldController?.dispose();
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
              if (nameController.text.trim().isNotEmpty) {
                setState(() {
                  bucketService.addTodoList(
                    widget.bucket.id,
                    nameController.text,
                  );
                  _recreateController(); // rebuild tabs after list changes
                });
              }
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
            if (index == widget.bucket.todoLists.length) {
              _controller?.animateTo(widget.bucket.todoLists.length - 1);

              showAddDialog();
            }
          },
          tabs: [
            ...widget.bucket.todoLists.map((e) => Tab(text: e.name)),
            const Tab(icon: Icon(Icons.add, size: 20)),
          ],
        ),

        Expanded(
          child: TabBarView(
            controller: _controller,
            children:
                widget.bucket.todoLists.map((e) {
                  return Center(child: TodoList(todolist: e));
                }).toList() +
                [Center()],
          ),
        ),
      ],
    );
  }
}
