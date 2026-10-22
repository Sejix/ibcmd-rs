//! The stored row of an `ExternalDataSourceTable` (object code 11).
//!
//! The row is a descriptor like every other metadata record, but its fields
//! live in their own collection (`c7075688-…`) and each field carries its type
//! as a `{"Pattern", …}` list. This module reads the row into the facts the
//! writer needs; it does not yet emit XML.

use anyhow::{Result, bail};

use super::brace::{Brace, parse_row};
use super::export::values::header;
use super::export::{atom, list};

/// Object code of a stored external-data-source table row.
pub(crate) const TABLE_RECORD_CODE: &str = "11";

/// The field collection id the platform stores.
pub(crate) const FIELD_COLLECTION: &str = "c7075688-01c1-41ba-bd1c-1023c8189079";

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct FieldRecord {
    pub uuid: String,
    pub name: String,
    /// `NameInDataSource`: the column name in the external source. It differs
    /// from the field name (`_c_id` is stored as `1c_id`).
    pub name_in_source: String,
    /// `{"N",10,0,0}`, `{"S",255,1}`, … as stored.
    pub pattern: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct TableRecord {
    pub uuid: String,
    pub name: String,
    /// The header's synonym, as stored (`<Synonym>` content).
    pub synonym: Brace,
    pub comment: String,
    /// Generated type pairs in stored order (the XML order differs).
    pub generated: Vec<(String, String)>,
    pub fields: Vec<FieldRecord>,
}

pub(crate) fn parse(text: &str) -> Result<TableRecord> {
    let row = parse_row(text.as_bytes())?;
    from_brace(&row)
}

pub(crate) fn from_brace(row: &Brace) -> Result<TableRecord> {
    let outer = list(row)?;
    if outer.len() < 3 || atom(&outer[0])? != "1" {
        bail!("external-data-source table: outer layout");
    }
    let body = list(&outer[1])?;
    if body.len() < 4 || atom(&body[0])? != TABLE_RECORD_CODE {
        bail!("external-data-source table: record code");
    }
    let identity = list(&body[1])?;
    if identity.len() != 2 || atom(&identity[0])? != "0" {
        bail!("external-data-source table: identity wrapper");
    }
    let head = header(&identity[1])?;
    // Every generated type is a pair of top-level identifiers. The stored
    // order is not the XML order: `RecordManager` sits after the table's own
    // scalar run, behind the field-map lists.
    let mut generated = Vec::new();
    let mut pending: Option<&str> = None;
    for node in body.iter().skip(2) {
        let Some(value) = node.as_atom() else {
            pending = None;
            continue;
        };
        if value.len() != 36 || value == super::brace::NIL_UUID {
            pending = None;
            continue;
        }
        match pending.take() {
            Some(first) => generated.push((first.to_string(), value.to_string())),
            None => pending = Some(value),
        }
    }
    let fields = outer
        .iter()
        .skip(3)
        .find_map(|candidate| {
            let collection = list(candidate).ok()?;
            (collection
                .first()
                .and_then(|first| first.as_atom())
                == Some(FIELD_COLLECTION))
            .then_some(collection)
        })
        .map(|collection| {
            collection
                .iter()
                .skip(2)
                .filter_map(|entry| field_record(entry).ok())
                .collect::<Vec<_>>()
        })
        .unwrap_or_default();
    Ok(TableRecord {
        uuid: head.uuid,
        name: head.name,
        synonym: head.synonym,
        comment: head.comment,
        generated,
        fields,
    })
}

fn field_record(entry: &Brace) -> Result<FieldRecord> {
    // `{{1,{27,{2,{3,<header>,{"Pattern",…}},…}},"<name>",0,0},0}`
    let outer = list(entry)?;
    let wrapped = list(&outer[0])?;
    let ordinal = list(&wrapped[1])?;
    let holder = list(&ordinal[1])?;
    let head = header(&holder[1])?;
    let pattern = list(&holder[2])?;
    if pattern.first().and_then(Brace::as_str) != Some("Pattern") {
        bail!("external-data-source table: field type is not a Pattern");
    }
    let kind = list(&pattern[1])?
        .iter()
        .map(|part| {
            part.as_atom()
                .or_else(|| part.as_str())
                .map(str::to_string)
                .ok_or_else(|| anyhow::anyhow!("external-data-source table: non-scalar type member"))
        })
        .collect::<Result<Vec<_>>>()?;
    Ok(FieldRecord {
        uuid: head.uuid,
        name: head.name,
        name_in_source: wrapped
            .get(2)
            .and_then(Brace::as_str)
            .unwrap_or_default()
            .to_string(),
        pattern: kind,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    /// The real row of
    /// `ExternalDataSource.ВнешнийИсточникДанных1.Table.oc_product_to_1c`
    /// (ServiceDesk 3.10.28, 8.5.1.1529), inflated from the `Config` table.
    const ROW: &str = include_str!("fixtures/external_data_source_table_row.txt");

    #[test]
    fn reads_the_table_identity_types_and_fields() {
        let table = parse(ROW).expect("a table row");
        assert_eq!(table.name, "oc_product_to_1c");
        assert_eq!(table.uuid, "78afc3f6-2d63-4482-a33e-fd11f668c2b6");
        assert_eq!(table.generated.len(), 8, "seven pairs plus RecordManager");
        assert_eq!(
            table.generated[0],
            (
                "59d4ebec-6e82-490e-acae-9d528e1e0245".to_string(),
                "5c8729a1-e458-4723-a52d-35c80789f296".to_string()
            ),
            "Manager comes first"
        );
        assert_eq!(
            table.generated[7],
            (
                "57f5e963-2504-4a54-8192-6dc60932dd59".to_string(),
                "537441e5-2b29-4613-978b-eb29a76ca9fc".to_string()
            ),
            "RecordManager is stored last"
        );
        assert_eq!(table.fields.len(), 2);
        assert_eq!(table.fields[0].name, "product_id");
        assert_eq!(table.fields[0].name_in_source, "product_id");
        assert_eq!(table.fields[0].pattern, vec!["N", "10", "0", "0"]);
        assert_eq!(table.fields[1].name, "_c_id");
        // The column name in the source differs from the field name.
        assert_eq!(table.fields[1].name_in_source, "1c_id");
        assert_eq!(table.fields[1].pattern, vec!["S", "255", "1"]);
        assert_eq!(table.comment, "");
    }
}
