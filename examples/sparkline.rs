use wagashi::SparkLine;

fn main() {
    let sparkline = SparkLine::new(&[1.0, 2.0, 9.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 10.0]);
    sparkline.render();
}
