use crate::data::RelativeLocation;
use dioxus::prelude::*;

#[component]
pub fn LocationIndicator(location: RelativeLocation) -> Element {
    use RelativeLocation::*;
    let rotation = match location {
        Top => -90.0,
        Bottom => 90.0,
        Left | Right | Center => 0.0,
    };

    rsx! {
        path {
            d: "M4 8a.5.5 0 0 1 .5-.5h5.793L8.146 5.354a.5.5 0 1 1 .708-.708l3 3a.5.5 0 0 1 0 .708l-3 3a.5.5 0 0 1-.708-.708L10.293 8.5H4.5A.5.5 0 0 1 4 8",
            transform: format!("rotate({} 8 8)", rotation),
        }
    }
}
