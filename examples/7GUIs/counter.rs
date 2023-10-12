use vizia::prelude::*;

#[derive(Lens)]
pub struct AppData {
    count: i32,
}

pub enum AppEvent {
    Increment,
}

impl Model for AppData {
    fn event(&mut self, _: &mut EventContext, event: &mut Event) {
        event.map(|app_event, _| match app_event {
            AppEvent::Increment => self.count += 1,
        });
    }
}

fn main() {
    Application::new(|| {
        AppData { count: 0 }.build();

        HStack::new(|| {
            Button::new(|cx| cx.emit(AppEvent::Increment), || Label::new("Increment"));
            Label::new(AppData::count).width(Pixels(50.0));
        })
        .child_space(Stretch(1.0))
        .col_between(Pixels(50.0));
    })
    .title("Counter")
    .inner_size((400, 100))
    .run();
}
