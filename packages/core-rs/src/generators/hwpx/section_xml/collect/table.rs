use super::super::text::flatten_inline_children;
use super::super::{CoreRsError, TABLE_WIDTH, TableBlock, TableSpec};

pub(super) fn build_table_spec(
    table: &TableBlock,
    strict_mode: bool,
    id: u64,
) -> Result<TableSpec, CoreRsError> {
    let mut rows = Vec::new();
    if !table.headers.is_empty() {
        rows.push(
            table
                .headers
                .iter()
                .map(|cell| flatten_inline_children(cell, strict_mode))
                .collect::<Result<Vec<_>, _>>()?,
        );
    }
    for row in &table.rows {
        rows.push(
            row.cells
                .iter()
                .map(|cell| flatten_inline_children(cell, strict_mode))
                .collect::<Result<Vec<_>, _>>()?,
        );
    }

    let col_count = rows.iter().map(Vec::len).max().unwrap_or(0);
    let col_widths = equal_widths(col_count);
    for row in &mut rows {
        row.resize(col_count, String::new());
    }

    Ok(TableSpec {
        id,
        rows,
        col_widths,
    })
}

fn equal_widths(count: usize) -> Vec<u32> {
    if count == 0 {
        return Vec::new();
    }

    let base = TABLE_WIDTH / count as u32;
    let remainder = TABLE_WIDTH % count as u32;
    let mut widths = vec![base; count];
    if let Some(last) = widths.last_mut() {
        *last += remainder;
    }
    widths
}
