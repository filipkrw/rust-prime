mod shapes;

use shapes::{circle::Circle, collisions::Collidable, rect::Rect};

fn main() {
    let rect1 = Rect::default();
    let rect2 = Rect::default();

    let circle1 = Circle {
        x: 0.0,
        y: 0.0,
        radius: 1.0,
    };
    let circle2 = Circle {
        x: 3.0,
        y: 5.0,
        radius: 2.0,
    };

    rect1.collide(&rect2);
    rect2.collide(&circle1);
    circle1.collide(&circle2);
    circle1.collide(&rect2);
}
