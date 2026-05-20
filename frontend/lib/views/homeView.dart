import 'package:bucket_list/dto/bucketDTO.dart';
import 'package:bucket_list/views/BucketView.dart';
import 'package:flutter/material.dart';

class HomeView extends StatefulWidget {
  const HomeView({super.key});

  @override
  State<HomeView> createState() => _HomeViewState();
}

class _HomeViewState extends State<HomeView> {
  List<BucketDTO> sideBarBucketList = [
    BucketDTO(id: 0, name: "Bucket 1"),
    BucketDTO(id: 1, name: "Bucket 2"),
    BucketDTO(id: 2, name: "Bucket 3"),
  ];
  int sideBarBucketListIndex = 0;

  BucketView currentBucketView;

  _HomeViewState() : currentBucketView = BucketView(id: 0);

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      appBar: AppBar(
        backgroundColor: Theme.of(context).colorScheme.inversePrimary,
        title: Text("${sideBarBucketList[sideBarBucketListIndex].name} bucket"),
      ),
      drawer: Drawer(
        child: ListView(
          children: List.generate(sideBarBucketList.length, (index) {
            final item = sideBarBucketList[index];

            return ListTile(
              title: Text(item.name),
              onTap: () {
                setState(() {
                  sideBarBucketListIndex = index;
                  currentBucketView = BucketView(id: item.id);
                });
                Navigator.pop(context);
              },
            );
          }),
        ),
      ),
      body: Center(child: currentBucketView),
    );
  }
}
