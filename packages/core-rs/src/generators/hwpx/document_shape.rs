use crate::models::block::Block;

pub(crate) fn max_unordered_list_depth(blocks: &[Block]) -> u8 {
    blocks
        .iter()
        .map(block_unordered_list_depth)
        .max()
        .unwrap_or(0)
}

pub(crate) fn max_ordered_list_depth(blocks: &[Block]) -> u8 {
    blocks
        .iter()
        .map(block_ordered_list_depth)
        .max()
        .unwrap_or(0)
}

pub(crate) fn max_blockquote_depth(blocks: &[Block]) -> u8 {
    blocks.iter().map(block_blockquote_depth).max().unwrap_or(0)
}

pub(crate) fn uses_legacy_quote_only_contract(blocks: &[Block]) -> bool {
    !blocks.is_empty()
        && blocks.iter().all(|block| match block {
            Block::BlockQuote(children) => {
                !children.is_empty()
                    && children
                        .iter()
                        .all(|child| matches!(child, Block::Paragraph(_)))
            }
            _ => false,
        })
}

fn block_unordered_list_depth(block: &Block) -> u8 {
    match block {
        Block::List(list) if !list.ordered => {
            let nested_depth = list
                .items
                .iter()
                .flat_map(|item| item.blocks.iter())
                .map(block_unordered_list_depth)
                .max()
                .unwrap_or(0);
            1 + nested_depth
        }
        Block::BlockQuote(blocks) => blocks
            .iter()
            .map(block_unordered_list_depth)
            .max()
            .unwrap_or(0),
        Block::Paragraph(_)
        | Block::Heading { .. }
        | Block::CodeBlock { .. }
        | Block::Table(_)
        | Block::ThematicBreak
        | Block::List(_) => 0,
    }
}

fn block_ordered_list_depth(block: &Block) -> u8 {
    match block {
        Block::List(list) if list.ordered => {
            let nested_depth = list
                .items
                .iter()
                .flat_map(|item| item.blocks.iter())
                .map(block_ordered_list_depth)
                .max()
                .unwrap_or(0);
            1 + nested_depth
        }
        Block::BlockQuote(blocks) => blocks
            .iter()
            .map(block_ordered_list_depth)
            .max()
            .unwrap_or(0),
        Block::Paragraph(_)
        | Block::Heading { .. }
        | Block::CodeBlock { .. }
        | Block::Table(_)
        | Block::ThematicBreak
        | Block::List(_) => 0,
    }
}

fn block_blockquote_depth(block: &Block) -> u8 {
    match block {
        Block::BlockQuote(blocks) => {
            1 + blocks.iter().map(block_blockquote_depth).max().unwrap_or(0)
        }
        Block::List(list) => list
            .items
            .iter()
            .flat_map(|item| item.blocks.iter())
            .map(block_blockquote_depth)
            .max()
            .unwrap_or(0),
        Block::Paragraph(_)
        | Block::Heading { .. }
        | Block::CodeBlock { .. }
        | Block::Table(_)
        | Block::ThematicBreak => 0,
    }
}
