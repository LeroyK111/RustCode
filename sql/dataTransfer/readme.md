# ape-dts  数据同步/校验/订阅/加工

官网: https://github.com/apecloud/ape-dts

- ape-dts 是一款旨在实现 any-to-any 的数据迁移工具。
- 极其简单轻量，不依赖第三方组件和额外存储。
- 提供对多种数据库的：库表结构迁移，全量数据迁移，增量数据迁移，数据校验，数据订正，数据订阅，断点续传，etl 等能力。
- 使用 Rust。

# 支持任务类型

![](https://raw.githubusercontent.com/LeroyK111/pictureBed/master/20241126213654.png)

![](https://raw.githubusercontent.com/LeroyK111/pictureBed/master/20241126213853.png)

## 极简使用示例
例子：同步 mysql 源库 test_db 的全量数据到 mysql 目标库

- mysql_1
```sh
mysql -h127.0.0.1 -uroot -p123456 -P3307  
  
CREATE DATABASE test_db;  
CREATE TABLE test_db.tb_1(id int, value int, primary key(id));  
CREATE TABLE test_db.tb_2(id int, value int, primary key(id));  
  
INSERT INTO test_db.tb_1 VALUES(1,1),(2,2);  
INSERT INTO test_db.tb_2 VALUES(5,5),(6,6);
```
- mysql_2
```sh
mysql -h127.0.0.1 -uroot -p123456 -P3308  
  
CREATE DATABASE test_db;  
CREATE TABLE test_db.tb_1(id int, value int, primary key(id));  
CREATE TABLE test_db.tb_2(id int, value int, primary key(id));
```

### 创建任务配置
```sh
rm -rf /tmp/ape_dts
mkdir -p /tmp/ape_dts
```
创建脚本
```sh
cat <<EOL > /tmp/ape_dts/task_config.ini 
```
task_config.ini 
```ini
[extractor]  
db_type=mysql  
extract_type=snapshot  
url=mysql://root:123456@127.0.0.1:3307?ssl-mode=disabled  
  
[sinker]  
db_type=mysql  
sink_type=write  
url=mysql://root:123456@127.0.0.1:3308?ssl-mode=disabled  
  
[filter]  
do_dbs=test_db  
do_events=insert  
  
[parallelizer]  
parallel_type=snapshot  
parallel_size=8  
  
[pipeline]  
buffer_size=16000  
checkpoint_interval_secs=1  
EOL
```

同步全量数据(这里使用了ape-dts:2.0.8 容器的镜像)
``` sh
# 环境变量
export APE_DTS_IMAGE="apecloud-registry.cn-zhangjiakou.cr.aliyuncs.com/apecloud/ape-dts:2.0.8"  
# 执行脚本
docker run --rm --network host \  
-v "/tmp/ape_dts/task_config.ini:/task_config.ini" \  
"$APE_DTS_IMAGE" /task_config.ini
```
检查目标库
```sh
mysql -h 127.0.0.1 -u root -p 123456 -P 3308  
  
mysql> SELECT * FROM test_db.tb_1;  
+----+-------+  
| id | value |  
+----+-------+  
| 1 | 1 |  
| 2 | 2 |  
+----+-------+  
  
mysql> SELECT * FROM test_db.tb_2;  
+----+-------+  
| id | value |  
+----+-------+  
| 5 | 5 |  
| 6 | 6 |  
+----+-------+
```


