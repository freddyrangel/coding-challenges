/*
 * FIND RECTANGLE OVERLAP
 * https://www.interviewcake.com/question/javascript/rectangular-love
 * Calculate the overlap between the two rectangles
 */

use std::cmp;

#[derive(Debug, PartialEq)]
struct Rectangle {
    left_x: usize,
    bottom_y: usize,
    width: usize,
    height: usize,
}

struct RangeOverlap {
    start_point: usize,
    overlap_length: usize,
}

fn find_range_overlap(
    point1: usize,
    length1: usize,
    point2: usize,
    length2: usize,
) -> Option<RangeOverlap> {
    // Find the highest start point and lowest end point.
    // The highest ("rightmost" or "upmost") start point is
    // the start point of the overlap.
    // The lowest end point is the end point of the overlap.
    let highest_start_point = cmp::max(point1, point2);
    let lowest_end_point = cmp::min(point1 + length1, point2 + length2);

    // Return None overlap if there is no overlap
    if highest_start_point >= lowest_end_point {
        None
    } else {
        // Compute the overlap length
        let overlap_length = lowest_end_point - highest_start_point;

        Some(RangeOverlap {
            start_point: highest_start_point,
            overlap_length,
        })
    }
}

fn find_rectangular_overlap(rect1: Rectangle, rect2: Rectangle) -> Option<Rectangle> {
    // Get the x and y overlap points and lengths
    let x_overlap = find_range_overlap(rect1.left_x, rect1.width, rect2.left_x, rect2.width);
    let y_overlap = find_range_overlap(rect1.bottom_y, rect1.height, rect2.bottom_y, rect2.height);

    match x_overlap.zip(y_overlap) {
        Some((x, y)) => Some(Rectangle {
            left_x: x.start_point,
            bottom_y: y.start_point,
            width: x.overlap_length,
            height: y.overlap_length,
        }),

        // Return null rectangle if there is no overlap
        None => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn overlap_along_both_axes() {
        let rect1 = Rectangle {
            left_x: 1,
            bottom_y: 1,
            width: 6,
            height: 3,
        };
        let rect2 = Rectangle {
            left_x: 5,
            bottom_y: 2,
            width: 3,
            height: 6,
        };
        let actual = find_rectangular_overlap(rect1, rect2);
        let expected = Rectangle {
            left_x: 5,
            bottom_y: 2,
            width: 2,
            height: 2,
        };

        assert_eq!(actual, Some(expected));
    }

    #[test]
    fn one_rectangle_inside_another() {
        let rect1 = Rectangle {
            left_x: 1,
            bottom_y: 1,
            width: 6,
            height: 6,
        };
        let rect2 = Rectangle {
            left_x: 3,
            bottom_y: 3,
            width: 2,
            height: 2,
        };
        let actual = find_rectangular_overlap(rect1, rect2);
        let expected = Rectangle {
            left_x: 3,
            bottom_y: 3,
            width: 2,
            height: 2,
        };
        assert_eq!(actual, Some(expected));
    }

    #[test]
    fn both_rectangles_the_same() {
        let rect1 = Rectangle {
            left_x: 2,
            bottom_y: 2,
            width: 4,
            height: 4,
        };
        let rect2 = Rectangle {
            left_x: 2,
            bottom_y: 2,
            width: 4,
            height: 4,
        };
        let actual = find_rectangular_overlap(rect1, rect2);
        let expected = Rectangle {
            left_x: 2,
            bottom_y: 2,
            width: 4,
            height: 4,
        };
        assert_eq!(actual, Some(expected));
    }

    #[test]
    fn touch_on_horizontal_edge() {
        let rect1 = Rectangle {
            left_x: 1,
            bottom_y: 2,
            width: 3,
            height: 4,
        };
        let rect2 = Rectangle {
            left_x: 2,
            bottom_y: 6,
            width: 2,
            height: 2,
        };
        let actual = find_rectangular_overlap(rect1, rect2);
        assert_eq!(actual, None);
    }

    #[test]
    fn touch_on_vertical_edge() {
        let rect1 = Rectangle {
            left_x: 1,
            bottom_y: 2,
            width: 3,
            height: 4,
        };
        let rect2 = Rectangle {
            left_x: 4,
            bottom_y: 3,
            width: 2,
            height: 2,
        };
        let actual = find_rectangular_overlap(rect1, rect2);
        assert_eq!(actual, None);
    }

    #[test]
    fn touch_at_a_corner() {
        let rect1 = Rectangle {
            left_x: 1,
            bottom_y: 1,
            width: 2,
            height: 2,
        };
        let rect2 = Rectangle {
            left_x: 3,
            bottom_y: 3,
            width: 2,
            height: 2,
        };
        let actual = find_rectangular_overlap(rect1, rect2);
        assert_eq!(actual, None);
    }

    #[test]
    fn no_overlap() {
        let rect1 = Rectangle {
            left_x: 1,
            bottom_y: 1,
            width: 2,
            height: 2,
        };
        let rect2 = Rectangle {
            left_x: 4,
            bottom_y: 6,
            width: 3,
            height: 6,
        };
        let actual = find_rectangular_overlap(rect1, rect2);
        assert_eq!(actual, None);
    }
}
