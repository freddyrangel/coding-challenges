/*
 * FIND RECTANGLE OVERLAP
 * https://www.interviewcake.com/question/javascript/rectangular-love
 * Calculate the overlap between the two rectangles
 */

#[cfg(test)]
mod tests {
    use super::*;

    // #[test]
    // fn overlap_along_both_axes() {
    // let rect1 = Rectangle {
    // left_x: 1,
    // bottom_y: 1,
    // width: 6,
    // height: 3,
    // };
    // let rect2 = Rectangle {
    // left_x: 5,
    // bottom_y: 2,
    // width: 3,
    // height: 6,
    // };
    // let actual = find_rectangular_overlap(rect1, rect2);
    // let expected = Rectangle {
    // left_x: 5,
    // bottom_y: 2,
    // width: 2,
    // height: 2,
    // };

    // assert_eq!(actual, Some(expected));
    // }

    // #[test]
    // fn one_rectangle_inside_another() {
    // let rect1 = Rectangle {
    // left_x: 1,
    // bottom_y: 1,
    // width: 6,
    // height: 6,
    // };
    // let rect2 = Rectangle {
    // left_x: 3,
    // bottom_y: 3,
    // width: 2,
    // height: 2,
    // };
    // let actual = find_rectangular_overlap(rect1, rect2);
    // let expected = Rectangle {
    // left_x: 3,
    // bottom_y: 3,
    // width: 2,
    // height: 2,
    // };
    // assert_eq!(actual, Some(expected));
    // }

    // #[test]
    // fn both_rectangles_the_same() {
    // let rect1 = Rectangle {
    // left_x: 2,
    // bottom_y: 2,
    // width: 4,
    // height: 4,
    // };
    // let rect2 = Rectangle {
    // left_x: 2,
    // bottom_y: 2,
    // width: 4,
    // height: 4,
    // };
    // let actual = find_rectangular_overlap(rect1, rect2);
    // let expected = Rectangle {
    // left_x: 2,
    // bottom_y: 2,
    // width: 4,
    // height: 4,
    // };
    // assert_eq!(actual, Some(expected));
    // }

    // #[test]
    // fn touch_on_horizontal_edge() {
    // let rect1 = Rectangle {
    // left_x: 1,
    // bottom_y: 2,
    // width: 3,
    // height: 4,
    // };
    // let rect2 = Rectangle {
    // left_x: 2,
    // bottom_y: 6,
    // width: 2,
    // height: 2,
    // };
    // let actual = find_rectangular_overlap(rect1, rect2);
    // assert_eq!(actual, None);
    // }

    // #[test]
    // fn touch_on_vertical_edge() {
    // let rect1 = Rectangle {
    // left_x: 1,
    // bottom_y: 2,
    // width: 3,
    // height: 4,
    // };
    // let rect2 = Rectangle {
    // left_x: 4,
    // bottom_y: 3,
    // width: 2,
    // height: 2,
    // };
    // let actual = find_rectangular_overlap(rect1, rect2);
    // assert_eq!(actual, None);
    // }

    // #[test]
    // fn touch_at_a_corner() {
    // let rect1 = Rectangle {
    // left_x: 1,
    // bottom_y: 1,
    // width: 2,
    // height: 2,
    // };
    // let rect2 = Rectangle {
    // left_x: 3,
    // bottom_y: 3,
    // width: 2,
    // height: 2,
    // };
    // let actual = find_rectangular_overlap(rect1, rect2);
    // assert_eq!(actual, None);
    // }

    // #[test]
    // fn no_overlap() {
    // let rect1 = Rectangle {
    // left_x: 1,
    // bottom_y: 1,
    // width: 2,
    // height: 2,
    // };
    // let rect2 = Rectangle {
    // left_x: 4,
    // bottom_y: 6,
    // width: 3,
    // height: 6,
    // };
    // let actual = find_rectangular_overlap(rect1, rect2);
    // assert_eq!(actual, None);
    // }
}
