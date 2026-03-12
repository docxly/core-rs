#![allow(dead_code, unused_imports)]

#[path = "../../src/bin/support/archive_fixture.rs"]
mod shared_archive_fixture;

pub(crate) use shared_archive_fixture::{
    ArchiveEntry, BinaryRepresentation, FixtureResult, NormalizedEntry, archive_entries,
    archive_entry_names, detect_binary_representations, discover_fixture_dirs, hash_entries,
    normalized_archives_equivalent, normalized_entries, normalized_entries_from_archive,
    read_expected_entries, read_fixture_input, refresh_fixture_metadata,
};
