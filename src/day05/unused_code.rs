    // fn get_intersection(&self, other: &LineSegment) -> Option<Point> {
    //     // if !self.is_parallel_to(other) & self.intersects(other) {}
    //     let s10_x = self.p2.x - self.p1.x;
    //     let s10_y = self.p2.y - self.p1.y;
    //     let s32_x = other.p2.x - other.p1.x;
    //     let s32_y = other.p2.y - other.p1.y;

    //     let denom = s10_x * s32_y - s32_x * s10_y;
    //     let denom_is_positive = denom > 0;

    //     let s02_x = self.p1.x - other.p1.x;
    //     let s02_y = self.p1.y - other.p1.y;

    //     let s_numer = s10_x * s02_y - s10_y * s02_x;

    //     if (s_numer < 0) == denom_is_positive { return None }  // no collision

    //     let t_numer = s32_x * s02_y - s32_y * s02_x;

    //     if (t_numer <0) == denom_is_positive { return None }  // no collision

    //     if ((s_numer > denom) == denom_is_positive) || ((t_numer > denom) == denom_is_positive) { return None }  // no collision

    //     // collision detected
    //     let t = t_numer / denom;

    //     let intersection_point = Point {
    //         x: self.p1.x + (t * s10_x),
    //         y: self.p1.y + (t * s10_y)
    //     };
    //     Some(intersection_point)
    // }

    // fn is_parallel_to(&self, other: &LineSegment) -> bool {
    //     if (self.direction != Direction::Diag) & (other.direction != Direction::Diag) {
    //         self.direction != other.direction
    //     } else {
    //         self.slope() != other.slope()
    //     }
    // }

    // fn shares_point(&self, other: &LineSegment) -> bool {
    //     let mut all_pts: Vec<Point> = vec![self.start, self.end, other.start, other.end];
    //     all_pts.sort_by_key(|p| p.x);
    //     all_pts.dedup();
    //     all_pts.len() < 4
    // }


    // fn contains_pt(&self, p: Point) -> bool {
    //     (p.x <= max(self.p1.x, self.p2.x) && p.x <= min(self.p1.x, self.p2.x))
    //         && (p.y <= max(self.p1.y, self.p2.y) && p.y <= min(self.p1.y, self.p2.y))

        // min(other.p1.x, other.p2.x) <= max(self.p1.x, self.p2.x)
        //     || max(other.p1.x, other.p2.x) >= min(self.p1.x, self.p2.x)
        //         && min(other.p1.y, other.p2.y) <= max(self.p1.y, self.p2.y)
        //     || max(other.p1.y, other.p2.y) >= min(self.p1.y, self.p2.y)
    // }

    // #[rustfmt::skip]
    // fn intersects(&self, other: &LineSegment) -> bool {
    //     let dir1 = direction(self.p1, self.p2, other.p1);
    //     let dir2 = direction(self.p1, self.p2, other.p2);
    //     let dir3 = direction(other.p1, other.p2, self.p1);
    //     let dir4 = direction(other.p1, other.p2, self.p2);

    //     if dir1 != dir2 && dir3 != dir4 { return true; }
    //     if dir1 == Dir::Col && self.contains_pt(other.p1) { return true; }
    //     if dir2 == Dir::Col && self.contains_pt(other.p2) { return true; }
    //     if dir3 == Dir::Col && other.contains_pt(self.p1) { return true; }
    //     if dir4 == Dir::Col && other.contains_pt(self.p2) { return true; }

    //     false
    // }

// #[derive(PartialEq, Eq, Debug)]
// enum Dir {
//     Cw,  // Clockwise
//     Ccw, // Counter-clockwise
//     Col, // Colinear
// }

// fn direction(a: Point, b: Point, c: Point) -> Dir {
//     let val = (b.y - a.y) * (c.x - b.x) - (b.x - a.x) * (c.y - b.y);
//     if val == 0 {
//         return Dir::Col;
//     } else if val < 0 {
//         return Dir::Ccw;
//     }
//     Dir::Cw
// }