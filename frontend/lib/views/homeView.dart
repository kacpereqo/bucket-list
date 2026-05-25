import 'package:bucket_list/dto/bucketDTO.dart';
import 'package:bucket_list/services/bucketService.dart';
import 'package:bucket_list/views/BucketView.dart';
import 'package:flutter/material.dart';

class HomeView extends StatefulWidget {
  const HomeView({super.key});

  @override
  State<HomeView> createState() => _HomeViewState();
}

class _HomeViewState extends State<HomeView> {
  final BucketService bucketService = BucketService.instance;

  List<BucketDTO> sideBarBucketList = [];

  int sideBarBucketListIndex = 0;
  late Widget currentBucketView;

  String currentBucketName = "";

  @override
  void initState() {
    super.initState();

    bucketService.getAll().forEach((element) {
      sideBarBucketList.add(element);
    });

    if (sideBarBucketList.isEmpty) {
      currentBucketView = Container();
    } else {
      currentBucketView = BucketView(bucket: sideBarBucketList[0]);
    }

    if (sideBarBucketList.isEmpty) {
      currentBucketName = "-";
    } else {
      currentBucketName =
          "${sideBarBucketList[sideBarBucketListIndex].name} bucket";
    }
  }

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      appBar: AppBar(
        backgroundColor: Theme.of(context).colorScheme.inversePrimary,
        title: Text(currentBucketName),
      ),
      drawer: Drawer(
        child: ListView(
          children: List.generate(sideBarBucketList.length, (index) {
            final item = sideBarBucketList[index];

            return ListTile(
              title: Text(item.name),
              onTap: () {
                setState(() {
                  currentBucketView = BucketView(bucket: item);
                  currentBucketName = "${item.name} bucket";
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
