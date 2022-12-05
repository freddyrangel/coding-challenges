// Merge meetings ranges
// https://www.interviewcake.com/question/javascript/merging-ranges

#![allow(dead_code, unused_imports)]

use std::cmp;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct Meeting {
    start_time: u32,
    end_time: u32,
}

fn merge_ranges(meetings: &Vec<Meeting>) -> Vec<Meeting> {
    let mut sorted_meetings: Vec<Meeting> = meetings.clone().to_vec();

    sorted_meetings.sort_by(|a, b| a.start_time.cmp(&b.start_time));

    let mut merged_meetings = vec![sorted_meetings[0]];

    for current_meeting in sorted_meetings {
        let last_merged_index = merged_meetings.len() - 1;
        let mut last_merged_meeting = merged_meetings[last_merged_index];

        // If the current meeting overlaps with the last merged meeting, use the
        // later end time of the two
        if current_meeting.start_time <= last_merged_meeting.end_time {
            last_merged_meeting.end_time =
                cmp::max(last_merged_meeting.end_time, current_meeting.end_time);
            merged_meetings[last_merged_index] = last_merged_meeting;
        } else {
            // Add the current meeting since it doesn't overlap
            merged_meetings.push(current_meeting);
        }
    }

    merged_meetings
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn meetings_overlap() {
        let ranges = vec![
            Meeting {
                start_time: 1,
                end_time: 3,
            },
            Meeting {
                start_time: 2,
                end_time: 4,
            },
        ];
        let actual = merge_ranges(&ranges);
        let expected = vec![Meeting {
            start_time: 1,
            end_time: 4,
        }];
        assert_eq!(&actual, &expected);
    }

    #[test]
    fn meetings_touch() {
        let ranges = vec![
            Meeting {
                start_time: 5,
                end_time: 6,
            },
            Meeting {
                start_time: 6,
                end_time: 8,
            },
        ];
        let actual = merge_ranges(&ranges);
        let expected = vec![Meeting {
            start_time: 5,
            end_time: 8,
        }];
        assert_eq!(actual, expected);
    }

    #[test]
    fn meeting_contains_other_meeting() {
        let ranges = vec![
            Meeting {
                start_time: 1,
                end_time: 8,
            },
            Meeting {
                start_time: 2,
                end_time: 5,
            },
        ];
        let actual = merge_ranges(&ranges);
        let expected = [Meeting {
            start_time: 1,
            end_time: 8,
        }];
        assert_eq!(actual, expected);
    }

    #[test]
    fn meetings_stay_separate() {
        let ranges = vec![
            Meeting {
                start_time: 1,
                end_time: 3,
            },
            Meeting {
                start_time: 4,
                end_time: 8,
            },
        ];
        let actual = merge_ranges(&ranges);
        let expected = vec![
            Meeting {
                start_time: 1,
                end_time: 3,
            },
            Meeting {
                start_time: 4,
                end_time: 8,
            },
        ];
        assert_eq!(actual, expected);
    }

    #[test]
    fn multiple_merged_meetings() {
        let ranges = vec![
            Meeting {
                start_time: 1,
                end_time: 4,
            },
            Meeting {
                start_time: 2,
                end_time: 5,
            },
            Meeting {
                start_time: 5,
                end_time: 8,
            },
        ];
        let actual = merge_ranges(&ranges);
        let expected = [Meeting {
            start_time: 1,
            end_time: 8,
        }];
        assert_eq!(actual, expected);
    }

    #[test]
    fn meetings_not_sorted() {
        let ranges = vec![
            Meeting {
                start_time: 5,
                end_time: 8,
            },
            Meeting {
                start_time: 1,
                end_time: 4,
            },
            Meeting {
                start_time: 6,
                end_time: 8,
            },
        ];
        let actual = merge_ranges(&ranges);
        let expected = vec![
            Meeting {
                start_time: 1,
                end_time: 4,
            },
            Meeting {
                start_time: 5,
                end_time: 8,
            },
        ];
        assert_eq!(actual, expected);
    }

    #[test]
    fn one_long_meeting_contains_smaller_meetings() {
        let ranges = vec![
            Meeting {
                start_time: 1,
                end_time: 10,
            },
            Meeting {
                start_time: 2,
                end_time: 5,
            },
            Meeting {
                start_time: 6,
                end_time: 8,
            },
            Meeting {
                start_time: 9,
                end_time: 10,
            },
            Meeting {
                start_time: 10,
                end_time: 12,
            },
        ];
        let actual = merge_ranges(&ranges);
        let expected = vec![Meeting {
            start_time: 1,
            end_time: 12,
        }];
        assert_eq!(actual, expected);
    }

    #[test]
    fn sample_input() {
        let ranges = vec![
            Meeting {
                start_time: 0,
                end_time: 1,
            },
            Meeting {
                start_time: 3,
                end_time: 5,
            },
            Meeting {
                start_time: 4,
                end_time: 8,
            },
            Meeting {
                start_time: 10,
                end_time: 12,
            },
            Meeting {
                start_time: 9,
                end_time: 10,
            },
        ];
        let actual = merge_ranges(&ranges);
        let expected = vec![
            Meeting {
                start_time: 0,
                end_time: 1,
            },
            Meeting {
                start_time: 3,
                end_time: 8,
            },
            Meeting {
                start_time: 9,
                end_time: 12,
            },
        ];
        assert_eq!(actual, expected);
    }
}
