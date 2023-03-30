
适于 5.7 版本之后

对于文库 电商商品的结构多样化解决方案

document db类型
mysql+redis   多db组合拳 redis一般不能保证事务性

## json数据类型

弱约束

可以存储一对多 多对多 数据 （冗余？数据）

基于虚拟列实现索引优化

### 可以叫的名称
extra ext 等

json 列类型 查询和筛选时的语法格式
~~~sql

select document.* , extra->'$.sub_key' from document
where extra->'$.sub_key' = 'some_val'
~~~

上面一般情况是会进行全表扫描的 千万级别的数据 就可能瘫掉

### 虚拟列
添加一个虚拟列
~~~sql
ALTER TABLE document
ADD COLUMN 'v_col1' varchar(32)
GENERATED ALWAYS AS (json_unquote(json_extract(`extra`, utf8mb4 '$.sub_col'))) VIRTUAL NULL
~~~
NULL 允许为空

需要用索引来优化搜索
针对虚拟字段建立索引

~~~SQL
create index idx_v_sub_col on document(v_col1)
~~~