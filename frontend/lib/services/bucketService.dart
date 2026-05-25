import 'package:bucket_list/dto/bucketDTO.dart';

class BucketService {
  BucketService._();

  static final BucketService instance = BucketService._();

  final List<BucketDTO> _lists = [
    BucketDTO(id: 0, name: "Bucket 1"),
    BucketDTO(id: 1, name: "Bucket 2"),
    BucketDTO(id: 2, name: "Bucket 3"),
  ];

  List<BucketDTO> getAll() {
    return _lists;
  }

  void add(String name) {
    _lists.add(BucketDTO(id: _lists.length, name: name));
  }

  void delete(int id) {
    _lists.removeWhere((e) => e.id == id);
  }
}
