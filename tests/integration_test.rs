// Integration test for SigmaOS-mint-event-handler
// Verifies the crate builds and links as a dependency.
#![allow(unused_imports)]

use mint_event_handler as _;

#[test]
fn crate_links_and_runs() {
    assert_eq!(2 + 2, 4);
}
