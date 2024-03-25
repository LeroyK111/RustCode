/*
在Rust中使用Serde的详细指南

在处理HTTP请求时，我们总是需要在一种数据结构(可以是enum、struct等)和一种可以存储或传输并稍后重建的格式(例如JSON)之间来回转换。

Serde是一个库(crate)，用于高效、通用地序列化和反序列化Rust数据结构。在本文中，将详细介绍如何使用Serde对数据结构进行序列化和反序列化操作。

让我们从一个简单的结构体Student开始，它的定义如下所示，并进行初始化。
*/

use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
struct Student {
    pub name: String, 
    pub student_id: String,
}

let student = Student{name:"tom".to_owned(), student_id:"J19990".to_owned()};

/*
约定

对于上面的示例，如果我们使用serde_json::to_string(&student)将其转换为JSON字符串，那么输出将如下所示。
{
  "name": "tom",
  "student_id": "J19990"
}

看起来太棒了！然而，这依赖于发送HTTP请求的内容，很有可能会与Rust中的数据结构有不同的大小写约定。

基本上有两种方法可以进行约定：可以重命名字段；也可以对整个结构应用大小写约定。

例如，我们实际上希望使用studentId而不是student_id作为字段名。

方法1：使用#[serde(rename="")重命名单个字段。
*/


// 方法1：使用#[serde(rename="")重命名单个字段。
struct Student {
    pub name: String, 
    #[serde(rename="studentId")
    pub student_id: String,
}
// 方法2：使用#[serde(rename_all="camelCase")将大小写约定驼峰形式，应用于整个结构体。
#[serde(rename_all = "camelCase")]
struct Student {
    pub name: String, 
    pub student_id: String,
}
// 任何一种方法都会给出以下输出：
// {
//   "name": "tom",
//   "studentId": "J19990"
// }
// 除了camelCase之外，还可以应用其他的约定。取值为“lowercase, UPPERCASE, PascalCase, camelCase, snake_case, SCREAMING_SNAKE_CASE, kebab-case, SCREAMING-KEBAB-CASE”。



// Skip

// Skip可用于不希望序列化或反序列化的字段。下面是一个简单的例子。让我们给Student添加birth_year和age。
struct Student {
    pub name: String, 
    pub student_id: String,
    pub birth_year: u32,
    pub age: u32,
}
// 我们可能希望动态更新年龄，因此需要对学生birth_year的引用。但是，当我们发送请求时，应该只显示age字段，这可以使用#[serde(skip)]来解决。
struct Student {
    pub name: String, 
    pub student_id: String,
    #[serde(skip)]
    pub birth_year: u32,
    pub age: u32,
}
// 通过这样做，我们的JSON对象将变成：
// {
//   "name": "tom",
//   "studentId": "J19990",
//   "age": 123
// }


// Skip If

// 最常见的两种使用方法是作用于Option字段和空的vector字段。

// Option

// 假设我们有一个middle_name: Option<String>字段，如果我们想在学生没有这个字段的情况下跳过这个字段，我们可以这样做。
struct Student {
    pub name: String, 
    pub student_id: String,
    #[serde(skip_serializing_if="Option::is_none")]
    pub middle_name: Option<String>
}
// 这将为带有或不带有中间名的学生生成如下JSON：
// // 没有中间名
// {
//   "name": "tom",
//   "studentId": "J19990",
// }

// // 有中间名
// {
//   "name": "tom",
//   "studentId": "J19990",
//   "middleName": "middle"
// }
// Vector

// 例如，我们为student结构体提供了pets: Vec<String>字段。由于学生不必拥有宠物，它可以是一个空向量。

// 要跳过对空向量的序列化，可以向字段添加以下属性。
#[serde(skip_serializing_if="Vec::is_empty")]
pub pets: Vec<String>,
// 有属性和没有属性之间的输出差异如下所示。
// 没有属性
{
  "name": "tom",
  "studentId": "J19990",
  "pets": []
}

// 有属性
{
  "name": "tom",
  "studentId": "J19990"
}


Flatten

