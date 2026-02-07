use lab1::Rectangle;
use lab1::Circle;
use lab1::Triangle;
use lab1::compare_areas;

fn main() {
    let rec = Rectangle::new(10, 3);
    let cir = Circle::new(10.0);
    let tri = Triangle::new(5.0, 3.0);

    let area_rec = rec.area();
    let area_cir = cir.area();
    let area_tri = tri.area();

    let per_rec = rec.perimeter();
    let per_cir = cir.perimeter();
    let per_tri = tri.perimeter();

    compare_areas(area_rec, area_cir);
    compare_areas(area_rec, area_tri);
    compare_areas(area_tri, area_cir);
}