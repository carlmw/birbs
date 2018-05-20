use ntree::{Region};
use cgmath::{Vector2};
use boid::Boid;

#[derive(Clone, Debug, PartialEq)]
pub struct QuadTreeRegion {
    pub x: f64,
    pub y: f64,
    pub width: f64,
    pub height: f64
}

impl QuadTreeRegion {
    pub fn square(x: f64, y: f64, wh: f64) -> QuadTreeRegion {
        QuadTreeRegion { x: x, y: y, width: wh, height: wh }
    }
}

fn contains_point(region: &QuadTreeRegion, point: Vector2<f64>) -> bool {
    region.x <= point.x &&
    region.y <= point.y &&
    (region.x + region.width) >= point.x &&
    (region.y + region.height) >= point.y
}

impl Region<Boid> for QuadTreeRegion {
    fn contains(&self, boid: &Boid) -> bool {
        self.x <= boid.position.x &&
        self.y <= boid.position.y &&
        (self.x + self.width) >= boid.position.x &&
        (self.y + self.height) >= boid.position.y
    }

    fn split(&self) -> Vec<QuadTreeRegion> {
        let halfwidth = self.width / 2.0;
        let halfheight = self.height / 2.0;
        vec![
            QuadTreeRegion {
                x: self.x,
                y: self.y,
                width: halfwidth,
                height: halfheight
            },

            QuadTreeRegion {
                x: self.x,
                y: self.y + halfheight,
                width: halfwidth,
                height: halfheight
            },

            QuadTreeRegion {
                x: self.x + halfwidth,
                y: self.y,
                width: halfwidth,
                height: halfheight
            },

            QuadTreeRegion {
                x: self.x + halfwidth,
                y: self.y + halfheight,
                width: halfwidth,
                height: halfheight
            }
        ]
    }

    fn overlaps(&self, other: &QuadTreeRegion) -> bool {
        contains_point(other, Vector2 { x: self.x, y: self.y })
            || contains_point(other, Vector2 { x: self.x + self.width, y: self.y })
            || contains_point(other, Vector2 { x: self.x, y: self.y + self.height })
            || contains_point(other, Vector2 { x: self.x + self.width, y: self.y + self.height })
    }
}

#[cfg(test)]
mod tests {
    use ntree::{NTree};
    use super::*;

    #[test]
    fn test_contains() {
        let ntree = NTree::new(QuadTreeRegion::square(0.0, 0.0, 100.0), 4);
        assert!(ntree.contains(&make_boid(50.0, 50.0)));
    }

    #[test]
    fn test_insert() {
        let mut ntree = NTree::new(QuadTreeRegion::square(0.0, 0.0, 100.0), 4);
        assert!(ntree.insert(make_boid(50.0, 50.0)));
        assert_eq!(ntree.nearby(&make_boid(40.0, 40.0)), Some(&[make_boid(50.0, 50.0)] as &[_]));
    }

    #[test]
    fn test_nearby() {
        let mut ntree = NTree::new(QuadTreeRegion::square(0.0, 0.0, 100.0), 4);

        // Bottom left corner
        ntree.insert(make_boid(30.0, 30.0));
        ntree.insert(make_boid(20.0, 20.0));
        ntree.insert(make_boid(10.0, 10.0));

        // Top right corner
        ntree.insert(make_boid(75.0, 75.0));

        // Top left corner
        ntree.insert(make_boid(40.0, 70.0));

        // Bottom right corner
        ntree.insert(make_boid(80.0, 20.0));

        // Bottom left corner
        assert_eq!(
            ntree.nearby(&make_boid(40.0, 40.0)),
            Some(&[
                make_boid(30.0, 30.0),
                make_boid(20.0, 20.0),
                make_boid(10.0, 10.0)
            ] as &[_])
        );

        // Top right corner
        assert_eq!(
            ntree.nearby(&make_boid(90.0, 90.0)),
            Some(&[make_boid(75.0, 75.0)] as &[_])
        );

        // Top left corner
        assert_eq!(
            ntree.nearby(&make_boid(20.0, 80.0)),
            Some(&[make_boid(40.0, 70.0)] as &[_])
        );

        // Bottom right corner
        assert_eq!(
            ntree.nearby(&make_boid(94.0, 12.0)),
            Some(&[make_boid(80.0, 20.0)] as &[_])
        );
    }

    #[test]
    fn test_range_query() {
        let mut ntree = NTree::new(QuadTreeRegion::square(0.0, 0.0, 100.0), 4);

        // Inside (y < 40)
        ntree.insert(make_boid(30.0, 30.0));
        ntree.insert(make_boid(20.0, 20.0));
        ntree.insert(make_boid(10.0, 10.0));
        ntree.insert(make_boid(60.0, 20.0));

        // Outside (y > 40)
        ntree.insert(make_boid(60.0, 59.0));
        ntree.insert(make_boid(60.0, 45.0));

        let result = ntree
        .range_query(&QuadTreeRegion { x: 0.0, y: 0.0, width: 100.0, height: 40.0 })
        .map(|x| x.clone())
        .collect::<Vec<Boid>>();

        assert_eq!(
            result,
            vec![
                make_boid(30.0, 30.0),
                make_boid(20.0, 20.0),
                make_boid(10.0, 10.0),
                make_boid(60.0, 20.0)
            ]
        );
    }


    #[test]
    fn test_overlaps() {
        assert!(QuadTreeRegion::square(0.0, 0.0, 100.0).overlaps(&QuadTreeRegion::square(50.0, 50.0, 100.0)));
    }

    #[test]
    fn test_split() {
        let fifty = 100.0 / 2.0;
        assert_eq!(QuadTreeRegion::square(0.0, 0.0, 100.0).split(),
            vec![
                QuadTreeRegion {
                    x: 0.0,
                    y: 0.0,
                    width: fifty,
                    height: fifty
                },

                QuadTreeRegion {
                    x: 0.0,
                    y: 0.0 + fifty,
                    width: fifty,
                    height: fifty
                },

                QuadTreeRegion {
                    x: 0.0 + fifty,
                    y: 0.0,
                    width: fifty,
                    height: fifty
                },

                QuadTreeRegion {
                    x: 0.0 + fifty,
                    y: 0.0 + fifty,
                    width: fifty,
                    height: fifty
                }
            ]
        )
    }

    fn make_boid(x: f64, y: f64) -> Boid {
        Boid { position: Vector2 { x: x, y: y }, velocity: Vector2 { x: 0.0, y: 0.0 }, color: "#ff0000".to_string() }
    }
}
