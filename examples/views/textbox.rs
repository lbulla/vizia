use vizia::prelude::*;
use vizia_core::state::StaticLens;

#[derive(Lens, Setter, Model)]
pub struct AppData {
    text: String,
}

fn main() {
    Application::new(|cx| {
        AppData { text: "This is some text\nthat spans two lines".to_string() }.build(cx);

        Textbox::new_multiline(cx, AppData::text, false)
            .on_edit(|cx, text| cx.emit(AppDataSetter::Text(text)))
            .width(Pixels(150.0))
            .height(Pixels(100.0))
            .on_build(|cx| {
                cx.emit(TextEvent::StartEdit);
            });

        // Textbox::new_multiline(
        //     cx,
        //     StaticLens::new(
        //         &"This text is editable, but will reset on blur. Good luck editing it, haha!",
        //     ),
        //     true,
        // )
        // .width(Pixels(200.0));
    })
    .title("Textbox")
    .run();
}
