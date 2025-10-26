use serde::{Deserialize, Serialize};

// 定义数据结构
#[derive(Serialize, Deserialize, Debug)]
struct Document {
    title: String,
    body: String,
}

use tantivy::{doc, schema::*, Index};

// 创建索引
fn create_index() -> Index {
    // 定义Schema
    let mut schema_builder = Schema::builder();
    schema_builder.add_text_field("title", TEXT | STORED);
    schema_builder.add_text_field("body", TEXT);
    let schema = schema_builder.build(); // 在目录中创建索引
    let index = Index::create_in_ram(schema.clone()); // 获取索引写入器
    let mut index_writer = index.writer(50_000_000).unwrap(); // 添加文档
    let title = schema.get_field("title").unwrap();
    let body = schema.get_field("body").unwrap();
    let doc = doc!(title => "Example Title", body => "This is the body of the document.");
    let _ = index_writer.add_document(doc); // 将文档提交到索引
    let _ = index_writer.commit();
    index
}

use tantivy::collector::TopDocs;
use tantivy::query::QueryParser;

// 搜索
fn search_index(index: &Index, query_str: &str) -> tantivy::Result<()> {
    let reader = index.reader()?;
    let searcher = reader.searcher();
    let schema = index.schema();
    let title = schema.get_field("title").unwrap();
    let body = schema.get_field("body").unwrap();
    let query_parser = QueryParser::for_index(index, vec![title, body]);
    let query = query_parser.parse_query(query_str)?;
    let top_docs = searcher.search(&query, &TopDocs::with_limit(10))?;
    for (_, doc_address) in top_docs {
        let retrieved_doc = searcher.doc(doc_address)?;
        println!("{:?}", retrieved_doc);
    }
    Ok(())
}

fn main() -> Result<(), TantivyError> {
    println!("Hello, Shrimp!");

    // 创建索引并存储它
    let index = create_index();

    // 在创建的索引中搜索
    search_index(&index, "Example")?;

    Ok(())
}
