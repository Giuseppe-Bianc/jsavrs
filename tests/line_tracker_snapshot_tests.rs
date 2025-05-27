use insta::assert_snapshot;
use jsavrs::location::line_tracker::LineTracker;

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
