/// Stolen! Gasp. I know. Was not having luck with this one.
/// https://gitlab.com/mbryant/aoc-2021/-/blob/main/src/lib.rs#L1853
type Ranges = ((isize, isize), (isize, isize));

fn parse_input(input: &str) -> Ranges {
    let mut ranges = input["target area: ".len()..]
        .trim_end()
        .split(", ")
        .map(|range| {
            range[2..]
                .split_once("..")
                .map(|(l, r)| {
                    (
                        l.parse().expect("Ranges should be composed of integers"),
                        r.parse().expect("Ranges should be composed of integers"),
                    )
                })
                .expect("Ranges should be separate")
        });
    (
        ranges.next().expect("Must be a x range"),
        ranges.next().expect("Must be a y range"),
    )
}

/// Probe trajectory is a parabola, so it'll always pass back through 0 with `-initial`
/// velocity.  Assume we're hitting the lowest part of our target in a single step from 0,
/// then figure out how high it must've been.
/// The trajectory will be `y + (y-1) + ... = \sum_1^y y`, or `y * (y+1) / 2`.
///
/// TIL: Rust argument destructuring is really cool
fn part1(&(_, (by, _)): &Ranges) -> isize {
    (by * (by + 1)) / 2
}

fn part2(&((left_x, right_x), (bottom_y, top_y)): &Ranges) -> usize {
    assert!(top_y < 0, "Assume targets are lower than us");

    let min_x = (1..)
        .find(|candidate_x| candidate_x * (candidate_x + 1) >= left_x * 2)
        .expect("Must be a minimum x-velocity");

    (min_x..=right_x)
        .flat_map(|xv| {
            (bottom_y..=0)
                .filter(move |&yv| {
                    let (mut xv, mut yv) = (xv, yv);
                    let (mut x, mut y) = (xv, yv);

                    loop {
                        if y < bottom_y || x > right_x {
                            break false;
                        } else if y <= top_y && x >= left_x {
                            // We made it.
                            break true;
                        }

                        xv = std::cmp::max(0, xv - 1);
                        yv -= 1;
                        x += xv;
                        y += yv;
                    }
                })
                .chain((1..bottom_y.abs()).filter(move |&yv| {
                    // We know our minimum steps are 2y+1, since that'll bring our
                    // y back to 0.
                    let min_steps = 2 * yv + 1;

                    let (mut y, mut yv) = (0, -yv);
                    let (mut x, mut xv) = if xv > min_steps {
                        // We're still stepping along
                        (
                            xv * min_steps - (min_steps * (min_steps - 1) / 2),
                            xv - (min_steps - 1),
                        )
                    } else {
                        // We'll be done stepping at this point.
                        (xv * (xv + 1) / 2, 0)
                    };

                    loop {
                        if y < bottom_y || x > right_x {
                            break false;
                        } else if y <= top_y && x >= left_x {
                            // We made it.
                            break true;
                        }

                        xv = std::cmp::max(0, xv - 1);
                        yv -= 1;
                        x += xv;
                        y += yv;
                    }
                }))
        })
        .count()
}

pub fn run(example: bool) {
    let input = if example {
        "target area: x=20..30, y=-10..-5"
    } else {
        "target area: x=169..206, y=-108..-68"
    };
    let ranges = parse_input(input);
    // Part 1
    let pt1_result = part1(&ranges);
    println!("Part 1 - highest point: {pt1_result}");

    let pt2_result = part2(&ranges);
    println!("Part 2 - number of possible initial\n velocities that hit target area: {pt2_result}");
}
