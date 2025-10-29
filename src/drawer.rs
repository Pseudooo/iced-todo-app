use iced::advanced::Widget;
use iced::widget::{button, container, Space, Stack};
use iced::{Border, Color, Element, Fill, Length, Padding, Point, Rectangle, Size, Theme, Vector};
use iced_anim::{animation_builder, Motion};

pub fn drawer<'a, Message>(
    is_open: bool,
    toggle_drawer_message: Message,
    drawer_content: impl Fn() -> Element<'a, Message> + 'a,
    window_size: Size,
    content: impl Into<Element<'a, Message>>
) -> Element<'a, Message>
where Message: Clone + 'a
{
    const PADDING: f32 = 8.0;
    const MAX_HEIGHT: f32 = 350.0;

    let height = match is_open {
        true => MAX_HEIGHT,
        false => 0.0,
    };

    let background = match is_open {
        true => Color::from_rgba(0.0, 0.0, 0.0, 0.75),
        false => Color::TRANSPARENT,
    };

    let motion = Motion::SNAPPY;

    let drawer_stack = Stack::new()
        .width(Fill)
        .height(Fill)
        .push(content)
        .push(
            // Underlay
            animation_builder((background, height), move |(background, height)| {
                container(
                    button(container(Space::new(Fill, Fill))
                        .center(Fill))
                        .on_press_maybe(is_open.then_some(toggle_drawer_message.clone()))
                        .style(move |_, _| button::Style {
                            background: Some(background.into()),
                            ..Default::default()
                        }),
                )
                .padding(Padding::new(0.0).top(height + PADDING))
                .into()
            })
            .animation(motion)
            .animates_layout(true),
        )
        .push(
            // Drawer content
            animation_builder((background, height), move |(background, height)| {
                let offset_y = window_size.height - height - PADDING * height / MAX_HEIGHT;
                Offset::new(
                    container(
                        container(drawer_content())
                            .style(move |theme: &Theme| container::Style {
                                background: Some(
                                    theme.extended_palette().background.base.color.into(),
                                ),
                                border: Border::default().rounded(8),
                                ..Default::default()
                            })
                            .padding(8)
                            .height(Fill)
                            .center_y(Length::Fixed(MAX_HEIGHT)),
                        )
                        .padding(Padding::new(PADDING).bottom(0))
                        .style(move |_| container::Style {
                            background: Some(background.into()),
                            ..Default::default()
                        }),
                    )
                    .offset(Point::new(0.0, offset_y))
                    .into()
            })
            .animates_layout(true)
            .animation(motion),
        );

    container(drawer_stack)
        .width(Fill)
        .height(Fill)
        .into()
}

struct Offset<'a, Message, Theme, Renderer>
where
    Message: 'a + Clone,
    Theme: 'a,
    Renderer: iced::advanced::Renderer,
{
    offset: Point,
    content: Element<'a, Message, Theme, Renderer>,
}

impl<'a, Message, Theme, Renderer> Offset<'a, Message, Theme, Renderer>
where
    Message: 'a + Clone,
    Theme: 'a,
    Renderer: iced::advanced::Renderer,
{
    pub fn new(content: impl Into<Element<'a, Message, Theme, Renderer>>) -> Self {
        Self {
            offset: Point::ORIGIN,
            content: content.into(),
        }
    }

    pub fn offset(mut self, offset: Point) -> Self {
        self.offset = offset;
        self
    }
}

impl<'a, Message, Theme, Renderer> Widget<Message, Theme, Renderer>
for Offset<'a, Message, Theme, Renderer>
where
    Message: 'a + Clone,
    Theme: 'a,
    Renderer: iced::advanced::Renderer,
{
    fn size(&self) -> iced::Size<Length> {
        self.content.as_widget().size()
    }

    fn size_hint(&self) -> Size<Length> {
        self.content.as_widget().size_hint()
    }

    fn layout(
        &self,
        tree: &mut iced::advanced::widget::Tree,
        renderer: &Renderer,
        limits: &iced::advanced::layout::Limits,
    ) -> iced::advanced::layout::Node {
        self.content
            .as_widget()
            .layout(tree, renderer, limits)
            .translate(Vector::new(self.offset.x, self.offset.y))
    }

    fn draw(
        &self,
        tree: &iced::advanced::widget::Tree,
        renderer: &mut Renderer,
        theme: &Theme,
        style: &iced::advanced::renderer::Style,
        layout: iced::advanced::Layout<'_>,
        cursor: iced::advanced::mouse::Cursor,
        viewport: &iced::Rectangle,
    ) {
        self.content
            .as_widget()
            .draw(tree, renderer, theme, style, layout, cursor, viewport);
    }

    fn tag(&self) -> iced::advanced::widget::tree::Tag {
        self.content.as_widget().tag()
    }

    fn state(&self) -> iced::advanced::widget::tree::State {
        self.content.as_widget().state()
    }

    fn children(&self) -> Vec<iced::advanced::widget::Tree> {
        self.content.as_widget().children()
    }

    fn diff(&self, tree: &mut iced::advanced::widget::Tree) {
        self.content.as_widget().diff(tree);
    }

    fn operate(
        &self,
        state: &mut iced::advanced::widget::Tree,
        layout: iced::advanced::Layout<'_>,
        renderer: &Renderer,
        operation: &mut dyn iced::advanced::widget::Operation<()>,
    ) {
        self.content
            .as_widget()
            .operate(state, layout, renderer, operation);
    }

    fn on_event(
        &mut self,
        state: &mut iced::advanced::widget::Tree,
        event: iced::Event,
        layout: iced::advanced::Layout<'_>,
        cursor: iced::advanced::mouse::Cursor,
        renderer: &Renderer,
        clipboard: &mut dyn iced::advanced::Clipboard,
        shell: &mut iced::advanced::Shell<'_, Message>,
        viewport: &iced::Rectangle,
    ) -> iced::advanced::graphics::core::event::Status {
        self.content.as_widget_mut().on_event(
            state, event, layout, cursor, renderer, clipboard, shell, viewport,
        )
    }

    fn mouse_interaction(
        &self,
        state: &iced::advanced::widget::Tree,
        layout: iced::advanced::Layout<'_>,
        cursor: iced::advanced::mouse::Cursor,
        viewport: &Rectangle,
        renderer: &Renderer,
    ) -> iced::advanced::mouse::Interaction {
        self.content
            .as_widget()
            .mouse_interaction(state, layout, cursor, viewport, renderer)
    }

    fn overlay<'b>(
        &'b mut self,
        state: &'b mut iced::advanced::widget::Tree,
        layout: iced::advanced::Layout<'_>,
        renderer: &Renderer,
        translation: iced::Vector,
    ) -> Option<iced::advanced::overlay::Element<'b, Message, Theme, Renderer>> {
        self.content
            .as_widget_mut()
            .overlay(state, layout, renderer, translation)
    }
}

impl<'a, Message, Theme, Renderer> From<Offset<'a, Message, Theme, Renderer>>
for Element<'a, Message, Theme, Renderer>
where
    Message: 'a + Clone,
    Theme: 'a,
    Renderer: 'a + iced::advanced::Renderer,
{
    fn from(offset: Offset<'a, Message, Theme, Renderer>) -> Self {
        Self::new(offset)
    }
}
