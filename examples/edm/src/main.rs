use odata_sql_helpers::reflect::into_entity_type;
use quick_xml::se::to_string;
use sea_orm::ModelTrait;
use test_model::Model;

mod test_model;

fn main() {
    let et = into_entity_type::<<Model as ModelTrait>::Entity>();
    let xml = to_string(&et).expect("invalid XML");
    println!("{}", xml);
}