// 让我们创建一个名为SideInfo的新结构体，并将Student结构体更改为以下内容。
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct Student {
    pub name: String, 
    pub student_id: String,
    #[serde(skip_serializing_if="Option::is_none")]
    pub side_info: Option<SideInfo>
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
struct SideInfo {
    #[serde(skip_serializing_if="Option::is_none")]
    pub pets: Option<Vec<String>>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub address: Option<String>,
}
// 让我们创建一个新的Student
let student = Student{name:"dan".to_owned(), student_id: "1".to_owned(), side_info:Some(SideInfo{address:Some("47 street".to_owned()), ..Default::default()})};
// 并输出它的JSON字符串：
// {
//   "name": "dan",
//   "studentId": "1",
//   "sideInfo": {
//     "address": "47 street"
//   }
// }
// 如你所见，地址字段嵌套在sideInfo中。我们可以通过将属性flatten添加到Student结构体中的sideInfo字段上，来消除嵌套。
#[serde(skip_serializing_if="Option::is_none", flatten)]
pub side_info: Option<SideInfo>
// 就会变成：
// {
//   "name": "dan",
//   "studentId": "1",
//   "address": "47 street"
// }


// 枚举上的标签与非标签

// 假设我们有一个StudentList enum，如下所示：
enum StudentList {
    Student1(Student), 
    Student2(Student)
}
// 定义学生名单
let student1 = Student{name:"tom".to_owned(), student_id:"J19990".to_owned(), pets: vec![], middle_name:Some("middle".to_owned())};
let student2 = Student{name:"dan".to_owned(), student_id:"J19990".to_owned(), pets: vec![], middle_name:Some("middle".to_owned())};

let student_list = vec![StudentList::Student1(student1), StudentList::Student2(student2)];
// 如果我们像现在一样打印出JSON，它将如下所示，它是有标签的，是serde的默认行为。
[
  {
    "Student1": {
      "name": "tom",
      "studentId": "J19990",
      "pets": [],
      "middleName": "middle"
    }
  },
  {
    "Student2": {
      "name": "dan",
      "studentId": "J19990",
      "pets": [],
      "middleName": "middle"
    }
  }
]
// 如果你希望所有标签都具有相同的名称，例如Student，该怎么办呢？你可能认为可以使用rename_all来实现这一点，但实际上并非如此，应该手动重命名枚举中的每个变体。
#[derive(Debug, Clone, Serialize, Deserialize)]
enum StudentList {
    #[serde(rename="Student")]
    Student1(Student), 
    #[serde(rename="Student")]
    Student2(Student)
}
输出如下：
[
  {
    "Student": {
      "name": "tom",
      "studentId": "J19990",
      "pets": [],
      "middleName": "middle"
    }
  },
  {
    "Student": {
      "name": "dan",
      "studentId": "J19990",
      "pets": [],
      "middleName": "middle"
    }
  }
]
// 不加标签

// 如果我们只想要一个简单的学生数组，而不显示枚举变量名称，该怎么办？我们可以通过向枚举中添加#[serde(untagged)]属性来实现这一点。通过这样做，我们的输出将变成：
[
  {
    "name": "tom",
    "studentId": "J19990",
    "pets": [],
    "middleName": "middle"
  },
  {
    "name": "dan",
    "studentId": "J19990",
    "pets": [],
    "middleName": "middle"
  }
]
// 内部标签

// 枚举的另一种表示形式是内部标签，让我们创建一个包含不同学生类型的新枚举，我们将有班长、副班长和普通学生。
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", rename_all="camelCase")]
enum StudentType {
    Regular(Student), 
    Leader(Student), 
    SubLeader(Student)
}
// 指定serde(tag = "type")将允许我们在内容中使用标签来识别我们正在处理的变体，如下所示：
// [
//   {
//     "type": "leader",
//     "name": "tom",
//     "studentId": "J19990",
//     "pets": [],
//     "middleName": "middle"
//   },
//   {
//     "type": "regular",
//     "name": "dan",
//     "studentId": "J19990",
//     "pets": [],
//     "middleName": "middle"
//   }
// ]
// 相邻标签

// 表示标签和内容作为同一对象中的两个字段彼此相邻。将枚举的属性修改如下：
// #[serde(tag = "type", content = "student", rename_all="camelCase")]
// json数据变成：
// [
//   {
//     "type": "leader",
//     "student": {
//       "name": "tom",
//       "studentId": "J19990",
//       "pets": [],
//       "middleName": "middle"
//     }
//   },
//   {
//     "type": "regular",
//     "student": {
//       "name": "dan",
//       "studentId": "J19990",
//       "pets": [],
//       "middleName": "middle"
//     }
//   }
// ]