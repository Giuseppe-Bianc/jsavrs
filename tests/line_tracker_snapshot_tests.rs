use jsavrs::location::line_tracker::LineTracker;
use std::sync::Arc;
use insta::assert_snapshot;

/*#[test]
fn snapshot_all_offsets_locations() {
    let source = "a\nbc\ndef\n";
    let tracker = LineTracker::new("snapshot.txt", source.to_string());

    let locations: Vec<_> = (0..=source.len())
        .map(|offset| {
            let loc = tracker.location_for(offset);
            (
                offset,
                format!("line {:?}, column {:?}, pos {:?}", loc.line, loc.column, loc.absolute_pos),
            )
        })
        .collect();
    

    assert_snapshot!(locations);
}*/

#[test]
fn snapshot_full_span_basic() {
    let source = "x\ny\nz";
    let tracker = LineTracker::new("snapshot.txt", source.to_string());

    let span = tracker.span_for(0..5);
    assert_snapshot!("span_0_to_5", span);
}

#[test]
fn snapshot_multibyte_span() {
    let source = "αβ\nγδ";
    let tracker = LineTracker::new("snapshot.txt", source.to_string());

    let span = tracker.span_for(0..6);
    assert_snapshot!("span_multibyte", span);
}
