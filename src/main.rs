use svg::node;
use svg::node::element::path::Data;
use svg::node::element::{Circle, Path, Rectangle, Text};
use svg::Document;

fn main() {
    let sqsize = 10;

    let mut paths = vec![];
    let mut coords = vec![];
    let letters = "ABCDEFGH";

    for i in 0..=8 {
        let data = Data::new()
            .move_to((0, i * sqsize))
            .line_by((8 * sqsize, 0))
            .move_to((i * sqsize, 0))
            .line_by((0, 8 * sqsize));
        let path = Path::new()
            .set("fill", "none")
            .set("stroke", "black")
            .set("stroke-width", 1)
            .set("d", data);
        paths.push(path);
    }

    for i in 0..8 {
        let letter = (letters.as_bytes()[i] as char).to_string();
        let data = node::Text::new(letter);
        let text = Text::new()
            .add(data)
            .set("x", i * sqsize + sqsize / 3)
            .set("y", -2);
        coords.push(text.set("font-size", "5px").set("lengthAdjust", "spacing"));
        let data = node::Text::new((i + 1).to_string());
        let text = Text::new()
            .add(data)
            .set("x", -5)
            .set("y", i * sqsize + 2 * sqsize / 3);
        coords.push(text.set("font-size", "5px").set("lengthAdjust", "spacing"));
    }

    let mut rects = vec![];
    let mut circles = vec![];
    let whites = vec![(3, 3), (4, 4)];
    let blacks = vec![(4, 3), (3, 4)];

    for (i, j) in (0..64).map(|i| (i / 8, i % 8)) {
        let rect = Rectangle::new()
            .set("x", sqsize * i)
            .set("y", sqsize * j)
            .set("width", sqsize)
            .set("height", sqsize)
            .set(
                "fill",
                if (i + j) % 2 == 0 {
                    "#3a911a"
                } else {
                    "#4BA30B"
                },
            );
        rects.push(rect);

        if let Some(_) = whites.iter().find(|(x, y)| (x, y) == (&i, &j)) {
            let circle = Circle::new()
                .set("cx", sqsize * i + sqsize / 2)
                .set("cy", sqsize * j + sqsize / 2)
                .set("fill", "white")
                .set("r", sqsize / 3);
            circles.push(circle);
        };
        if let Some(_) = blacks.iter().find(|(x, y)| (x, y) == (&i, &j)) {
            let circle = Circle::new()
                .set("cx", sqsize * i + sqsize / 2)
                .set("cy", sqsize * j + sqsize / 2)
                .set("fill", "black")
                .set("r", sqsize / 3);
            circles.push(circle);
        }
    }

    let mut document = Document::new().set("viewBox", (0, -7, 80, 87));

    for rect in rects {
        document = document.add(rect);
    }
    for path in paths {
        document = document.add(path);
    }
    for circle in circles {
        document = document.add(circle);
    }
    for coord in coords {
        document = document.add(coord);
    }

    svg::save("initial_othello.svg", &document).unwrap();
}
