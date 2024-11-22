use std::any::type_name;

pub struct MyClass {
    pub field1: i32,
    pub field2: String,
}

impl MyClass {
    pub fn get_field_names() -> Vec<&'static str> {
        let type_name = type_name::<Self>();
        let mut field_names = Vec::new();

        // 这里简化了类型名称解析的逻辑，仅作为演示
        // 实际上需要处理复杂的类型名称，例如带有命名空间和生命周期的名称
        if let Some(start_index) = type_name.find("{") {
            let body = &type_name[start_index + 1..type_name.len() - 1];
            for field in body.split(",") {
                let field = field.trim();
                let name_end = field.find(':').unwrap_or_else(|| field.len());
                let field_name = &field[..name_end];
                field_names.push(field_name);
            }
        }

        field_names
    }
}

fn main() {
    let field_names = MyClass::get_field_names();
    for name in field_names {
        println!("Field name: {}", name);
    }
}
