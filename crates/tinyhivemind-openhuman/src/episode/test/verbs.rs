//! The verbs the retry reads out to a seat must be verbs the seat has.
//!
//! `insist` names them in a sentence the seat is meant to obey literally, so a
//! name that reaches no belt is worse here than anywhere else. These pin the
//! two halves of that: nothing unserved can be named, and a conversation is
//! offered the one verb no layer withholds.

use super::super::recording_verbs;
use crate::runner::Lane;
use tinyhivemind::Sequence;

/// **Every verb named is a verb the server serves.**
///
/// The regression this exists for: the conversation arm named `post` and `dm`,
/// both of which sit in `UNSERVED` and reach no belt on any lane. Asserting
/// against `served_specs` rather than against a second hardcoded list is the
/// point -- a list compared to a list is two things to keep in step, which is
/// how the first one drifted.
#[test]
fn no_lane_names_a_verb_the_server_does_not_serve() {
    let served: Vec<&str> = tinyhivemind_tools::served_specs()
        .map(|spec| spec.name)
        .collect();
    for lane in [Lane::Desk, Lane::Thread(Sequence(7))] {
        let verbs = recording_verbs(lane);
        assert!(
            !verbs.is_empty(),
            "{lane:?} must leave the seat something to call"
        );
        for verb in &verbs {
            assert!(
                served.contains(&verb.as_str()),
                "{lane:?} names `{verb}`, which no belt carries: served = {served:?}"
            );
        }
    }
}

/// **A conversation is offered `complete_episode`, and only that.**
///
/// Not a restatement of the arm above it. `ask` and `ask_teammates` are refused
/// inside a conversation and `broadcast` is withheld by a host on a concluding
/// turn -- which a thread answer usually is -- so `complete_episode` is the one
/// verb that survives every layer. The room's own refusal already says so:
/// "call `complete_episode`, and its message is your answer".
#[test]
fn a_conversation_is_offered_only_the_verb_that_answers_it() {
    assert_eq!(
        recording_verbs(Lane::Thread(Sequence(7))),
        vec!["complete_episode".to_owned()],
    );
}

/// The desk keeps all three: nothing withholds them there by rule, and a seat
/// on the desk really can hand work off or ask instead of finishing.
#[test]
fn the_desk_keeps_the_verbs_that_record_there() {
    let verbs = recording_verbs(Lane::Desk);
    for expected in ["broadcast", "ask", "complete_episode"] {
        assert!(
            verbs.iter().any(|verb| verb == expected),
            "the desk offers `{expected}`: {verbs:?}"
        );
    }
}
