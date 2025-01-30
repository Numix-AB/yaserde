use yaserde::{YaDeserialize, YaSerialize};

#[derive(Debug, YaSerialize, YaDeserialize, PartialEq, Clone)]
#[yaserde(rename = "complexContent", namespaces = { xs = "http://www.w3.org/2001/XMLSchema"}, prefix = "xs")]
pub struct ComplexContent {
  #[yaserde(prefix = "xs", rename = "restriction")]
  pub restriction: Option<yaserde::XmlValue>,
}

#[test]
fn test_xml_value() {
  let xml = r###"
  <xs:complexContent xmlns:xs="http://www.w3.org/2001/XMLSchema">
    <xs:restriction base="xs:anyType">
      <xs:anyAttribute namespace="##other" processContents="lax"/>
    </xs:restriction>
  </xs:complexContent>
  "###;

  let deserialized: ComplexContent = yaserde::de::from_str(xml).unwrap();

  let serialized = yaserde::ser::to_string_with_config(
    &deserialized,
    &yaserde::ser::Config {
      perform_indent: true,
      ..Default::default()
    },
  )
  .unwrap();

  let de_serialized_again: ComplexContent = yaserde::de::from_str(&serialized).unwrap();

  assert_eq!(deserialized, de_serialized_again)
}
