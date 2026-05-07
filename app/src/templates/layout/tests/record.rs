// use rstest::*;
// use rxpect::expect;
// use rxpect::expectations::EqualityExpectations;
// use crate::templates::layout::record::FileRecordTemplate;


// const VALID_RECORD: &str = r#"
// id: A
// name: DETAIL RECORD A
// required: true
// fields:
//   - account_number:
//       pos: 10..20
//       key: true
//       required: true
// "#;


// mod deserialize {
//     use super::*;

//     #[rstest]
//     fn it_deserializes_record_from_yaml(#[values(VALID_RECORD)] yaml: &str) {
//         let record = serde_yaml::from_str::<FileRecordTemplate>(yaml).unwrap();

//         expect(record.id()).to_equal("A");
//         // expect(field.name()).to_equal("account_number");
//         // expect(field.pos().start()).to_equal(10);
//         // expect(field.pos().end()).to_equal(20);
//         // expect(field.data_type()).to_equal(":str");
//         // expect(field.required()).to_be_true();
//     }
// }
