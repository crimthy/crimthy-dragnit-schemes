extern crate dragnit;
use dragnit::*;

pub fn build_schemes() -> Vec<Schema> {
  
  let point_schema: Schema = Schema::new("point_schema".to_owned(), vec![
    Def::new("Point".to_owned(), DefKind::Struct, vec![
      Field {name: "x".to_owned(), type_id: TYPE_FLOAT, is_array: false, value: 0},
      Field {name: "y".to_owned(), type_id: TYPE_FLOAT, is_array: false, value: 0},
    ]),
  ]);

  vec![
    point_schema
  ]
}



