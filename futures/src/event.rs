//! Listen to runtime events.
use crate::core::event::{self, ApplicationEvent, Event};
use crate::core::window;
use crate::subscription::{self, Subscription};
use crate::MaybeSend;

/// Returns a [`Subscription`] to all the ignored runtime events.
///
/// This subscription will notify your application of any [`Event`] that was
/// not captured by any widget.
pub fn listen() -> Subscription<Event> {
    listen_with(|event, status, _window| match status {
        event::Status::Ignored => Some(event),
        event::Status::Captured => None,
    })
}

/// Creates a [`Subscription`] that listens and filters all the runtime events
/// with the provided function, producing messages accordingly.
///
/// This subscription will call the provided function for every [`Event`]
/// handled by the runtime. If the function:
///
/// - Returns `None`, the [`Event`] will be discarded.
/// - Returns `Some` message, the `Message` will be produced.
pub fn listen_with<Message>(
    f: fn(Event, event::Status, window::Id) -> Option<Message>,
) -> Subscription<Message>
where
    Message: 'static + MaybeSend,
{
    #[derive(Hash)]
    struct EventsWith;

    subscription::filter_map((EventsWith, f), move |event| match event {
        subscription::Event::Interaction {
            event: Event::Window(window::Event::RedrawRequested(_)),
            ..
        } => None,
        subscription::Event::Interaction {
            window,
            event,
            status,
        } => f(event, status, window),
        subscription::Event::Application { .. } => None,
    })
}

/// Creates a [`Subscription`] that produces a message for every runtime event,
/// including the redraw request events.
///
/// **Warning:** This [`Subscription`], if unfiltered, may produce messages in
/// an infinite loop.
pub fn listen_raw<Message>(
    f: fn(Event, event::Status, window::Id) -> Option<Message>,
) -> Subscription<Message>
where
    Message: 'static + MaybeSend,
{
    #[derive(Hash)]
    struct RawEvents;

    subscription::filter_map((RawEvents, f), move |event| match event {
        subscription::Event::Interaction {
            window,
            event,
            status,
        } => f(event, status, window),
        subscription::Event::Application {
            has_visible_windows: _,
        } => None,
    })
}

/// Creates a [`Subscription`] that produces a message for application's event. such as
///
pub fn listen_application() -> Subscription<ApplicationEvent> {
    #[derive(Hash)]
    struct AppEvents;

    subscription::filter_map(AppEvents, move |event| match event {
        subscription::Event::Interaction { .. } => None,
        subscription::Event::Application {
            has_visible_windows,
        } => Some(ApplicationEvent::ReOpen {
            has_visible_windows,
        }),
    })
}
