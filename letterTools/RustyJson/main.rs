// Objext
let mut json_obj = JsonObject::new();
json_obj.set("Name", "Ammar Dev");
json_obj.set("msg", "Github");
json_obj.set("number", 234);

println!("{}", json_obj); // {"msg": "Github", "Name": "Ammar Dev", "number": 234}

// Array
let mut json_arr = JsonArray::new();
json_arr.push("Value 1");
json_arr.push(324);
json_arr.push(vec![1, 2]);

println!("{}", json_arr);// ["Value 1", 324, [1, 2]]