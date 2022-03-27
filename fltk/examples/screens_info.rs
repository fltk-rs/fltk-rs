use fltk::{
    app::Screen,
    draw::{Coordinates, Rect},
};
type Coord = Coordinates<i32>;

fn main() {
    let screens = Screen::all_screens();
    println!("Number of detected screens = {}\n", screens.len());
    assert_eq![screens.len(), Screen::count() as usize];

    println!("Is scaling supported:");
    println!("- at all = {}", Screen::scaling_supported());
    println!("- separately = {}", Screen::scaling_supported_separately());

    let coord: Coord = [100, 100].into();
    let rect: Rect = [100, 100, 100, 100].into();

    // uncomment these lines to see out-of-boundaries errors:
    // let coord: Coord = [-100, 10_000].into();
    // let rect: Rect = [-100, 100, 10_000, 10_000].into();

    println!("\nScreen found:");
    println!("- at {coord:?} = {:?}", Screen::new_at(coord));
    println!("- inside {rect:?} = {:?}", Screen::new_inside(rect));

    println!("\nWork area:");
    println!("- at {coord:?} = {:?}", Screen::work_area_at(coord));
    println!("- under the mouse = {:?}", Screen::work_area_mouse());

    println!("\nXYWH:");
    println!("- at {coord:?} = {:?}", Screen::xywh_at(coord));
    println!("- inside {rect:?} = {:?}", Screen::xywh_inside(rect));
    println!("- under the mouse = {:?}", Screen::xywh_mouse());

    println!("\nFor each screen:");
    for s in screens {
        println!("+ {s:?}");
        println!("  - work area = {:?}:", s.work_area());
        println!("  - dpi = {:?}", s.dpi());
        println!("  - scale = {:?}", s.scale());
    }
}
