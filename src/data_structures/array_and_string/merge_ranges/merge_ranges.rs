/*
 * Merge meetings ranges
 * https://www.interviewcake.com/question/javascript/merging-ranges
 * Merge meetings ranges
 */

#[cfg(test)]
mod tests {
    use super::*;

    // #[test]
    // fn meetings_overlap() {
    // let ranges = vec![
    // Meeting {
    // start_time: 1,
    // end_time: 3,
    // },
    // Meeting {
    // start_time: 2,
    // end_time: 4,
    // },
    // ];
    // let actual = merge_ranges(&ranges);
    // let expected = vec![Meeting {
    // start_time: 1,
    // end_time: 4,
    // }];
    // assert_eq!(&actual, &expected);
    // }

    // #[test]
    // fn meetings_touch() {
    // let ranges = vec![
    // Meeting {
    // start_time: 5,
    // end_time: 6,
    // },
    // Meeting {
    // start_time: 6,
    // end_time: 8,
    // },
    // ];
    // let actual = merge_ranges(&ranges);
    // let expected = vec![Meeting {
    // start_time: 5,
    // end_time: 8,
    // }];
    // assert_eq!(actual, expected);
    // }

    // #[test]
    // fn meeting_contains_other_meeting() {
    // let ranges = vec![
    // Meeting {
    // start_time: 1,
    // end_time: 8,
    // },
    // Meeting {
    // start_time: 2,
    // end_time: 5,
    // },
    // ];
    // let actual = merge_ranges(&ranges);
    // let expected = [Meeting {
    // start_time: 1,
    // end_time: 8,
    // }];
    // assert_eq!(actual, expected);
    // }

    // #[test]
    // fn meetings_stay_separate() {
    // let ranges = vec![
    // Meeting {
    // start_time: 1,
    // end_time: 3,
    // },
    // Meeting {
    // start_time: 4,
    // end_time: 8,
    // },
    // ];
    // let actual = merge_ranges(&ranges);
    // let expected = vec![
    // Meeting {
    // start_time: 1,
    // end_time: 3,
    // },
    // Meeting {
    // start_time: 4,
    // end_time: 8,
    // },
    // ];
    // assert_eq!(actual, expected);
    // }

    // #[test]
    // fn multiple_merged_meetings() {
    // let ranges = vec![
    // Meeting {
    // start_time: 1,
    // end_time: 4,
    // },
    // Meeting {
    // start_time: 2,
    // end_time: 5,
    // },
    // Meeting {
    // start_time: 5,
    // end_time: 8,
    // },
    // ];
    // let actual = merge_ranges(&ranges);
    // let expected = [Meeting {
    // start_time: 1,
    // end_time: 8,
    // }];
    // assert_eq!(actual, expected);
    // }

    // #[test]
    // fn meetings_not_sorted() {
    // let ranges = vec![
    // Meeting {
    // start_time: 5,
    // end_time: 8,
    // },
    // Meeting {
    // start_time: 1,
    // end_time: 4,
    // },
    // Meeting {
    // start_time: 6,
    // end_time: 8,
    // },
    // ];
    // let actual = merge_ranges(&ranges);
    // let expected = vec![
    // Meeting {
    // start_time: 1,
    // end_time: 4,
    // },
    // Meeting {
    // start_time: 5,
    // end_time: 8,
    // },
    // ];
    // assert_eq!(actual, expected);
    // }

    // #[test]
    // fn one_long_meeting_contains_smaller_meetings() {
    // let ranges = vec![
    // Meeting {
    // start_time: 1,
    // end_time: 10,
    // },
    // Meeting {
    // start_time: 2,
    // end_time: 5,
    // },
    // Meeting {
    // start_time: 6,
    // end_time: 8,
    // },
    // Meeting {
    // start_time: 9,
    // end_time: 10,
    // },
    // Meeting {
    // start_time: 10,
    // end_time: 12,
    // },
    // ];
    // let actual = merge_ranges(&ranges);
    // let expected = vec![Meeting {
    // start_time: 1,
    // end_time: 12,
    // }];
    // assert_eq!(actual, expected);
    // }

    // #[test]
    // fn sample_input() {
    // let ranges = vec![
    // Meeting {
    // start_time: 0,
    // end_time: 1,
    // },
    // Meeting {
    // start_time: 3,
    // end_time: 5,
    // },
    // Meeting {
    // start_time: 4,
    // end_time: 8,
    // },
    // Meeting {
    // start_time: 10,
    // end_time: 12,
    // },
    // Meeting {
    // start_time: 9,
    // end_time: 10,
    // },
    // ];
    // let actual = merge_ranges(&ranges);
    // let expected = vec![
    // Meeting {
    // start_time: 0,
    // end_time: 1,
    // },
    // Meeting {
    // start_time: 3,
    // end_time: 8,
    // },
    // Meeting {
    // start_time: 9,
    // end_time: 12,
    // },
    // ];
    // assert_eq!(actual, expected);
    // }
}
